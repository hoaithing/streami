use std::fmt::Display;
use serde::{Deserialize, Serialize};
use std::io;
use sqlx::FromRow;

#[derive(Deserialize, Debug, Clone)]
pub struct DefaultQuery {
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

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub sku: String,
    pub provider: String
}


#[derive(Serialize)]
pub struct PaginatedSimResponse {
    pub total: i64,
    pub results: Vec<Sim>,
}

pub type SearchResult = io::Result<(usize, Option<Vec<(usize, String)>>)>;

#[derive(Debug, Deserialize)]
pub struct CsvData {
    pub imsi: Option<String>,
    pub iccid: String,
    pub msisdn: Option<String>,
    pub qr_code: Option<String>,
}

impl Display for CsvData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, {}",
               self.imsi.clone().unwrap_or("".to_string()),
               self.iccid,
               self.msisdn.clone().unwrap_or("".to_string()),
               self.qr_code.clone().unwrap_or("".to_string())
        )
    }
}

#[derive(Serialize, Deserialize)]
pub enum XploriStatusCode {
    Success = 1,
    Error = 2,
    InternalError = 3 ,
    NotFound = 4,
    BadRequest = 5,
    Unauthorized = 6,
    Forbidden,
    PayloadTooLarge,
    UnsupportedMediaType,
}

#[derive(Serialize, Deserialize)]
pub struct CustomResponse {
    pub message: String,
    pub data: Option<String>,
    pub status: XploriStatusCode,
}

impl Default for CustomResponse {
    fn default() -> Self {
        Self {
            message: "".to_string(),
            data: None,
            status: XploriStatusCode::Success
        }
    }
}
