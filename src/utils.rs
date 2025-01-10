use crate::serializers::*;
use axum::extract::{Multipart, Query};
use axum::Json;
use csv::ReaderBuilder;
use http::StatusCode;
use sqlx::{postgres::PgPoolOptions, Database, FromRow, Pool, Postgres};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::{fs, io, env};
use chrono::{DateTime, Local, Utc};

const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100 MB limit

pub async fn create_pool() -> Pool<Postgres> {
    let database_env = env::var("DATABASE_URL");
    if database_env.is_ok() {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_env.unwrap())
            .await
            .expect("Failed to create pool")
    } else {
        panic!("{:?}", database_env.unwrap());
    }
}

// Generic pagination trait
pub trait Pageable {
    fn get_table_name() -> &'static str;
    fn get_id_field() -> &'static str;
}


pub async fn paginated_query<T>(
    pool: &Pool<Postgres>,
    page_size: i32,
) -> Result<Vec<T>, sqlx::Error>
where
    T: for<'r> FromRow<'r, <Postgres as Database>::Row> + Send + Unpin + Pageable,
    Postgres: Database,
    i32: sqlx::Type<Postgres>,
{
    let query = format!(
        "SELECT * FROM {} WHERE 1=1 ORDER BY {} LIMIT $2",
        T::get_table_name(),
        T::get_id_field(),
    );

    sqlx::query_as::<_, T>(&query)
        .bind(page_size)
        .fetch_all(pool)
        .await
}



pub async fn get_sims_from_db(filters: SimQuery) -> (i64, Vec<Sim>) {
    let pool = create_pool().await;

    let mut query = sqlx::QueryBuilder::new(
        "SELECT id, sim_id, sim_serial, sim_number, provider, active FROM api_sim WHERE 1=1",
    );

    // let mut count_query = sqlx::QueryBuilder::new(
    //     "SELECT COUNT(*) FROM api_sim WHERE 1=1",
    // );

    if let Some(search) = filters.search {
        query.push(" AND sim_serial ILIKE ");
        query.push_bind(format!("%{}%", search));
        // count_query.push(" AND sim_serial ILIKE ");
        // count_query.push_bind(format!("%{}%", search));
    }

    if let Some(provider) = filters.provider {
        query.push(" AND provider = ");
        query.push_bind(provider.clone());
        // count_query.push(" AND provider = ");
        // count_query.push_bind(provider);
    }

    // count_query.push(";");

    if let Some(page_size) = filters.page_size {
        query.push(" LIMIT ");
        query.push_bind(page_size);
    }

    if let Some(page) = filters.page {
        query.push(" OFFSET ");
        query.push_bind(page);
    }

    query.push(";");

    // println!("{}", query.sql());
    // println!("{}", count_query.sql());

    let sims = query
        .build_query_as::<Sim>()
        .fetch_all(&pool)
        .await
        .unwrap_or(Vec::new());
    // let count = count_query.build_query_scalar::<i64>().fetch_one(&pool).await.unwrap_or(0);
    (0, sims)
}

pub async fn get_file_content(Query(query): Query<FileContentQuery>) -> Result<Json<FileContentResponse>, (StatusCode, String)> {
    // Validate and sanitize file path
    let file_directory = ".";
    let file_path = Path::new(file_directory).join(&query.file_name);

    // Prevent directory traversal
    if !file_path.starts_with(file_directory) {
        return Err((StatusCode::FORBIDDEN, "Invalid file path".to_string()));
    }

    // Open the file
    let mut file = match fs::File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            return match e.kind() {
                io::ErrorKind::NotFound => {
                    Err((StatusCode::NOT_FOUND, "File not found".to_string()))
                }
                io::ErrorKind::PermissionDenied => {
                    Err((StatusCode::FORBIDDEN, "Permission denied".to_string()))
                }
                _ => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error opening file: {}", e),
                )),
            };
        }
    };

    // Check file size
    let file_size = file
        .metadata()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not read file metadata".to_string(),
            )
        })?
        .len();

    if file_size > MAX_FILE_SIZE {
        return Err((StatusCode::PAYLOAD_TOO_LARGE, "File too large".to_string()));
    }

    // Count lines and search
    let mut reader = BufReader::new(&file);
    let (total_lines, search_results) =
        count_lines_and_search(&mut reader, query.search.as_deref())
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Determine start and end lines
    let num_lines = query.num_lines.unwrap_or(100);
    let (start_line, _) = if let Some(line) = query.line {
        // User-specified line
        let start = line.saturating_sub(1);
        let end = (start + num_lines).min(total_lines);
        (start, end)
    } else if let Some(results) = &search_results {
        // If search results exist, default to showing search results
        if !results.is_empty() {
            // Get the first few search results
            let start = results[0].0 - 1;
            let end = (start + num_lines).min(total_lines);
            (start, end)
        } else {
            // Default to last 100 lines
            let start = total_lines.saturating_sub(num_lines);
            let end = total_lines;
            (start, end)
        }
    } else {
        // Default to last 100 lines
        let start = total_lines.saturating_sub(num_lines);
        let end = total_lines;
        (start, end)
    };

    // Read specific lines range
    let content = read_lines_range(&mut file, start_line, num_lines)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if search_results.is_none() {
        Ok(Json(FileContentResponse { content }))
    } else {
        let search_result = search_results.unwrap();
        let results = search_result.iter().map(|x| x.1.clone()).collect();
        Ok(Json(FileContentResponse { content: results }))
    }
}


