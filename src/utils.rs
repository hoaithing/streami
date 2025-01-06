use crate::schema::api_sim::dsl::api_sim;
use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use std::{env, fs, io};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use axum::extract::{Multipart, Query};
use axum::Json;
use chrono::prelude::*;
use csv::Reader;
use http::StatusCode;
use crate::models::{SimMapper, Sim, SimMapperInsert};
use crate::schema::api_simidmapper;
use crate::serializers::*;


const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100 MB limit

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_sims(page: i64, page_size: i64) -> Result<Vec<Sim>, diesel::result::Error> {
    let conn = &mut establish_connection();
    let offset = (page - 1) * page_size;
    let res: Vec<Sim> = api_sim
        .offset(offset)
        .limit(page_size)
        .select(Sim::as_select())
        .load(conn)
        .expect("Error loading posts");
    Ok(res)
}

pub fn add_sim_mapper(
    imsi: String,
    iccid: &String,
    msisdn: String,
    qr_code: String,
    esim: bool,
    provider: &String,
) -> QueryResult<SimMapper> {
    let conn = &mut establish_connection();
    let new_sim = SimMapperInsert {
        imsi,
        iccid: iccid.clone(),
        esim,
        provider: provider.clone(),
        qr_code: Some(qr_code),
        synced: false,
        last_email: None,
        msisdn: Some(msisdn),
        active: false,
        booking_id: None,
        created: Some(Utc::now()),
        updated: Default::default(),
        assigned: false,
        joytel_pin: "".to_string(),
    };

    diesel::insert_into(api_simidmapper::table)
        .values(&new_sim)
        .returning(SimMapper::as_returning())
        .get_result(conn)

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


pub fn read_then_import(path: &str, provider: String) {
    let mut rdr = Reader::from_path(path).unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        let sim_id = record[0].trim();
        let sim_serial = record[1].trim();
        let sim_number = record[2].trim();
        let qr_code = record[5].trim();
        let added = add_sim_mapper(
            sim_id.to_string(),
            &sim_serial.to_string(),
            sim_number.to_string(),
            qr_code.to_string(),
            true,
            &provider,
        );
        match added {
            Ok(sim_mapper) => println!("Added {}", sim_mapper.iccid),
            Err(msg) => println!("Error: {}", msg),
        }
    }
}

// Handler for file upload
pub async fn upload(mut multipart: Multipart) -> Result<(StatusCode, String), (StatusCode, String)> {
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
            provider = field.text().await.unwrap().to_string();
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
    read_then_import(&file_path, provider);
    Ok((StatusCode::OK, "Uploaded".to_string()))
}


// Efficient line counting and searching
pub fn count_lines_and_search(reader: &mut impl BufRead, search_term: Option<&str>) -> SearchResult {
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