use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct SalePartnerList {
    pub id: i32,
    pub booking_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub source: String,
    pub product: Option<String>,
    pub package_id: Option<i32>,
    pub created: DateTime<Utc>,
    pub booking_date: DateTime<Utc>,
    pub sim_number: Option<String>,
    pub uid: Uuid,
}


#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct SalePartnerDetail {
    pub id: i32,
    pub booking_id: String,
    pub email: String,
    pub name: String,
    pub source: String,
    pub product: String,
    pub package: i32,
    pub created: String,
    pub booking_date: String,
    pub sim_number: String
}
