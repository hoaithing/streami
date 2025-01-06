use std::io;
use serde::{Deserialize, Serialize};
use crate::models::Sim;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub page_size: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 50,
        }
    }
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


#[derive(Serialize)]
pub struct SimResponse {
    pub count: i64,
    pub next: &'static str,
    pub prev: Option<&'static str>,
    pub results: Vec<Sim>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimMapper {
    pub id: i64,
    pub iccid: String,
    pub sim_number: String,
    pub sim_serial: String,
    pub qr_code: String,
    pub added: bool,
    pub provider: String,
    pub synced: bool,
    pub synced_at: Option<String>,
    pub product_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub type SearchResult = io::Result<(usize, Option<Vec<(usize, String)>>)>;
