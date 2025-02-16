use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;


#[derive(Deserialize, Debug, Clone)]
pub struct DefaultQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,
    pub provider: Option<String>,
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
    pub status: String,
    pub esim: bool,
    pub provider: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub sku: String,
    pub provider: String,
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub total: i64,
    pub results: Vec<T>,
}


#[derive(Serialize, Deserialize)]
pub enum XploriStatusCode {
    Success = 1,
    Error = 2,
    InternalError = 3,
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
    pub status: XploriStatusCode,
    pub data: Value,
}

impl Default for CustomResponse {
    fn default() -> Self {
        Self {
            message: "".to_string(),
            data: Value::Null,
            status: XploriStatusCode::Success,
        }
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
    pub search: Option<String>,
    pub search_fields: Option<Vec<String>>,
    #[serde(flatten)]
    pub fields: HashMap<String, String>,
}
