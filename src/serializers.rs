use serde::{Deserialize, Serialize};
use std::io;
use sqlx::FromRow;

#[derive(Deserialize, Debug, Clone)]
pub struct SimQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,
    pub provider: Option<String>,
}

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


#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct SimMapper {
    pub id: i32,
    pub iccid: String,
    pub imsi: String,
    pub msisdn: String,
    pub qr_code: String,
    pub added: bool,
    pub provider: String,
    pub synced: bool,
    pub synced_at: Option<String>,
    pub product_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Sim {
    pub id: i32,
    pub sim_id: String,
    pub sim_number: String,
    pub sim_serial: String,
    pub active: bool,
    pub provider: String
}


#[derive(Serialize)]
pub struct PaginatedSimResponse {
    pub count: i64,
    pub next: &'static str,
    pub prev: Option<&'static str>,
    pub results: Vec<Sim>,
}


pub type SearchResult = io::Result<(usize, Option<Vec<(usize, String)>>)>;
