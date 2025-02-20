use axum::extract::Multipart;
use chrono::{DateTime, Local, Utc};
use csv::ReaderBuilder;
use http::StatusCode;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, QueryBuilder};
use std::io::Write;
use std::{env, fs, io};
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use calamine::{open_workbook, Xlsx, Reader};

pub type SearchResult = io::Result<(usize, Option<Vec<(usize, String)>>)>;


#[derive(Deserialize)]
pub struct FileContentQuery {
    pub file_name: String,
    pub line: Option<usize>,
    pub num_lines: Option<usize>,
    pub search: Option<String>,
}


#[derive(Serialize)]
pub struct FileContentResponse {
    pub content: Vec<String>,
}


pub async fn create_pool() -> Pool<Postgres> {
    let database_env = env::var("DATABASE_URL");
    if database_env.is_ok() {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_env.unwrap())
            .await
            .expect("Failed to create pool")
    } else {
        panic!("DATABASE_URL not present");
    }
}


pub async fn extract_data(
    mut multipart: Multipart,
) -> Result<(String, String, bool), (StatusCode, String)> {
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
        .next_field()
        .await
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
                tracing::info!("received {} bytes", chunk.len());
                file.write_all(&chunk).unwrap();
            }
            file.flush().unwrap();
        }
    }
    Ok((provider, file_path, esim))
}


#[derive(Debug, Deserialize)]
pub struct CsvData {
    pub imsi: Option<String>,
    pub iccid: String,
    pub msisdn: Option<String>,
    pub qr_code: Option<String>,
}


impl Display for CsvData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.imsi.clone().unwrap_or("".to_string()),
            self.iccid,
            self.msisdn.clone().unwrap_or("".to_string()),
            self.qr_code.clone().unwrap_or("".to_string())
        )
    }
}


pub async fn save_csv_data_to_db(file_path: String, esim: bool, provider: String) -> (i32, i32) {
    let pool = create_pool().await;
    let data = fs::read_to_string(file_path.clone()).unwrap();
    let mut success = 0;
    let mut errors = 0;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    let records = rdr.deserialize();
    for result in records {
        let record: CsvData = result.unwrap();
        let iccid = record.iccid.trim();
        tracing::info!("ROW: {}", record);
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

        query
            .push_bind(imsi.trim())
            .push(", ")
            .push_bind(iccid.trim())
            .push(", ")
            .push_bind(msisdn.trim())
            .push(", ")
            .push_bind(qr_code.trim())
            .push(", ")
            .push_bind(esim)
            .push(", ")
            .push_bind(false)
            .push(", ")
            .push_bind(true)
            .push(", ")
            .push_bind(now)
            .push(", ")
            .push_bind(now)
            .push(", ")
            .push_bind(false)
            .push(", ")
            .push_bind("")
            .push(", ")
            .push_bind(&provider)
            .push(");");

        let res = query.build().fetch_optional(&pool).await;
        match res {
            Ok(row) => tracing::info!("Mapper Added {:?}", row),
            Err(msg) => tracing::warn!("Mapper Error: {}", msg),
        }

        // add data into Sim table as well
        let mut sim_query = QueryBuilder::new(
            "INSERT INTO api_sim(sim_id, sim_serial, sim_number, qr_code, esim, active, subscribed, \
             use_fup_code, sent_email,
             created, provider) VALUES (",
        );

        sim_query
            .push_bind(imsi.trim())
            .push(", ")
            .push_bind(iccid.trim())
            .push(", ")
            .push_bind(msisdn.trim())
            .push(", ")
            .push_bind(qr_code.trim())
            .push(", ")
            .push_bind(esim)
            .push(", ")
            .push_bind(true)
            .push(", ")
            .push_bind(false)
            .push(", ")
            .push_bind(false)
            .push(", ")
            .push_bind(false)
            .push(", ")
            .push_bind(now)
            .push(", ")
            .push_bind(&provider)
            .push(");");

        let sim_res = sim_query.build().fetch_optional(&pool).await;
        match sim_res {
            Ok(row) => { 
                tracing::info!("Added Sim {:?}", row);
                success += 1;
            },
            Err(msg) => {
                tracing::warn!("Sim Error: {}", msg);
                errors += 1;
            }
        }
    }
    (success, errors)
}


#[allow(dead_code)]
fn read_excel(file_path: String) {
    let mut excel: Xlsx<_> = open_workbook(file_path).unwrap();
    if let Ok(r) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }
}