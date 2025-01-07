use crate::serializers::*;
use axum::extract::{Multipart, Query};
use axum::Json;
use csv::Reader;
use http::StatusCode;
use serde::{Deserialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::{fs, io};

const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100 MB limit


pub async fn create_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://xplori:GZkwR$4Kfu4xff8@127.0.0.1/xplori")
        .await
        .expect("Failed to create pool")
}

pub async fn get_sims(
    Query(filters): Query<SimQuery>,
    pool: axum::extract::State<Pool<Postgres>>,
) -> Json<Vec<Sim>> {

    let mut query = sqlx::QueryBuilder::new(
        "WITH filtered_sims AS (
            SELECT id, sim_id, sim_serial, sim_number, provider, active FROM api_sim WHERE 1=1"
    );

    if let Some(search) = filters.search {
        query.push(" AND sim_serial ILIKE ");
        query.push_bind(format!("%{}%", search));
    }

    if let Some(provider) = filters.provider {
        query.push(" AND provider = ");
        query.push_bind(provider);
    }

    if let Some(page_size) = filters.page_size {
        query.push(" LIMIT ");
        query.push_bind(page_size);
    }

    if let Some(page) = filters.page {
        query.push(" OFFSET ");
        query.push_bind(page);
    }

    query.push(") SELECT *, COUNT(*) OVER() as total FROM filtered_sims");


    let users = query
        .build_query_as::<Sim>()
        .fetch_all(&*pool)
        .await
        .unwrap();

    let total_count = users.first().map(|u| u.total).unwrap_or(0);

    Json(PaginatedSimResponse {
        count: 0,
        next: "",
        prev: None,
        total: total_count,
        results: vec![],
    })

    Json(users)
}

pub async fn get_file_content(
    Query(query): Query<FileContentQuery>,
) -> Result<Json<FileContentResponse>, (StatusCode, String)> {
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

pub fn read_then_import(path: &str, provider: &str) {
    // let mut rdr = Reader::from_path(path).unwrap();
    // let mut records = rdr.records();
    // let header = records.next().unwrap().unwrap();
    // for (index, h) in header.iter().enumerate() {
    //     println!("{} {}", h, index);
    // }
    // for result in rdr.records() {
    //     let record = result.unwrap();
    //     let sim_id = record[0].trim();
    //     let sim_serial = record[1].trim();
    //     let sim_number = record[2].trim();
    //     let qr_code = record[5].trim();
        // let added = add_sim_mapper(sim_id, sim_serial, sim_number, qr_code, true, provider);
        // match added {
        //     Ok(sim_mapper) => println!("Added {}", sim_mapper.iccid),
        //     Err(msg) => println!("Error: {}", msg),
        // }
    // }
}

// Handler for file upload
pub async fn upload(
    mut multipart: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    fs::create_dir_all("./uploads").map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create upload directory: {}", e),
        )
    })?;

    let mut provider = "roaming".to_string();
    let mut file_path = "".to_string();

    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        if field.name() == Some("provider") {
            provider = field.text().await.unwrap();
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
    println!("file_path: {}", file_path);
    read_then_import(&file_path, provider.as_str());
    Ok((StatusCode::OK, "Uploaded".to_string()))
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
