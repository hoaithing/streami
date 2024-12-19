use axum::{
    extract::{DefaultBodyLimit, Multipart, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get, Json,
    Router,
};

use axum::routing::post;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{
    fs,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf}
};
use tower_http::cors::{Any, CorsLayer};

const FILE_DIRECTORY: &str = "/home/cherry";
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



// Handler for file upload
async fn upload_file(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Ensure upload directory exists
    fs::create_dir_all("./uploads")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create upload directory: {}", e)))?;

    let mut uploaded_files = Vec::new();

    // Process each field in the multipart form
    while let Some(field) = multipart.next_field().await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Multipart error: {}", e)))?
    {
        // Get filename, using sanitization
        let Some(filename) = field.file_name().map(sanitize_filename::sanitize) else {
            continue; // Skip fields without filenames
        };

        // Construct full file path
        let file_path = PathBuf::from("./uploads").join(&filename);

        // Open file for writing
        let mut file = fs::File::create(&file_path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create file: {}", e)))?;

        // Stream and save file content with size tracking
        let mut stream = field.into_stream();
        let mut total_size = 0;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error reading file chunk: {}", e)))?;

            // Check file size
            total_size += chunk.len();
            if total_size > 10 * 1024 * 1024 { // 10 MB limit
                return Err((
                    StatusCode::PAYLOAD_TOO_LARGE,
                    format!("File {} exceeds maximum size of 10MB", filename)
                ));
            }

            // Write chunk to file
            file.write_all(&chunk).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to write file chunk: {}", e)))?;
        }

        // Close the file
        file.flush().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to flush file: {}", e)))?;

        uploaded_files.push(filename);
    }

    // Respond with uploaded file names
    Ok((StatusCode::OK, format!("Uploaded files: {:?}", uploaded_files)))
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
            search_results.as_mut().unwrap().push((line_number + 1, line.clone()));
        }
    }

    Ok((total_lines, search_results))
}

// Efficient line retrieval
fn read_lines_range(
    file: &mut fs::File,
    start_line: usize,
    num_lines: usize
) -> io::Result<Vec<String>> {
    file.seek(SeekFrom::Start(0))?;
    let mut reader = BufReader::new(file);

    // Skip lines before start_line
    for _ in 0..start_line {
        reader.by_ref().lines().next();
    }

    // Read specified number of lines
    reader.by_ref()
          .lines()
          .take(num_lines)
          .collect()
}

async fn get_file_content(Query(query): Query<FileContentQuery>) -> Result<Json<FileContentResponse>, (StatusCode, String)> {
    // Validate and sanitize file path
    let file_path = Path::new(FILE_DIRECTORY).join(&query.file_name);

    // Prevent directory traversal
    if !file_path.starts_with(FILE_DIRECTORY) {
        return Err((StatusCode::FORBIDDEN, "Invalid file path".to_string()));
    }

    // Open the file
    let mut file = match fs::File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            return match e.kind() {
                io::ErrorKind::NotFound =>
                    Err((StatusCode::NOT_FOUND, "File not found".to_string())),
                io::ErrorKind::PermissionDenied =>
                    Err((StatusCode::FORBIDDEN, "Permission denied".to_string())),
                _ =>
                    Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error opening file: {}", e)))
            };
        }
    };

    // Check file size
    let file_size = file.metadata()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not read file metadata".to_string()))?
        .len();

    if file_size > MAX_FILE_SIZE {
        return Err((StatusCode::PAYLOAD_TOO_LARGE, "File too large".to_string()));
    }

    // Count lines and search
    let mut reader = BufReader::new(&file);
    let (total_lines, search_results) = count_lines_and_search(&mut reader, query.search.as_deref())
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
    let content = read_lines_range(
        &mut file,
        start_line,
        num_lines
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
    // Ensure file directory exists
    fs::create_dir_all(FILE_DIRECTORY).expect("Failed to create files directory");

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/upload", post(upload_file))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10 MB max request size
        .route("/api/file", get(get_file_content))
        .layer(cors);

    // Run it with hyper on localhost:8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}