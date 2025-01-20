use std::fmt::{Display, Formatter};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Request types matching SparkClient parameters
#[derive(Debug, Deserialize)]
pub struct AddSimCreditRequest {
    pub sub_id: String,
    pub amount: f64,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetBitrateRequest {
    pub imsi: String,
    pub bitrate: BitrateMap,
}

#[derive(Debug, Deserialize)]
pub struct GetBitrateRequest {
    pub imsi: String,
}

#[derive(Debug, Deserialize)]
pub struct GetSimInfoRequest {
    pub imsi: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangeSimStatusRequest {
    pub sub_id: String,
    pub status: StatusMap,
}

#[derive(Debug, Deserialize)]
pub struct ChangeSimNumberRequest {
    pub imsi: String,
    pub sim_number: String,
}

#[derive(Debug, Deserialize)]
pub struct AssignPackageRequest {
    pub imsi: String,
    pub package_id: String,
    pub days: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct AssignDailyPackageRequest {
    pub imsi: String,
    pub package_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ExpireAllPackageRequest {
    pub imsi: String,
}

#[derive(Debug, Deserialize)]
pub struct GetEsimHistoryRequest {
    pub sim_id: String,
}

// Custom error type
#[derive(Error, Debug)]
pub enum SparkError {
    #[error("Environment variable not set: {0}")]
    EnvVarError(String),
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Invalid response: {0}")]
    ResponseError(String),
    #[error("API error: {code} - {message}")]
    ApiError { code: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BitrateMap {
    KB128,
    KB256,
    KB384,
    KB512,
    KB1024,
    KB3072,
    KB5120,
    KB7680,
    KB10240,
    KB20480,
    Unlimited,
}

impl BitrateMap {
    fn as_str(&self) -> &'static str {
        match self {
            Self::KB128 => "KB_128",
            Self::KB256 => "KB_256",
            Self::KB384 => "KB_384",
            Self::KB512 => "KB_512",
            Self::KB1024 => "KB_1024",
            Self::KB3072 => "KB_3072",
            Self::KB5120 => "KB_5120",
            Self::KB7680 => "KB_7680",
            Self::KB10240 => "KB_10240",
            Self::KB20480 => "KB_20480",
            Self::Unlimited => "UNLIMITED",
        }
    }
}


impl Display for BitrateMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusMap {
    Active,
    Inactive,
    Disconnected,
    Suspended,
    EndOfLife,
}

impl StatusMap {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Inactive => "INACTIVE",
            Self::Disconnected => "DISCONNECTED",
            Self::Suspended => "SUSPENDED",
            Self::EndOfLife => "END_OF_LIFE",
        }
    }
}

impl Display for StatusMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.as_str())
    }
}

// Response types
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimInfo {
    pub subscriber_id: String,
    pub imsi: String,
    pub status: StatusMap,
    pub balance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
}

