mod schema;

use axum::routing::post;
use axum::{
    extract::{DefaultBodyLimit, Multipart, Query},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fs,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write},
    path::Path,
};
use tower_http::cors::{Any, CorsLayer};

const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100 MB limit

#[derive(Deserialize)]
struct FileContentQuery {
    file_name: String,
    line: Option<usize>,
    num_lines: Option<usize>,
    search: Option<String>,
}

#[derive(Serialize)]
struct FileContentResponse {
    content: Vec<String>,
}

async fn read_csv(path: &str) -> Vec<HashMap<String, String>> {
    let mut data: Vec<HashMap<String, String>> = Vec::new();
    let mut rdr = Reader::from_path(path).unwrap();
    for result in rdr.records() {
        let mut row: HashMap<String, String> = HashMap::new();
        let record = result.unwrap();
        let sim_id = record[0].trim();
        let sim_serial = record[1].trim();
        let sim_number = record[2].trim();
        let qr_code = record[5].trim();
        row.insert("sim_id".to_string(), sim_id.to_string());
        row.insert("sim_serial".to_string(), sim_serial.to_string());
        row.insert("sim_number".to_string(), sim_number.to_string());
        row.insert("qr_code".to_string(), qr_code.to_string());
        data.push(row);
    }
    data
}

async fn insert(data: &Vec<HashMap<String, String>>, provider: &str) -> Result<(), ()> {
    println!("{:?}", data);
    Ok(())
}

// Handler for file upload
async fn upload(mut multipart: Multipart) -> Result<(StatusCode, String), (StatusCode, String)> {
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
    let data = read_csv(&file_path).await;
    insert(&data, provider.as_str())
        .await
        .expect("TODO: panic message");
    Ok((StatusCode::OK, "Uploaded".to_string()))
}

type SearchResult = io::Result<(usize, Option<Vec<(usize, String)>>)>;

// Efficient line counting and searching
fn count_lines_and_search(reader: &mut impl BufRead, search_term: Option<&str>) -> SearchResult {
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
fn read_lines_range(
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

async fn get_file_content(
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

#[tokio::main]
async fn main() {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/upload", post(upload))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10 MB max request size
        .route("/api/file", get(get_file_content))
        .layer(cors);

    // Run it with hyper on localhost:8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