pub async fn extract_data(mut multipart: Multipart) -> Result<(String, String, bool), (StatusCode, String)> {
    fs::create_dir_all("./uploads").map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create upload directory: {}", e),
        )
    })?;

    let mut provider = "roaming".to_string();
    let mut esim = false;
    let mut file_path = "".to_string();

    while let Some(mut field) = multipart
        .next_field().await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        if field.name() == Some("provider") {
            provider = field.text().await.unwrap();
        } else if field.name() == Some("esim") {
            let _esim = field.text().await.unwrap();
            esim = _esim == "true";
        } else {
            let Some(filename) = field.file_name().map(sanitize_filename::sanitize) else {
                continue;
            };
            let mut file = fs::File::create(format!("./uploads/{}", filename)).unwrap();
            file_path = format!("./uploads/{}", filename);
            while let Some(chunk) = field
                .chunk()
                .await
                .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
            {
                println!("received {} bytes", chunk.len());
                file.write_all(&chunk).unwrap();
            }
            file.flush().unwrap();
        }
    }
    Ok((provider, file_path, esim))
}

pub async fn save_csv_data_to_db(file_path: String, esim: bool, provider: String) {
    let pool = create_pool().await;
    let data = fs::read_to_string(file_path.clone()).unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    let records = rdr.deserialize();
    for result in records {
        let record: CsvData = result.unwrap();
        let iccid = record.iccid.trim();
        println!("ROW: {}", record);
        if iccid == "iccid" {
            continue;
        }
        let imsi = record.imsi.unwrap_or_else(|| "IMSI".to_owned() + iccid);
        let msisdn = record.msisdn.unwrap_or_else(|| "MSISDN".to_owned() + iccid);
        let qr_code = record.qr_code.unwrap_or("".to_string());

        let mut query = sqlx::QueryBuilder::new(
            "INSERT INTO api_simidmapper(imsi, iccid, msisdn, qr_code, esim, synced, active, updated, created, assigned, joytel_pin, provider) VALUES (",
        );
        let now = DateTime::<Utc>::from_timestamp(Local::now().timestamp(), 0);

        query.push_bind(imsi.trim()).push(", ")
            .push_bind(iccid.trim()).push(", ")
            .push_bind(msisdn.trim()).push(", ")
            .push_bind(qr_code.trim()).push(", ")
            .push_bind(esim).push(", ")
            .push_bind(false).push(", ")
            .push_bind(true).push(", ")
            .push_bind(now).push(", ")
            .push_bind(now).push(", ")
            .push_bind(false).push(", ")
            .push_bind("").push(", ")
            .push_bind(&provider).push(");");

        let res = query.build().fetch_optional(&pool).await;
        match res {
            Ok(row) => println!("Mapper Added {:?}", row),
            Err(msg) => println!("Mapper Error: {}", msg),
        }

        // add data into Sim table as well
        let mut sim_query = sqlx::QueryBuilder::new(
            "INSERT INTO api_sim(sim_id, sim_serial, sim_number, qr_code, esim, active, subscribed, \
             use_fup_code, sent_email,
             created_time, provider) VALUES (",
        );

        sim_query.push_bind(imsi.trim()).push(", ")
            .push_bind(iccid.trim()).push(", ")
            .push_bind(msisdn.trim()).push(", ")
            .push_bind(qr_code.trim()).push(", ")
            .push_bind(esim).push(", ")
            .push_bind(true).push(", ")
            .push_bind(false).push(", ")
            .push_bind(false).push(", ")
            .push_bind(false).push(", ")
            .push_bind(now).push(", ")
            .push_bind(&provider).push(");");

        let sim_res = sim_query.build().fetch_optional(&pool).await;
        match sim_res {
            Ok(row) => println!("Added Sim {:?}", row),
            Err(msg) => println!("Sim Error: {}", msg),
        }
    }
}


// Efficient line counting and searching
pub fn count_lines_and_search(
    reader: &mut impl BufRead,
    search_term: Option<&str>,
) -> SearchResult {
    let mut total_lines = 0;
    let mut search_results = search_term.map(|_| Vec::new());
    let search_term = search_term.unwrap().to_lowercase().clone();

    for (line_number, line_result) in reader.by_ref().lines().enumerate() {
        let line = line_result?;
        total_lines += 1;

        // Search functionality
        if !&search_term.is_empty() && line.to_lowercase().contains(&search_term) {
            search_results
                .as_mut()
                .unwrap()
                .push((line_number + 1, line.clone()));
        }
    }

    Ok((total_lines, search_results))
}

// Efficient line retrieval
pub fn read_lines_range(
    file: &mut fs::File,
    start_line: usize,
    num_lines: usize,
) -> io::Result<Vec<String>> {
    file.seek(SeekFrom::Start(0))?;
    let mut reader = BufReader::new(file);

    // Skip lines before start_line
    for _ in 0..start_line {
        reader.by_ref().lines().next();
    }

    // Read specified number of lines
    reader.by_ref().lines().take(num_lines).collect()
}

pub fn get_xplori_token() -> String {
    "".to_string()
}
