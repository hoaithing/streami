use crate::serializers::*;
use axum::extract::{Multipart, Query};
use axum::Json;
use csv::ReaderBuilder;
use http::StatusCode;
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres, QueryBuilder, Row};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::{fs, io, env};
use std::collections::HashMap;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct DynamicFilters {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub esim: Option<bool>,
    pub active: Option<bool>,
    #[serde(flatten)]
    pub fields: HashMap<String, String>,
}

impl DynamicFilters {
    pub fn build_where_clause<'a>(&'a self, query_builder: &'a mut QueryBuilder<'a, Postgres>) -> &'a mut QueryBuilder<'a, Postgres> {
        query_builder.push(" WHERE 1=1");

        // Process all dynamic fields except pagination and sorting
        for (key, value) in self.fields.iter() {
            // Skip pagination and sorting parameters
            if value.is_empty() {
                continue;
            }
            if !["page", "page_size", "sort_by", "sort_order"].contains(&key.as_str()) {
                // Handle different operators in the field name
                if key.contains("__") {
                    let parts: Vec<&str> = key.split("__").collect();
                    let field_name = parts[0];
                    let operator = parts[1];

                    match operator {
                        "like" => {
                            query_builder.push(format!(" AND {} ILIKE ", field_name));
                            query_builder.push_bind(format!("%{}%", value));
                        }
                        "gt" => {
                            query_builder.push(format!(" AND {} > ", field_name));
                            query_builder.push_bind(value);
                        }
                        "lt" => {
                            query_builder.push(format!(" AND {} < ", field_name));
                            query_builder.push_bind(value);
                        }
                        "gte" => {
                            query_builder.push(format!(" AND {} >= ", field_name));
                            query_builder.push_bind(value);
                        }
                        "lte" => {
                            query_builder.push(format!(" AND {} <= ", field_name));
                            query_builder.push_bind(value);
                        }
                        "in" => {
                            let values: Vec<&str> = value.split(',').collect();
                            if !values.is_empty() {
                                query_builder.push(format!(" AND {} IN (", field_name));
                                let mut separated = query_builder.separated(", ");
                                for value in values {
                                    separated.push_bind(value.trim().to_string());
                                }
                                separated.push_unseparated(")");
                            }
                        }
                        _ => {
                            query_builder.push(format!(" AND {} = ", field_name));
                            query_builder.push_bind(value);
                        }
                    }
                } else {
                    // Default to exact match if no operator is specified
                    query_builder.push(format!(" AND {} = ", key));
                    query_builder.push_bind(value);
                }
            }
        }
        if let Some(esim) = self.esim {
            query_builder.push(" AND esim = ");
            query_builder.push_bind(esim);
        }
        if let Some(active) = self.active {
            query_builder.push(" AND active = ");
            query_builder.push_bind(active);
        }
        query_builder
    }


    pub fn get_sort_clause(&self) -> String {
        let sort_by = self.sort_by.as_deref().unwrap_or("created");
        let sort_order = self.sort_order.as_deref().unwrap_or("DESC");

        format!(" ORDER BY {} {}", sort_by, sort_order)
    }

    pub fn get_pagination(&self) -> (i64, i64) {
        let page_size = self.page_size.unwrap_or(50);
        let page = self.page.unwrap_or(1);
        let offset = (page - 1) * page_size;

        (page_size, offset)
    }
}

pub async fn get_data_from_db<T>(
    pool: Pool<Postgres>,
    filters: DynamicFilters,
    table: &str,
    select_columns: Option<&str>,
) -> Result<(i64, Vec<T>), sqlx::Error>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin
{
    let columns = select_columns.unwrap_or("*");
    let (limit, offset) = filters.get_pagination();
    let sort_clause = filters.get_sort_clause();

    // Start building the query
    let mut query_builder = QueryBuilder::<Postgres>::new(
        format!(
            r#"
            SELECT {columns},
                   COUNT(*) OVER() as full_count
            FROM {table}
            "#,
            columns = columns,
            table = table,
        )
    );

    // Add WHERE clause and bindings
    let query_builder = filters.build_where_clause(&mut query_builder);

    // Add sorting
    query_builder.push(&sort_clause);

    // Add LIMIT and OFFSET
    query_builder.push(" LIMIT ");
    query_builder.push_bind(limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);
    query_builder.push(";");

    // Debug: Print the SQL query
    // println!("Query SQL: {}", query_builder.sql());

    // Execute the query
    let rows = query_builder.build().fetch_all(&pool).await?;

    let mut results = Vec::new();
    let mut total_count = 0;

    // Process results
    if let Some(first_row) = rows.first() {
        total_count = first_row.try_get("full_count").unwrap_or(0);
    }

    for row in rows {
        if let Ok(item) = T::from_row(&row) {
            results.push(item);
        }
    }

    Ok((total_count, results))
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

        let mut query = QueryBuilder::new(
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
        let mut sim_query = QueryBuilder::new(
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
