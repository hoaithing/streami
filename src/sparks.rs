use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use thiserror::Error;
use log::{info, warn, error};

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

// Configuration struct
#[derive(Debug, Clone)]
pub struct SparkConfig {
    api_token: String,
    base_url: String,
    timeout: Duration,
    max_retries: u32,
}

impl Default for SparkConfig {
    fn default() -> Self {
        Self {
            api_token: env::var("SPARKS_API_TOKEN")
                .expect("SPARKS_API_TOKEN environment variable not set"),
            base_url: "https://ocs-api.telco-vision.com:7443/ocs-custo/main/v1".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
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

pub struct SparkClient {
    config: SparkConfig,
    client: Client,
}

impl SparkClient {
    const GET_SIM_INFO: &'static str = "getSingleSubscriber";
    const CHANGE_CREDITS: &'static str = "modifySubscriberBalance";
    const ASSIGN_REAL_PHONE_NUMBER: &'static str = "affectSubscriberRealPhoneNumber";
    const LIST_PACKAGES: &'static str = "listPrepaidPackageTemplate";
    const GET_SIM_PACKAGES: &'static str = "listSubscriberPrepaidPackages";
    const ASSIGN_PACKAGE: &'static str = "affectPackageToSubscriber";
    const ASSIGN_DAILY_PACKAGE: &'static str = "affectRecurringPackageToSubscriber";
    const REMOVE_ALL_PACKAGES: &'static str = "cleanSubscriberAllPackages";

    pub fn new(config: Option<SparkConfig>) -> Result<Self, SparkError> {
        let config = config.unwrap_or_default();
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(SparkError::NetworkError)?;

        Ok(Self { config, client })
    }

    async fn fetch<T: for<'de> Deserialize<'de>>(
        &self,
        data: Value,
        uri: Option<&str>,
    ) -> Result<ApiResponse<T>, SparkError> {
        let mut retries = 0;
        let url = format!("{}?token={}", self.config.base_url, self.config.api_token);

        loop {
            info!("[Spark] Fetching: {} Data: {}", uri.unwrap_or(""), data);

            match self.client
                .post(&url)
                .json(&data)
                .header("Content-Type", "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    let status = response.status();
                    if !status.is_success() {
                        return Err(SparkError::ResponseError(format!(
                            "HTTP error: {}",
                            status
                        )));
                    }

                    let response_json: Value = response.json().await
                        .map_err(SparkError::NetworkError)?;

                    if let Some(status) = response_json.get("status") {
                        return if status["code"].as_str() == Some("0") {
                            if let Some(uri_str) = uri {
                                if let Some(data) = response_json.get(uri_str) {
                                    return Ok(ApiResponse {
                                        success: true,
                                        data: Some(serde_json::from_value(data.clone())
                                            .map_err(|e| SparkError::ResponseError(e.to_string()))?),
                                        error: None,
                                    });
                                }
                            }
                            Ok(ApiResponse {
                                success: true,
                                data: None,
                                error: None,
                            })
                        } else {
                            Err(SparkError::ApiError {
                                code: status["code"].to_string(),
                                message: status["msg"].to_string(),
                            })
                        }
                    }
                    return Err(SparkError::ResponseError("Invalid response format".to_string()));
                }
                Err(e) if retries < self.config.max_retries => {
                    warn!("Request failed, retrying ({}/{}): {}", retries + 1, self.config.max_retries, e);
                    retries += 1;
                    tokio::time::sleep(Duration::from_secs(2u64.pow(retries))).await;
                    continue;
                }
                Err(e) => return Err(SparkError::NetworkError(e)),
            }
        }
    }

    pub async fn add_sim_credit(
        &self,
        sub_id: &str,
        amount: f64,
        description: &str,
    ) -> Result<ApiResponse<()>, SparkError> {
        let data = json!({
            Self::CHANGE_CREDITS: {
                "subscriber": {
                    "subscriberId": sub_id
                },
                "amount": amount,
                "description": description
            }
        });
        self.fetch(data, None).await
    }

    pub async fn get_sim_info(&self, imsi: &str) -> Result<ApiResponse<SimInfo>, SparkError> {
        let data = json!({
            Self::GET_SIM_INFO: {
                "imsi": imsi,
                "onlySubsInfo": true
            }
        });
        self.fetch(data, Some(Self::GET_SIM_INFO)).await
    }

    pub async fn assign_package(
        &self,
        imsi: &str,
        package_id: &str,
        days: i32,
        active_date: Option<DateTime<Utc>>,
    ) -> Result<ApiResponse<Package>, SparkError> {
        let data = match active_date {
            Some(date) => {
                let end_date = date.checked_add_signed(chrono::Duration::days(days as i64))
                    .ok_or_else(|| SparkError::ResponseError("Invalid date calculation".to_string()))?;

                json!({
                    Self::ASSIGN_PACKAGE: {
                        "packageTemplateId": package_id,
                        "subscriber": {
                            "imsi": imsi
                        },
                        "activePeriod": {
                            "start": date.to_rfc3339(),
                            "end": end_date.to_rfc3339()
                        }
                    }
                })
            },
            None => json!({
                Self::ASSIGN_PACKAGE: {
                    "packageTemplateId": package_id,
                    "subscriber": {
                        "imsi": imsi
                    }
                }
            }),
        };
        self.fetch(data, Some(Self::ASSIGN_PACKAGE)).await
    }

    pub async fn assign_daily_package(
        &self,
        imsi: &str,
        package_id: &str,
        active_date: Option<DateTime<Utc>>,
    ) -> Result<ApiResponse<Package>, SparkError> {
        let data = match active_date {
            Some(date) => json!({
                Self::ASSIGN_DAILY_PACKAGE: {
                    "packageTemplateId": package_id,
                    "subscriber": {
                        "imsi": imsi
                    },
                    "startTimeUTC": date.to_rfc3339(),
                    "activationAtFirstUse": false
                }
            }),
            None => json!({
                Self::ASSIGN_DAILY_PACKAGE: {
                    "packageTemplateId": package_id,
                    "subscriber": {
                        "imsi": imsi
                    },
                    "activationAtFirstUse": true
                }
            }),
        };
        self.fetch(data, Some(Self::ASSIGN_DAILY_PACKAGE)).await
    }

    pub async fn change_sim_number(
        &self,
        imsi: &str,
        sim_number: &str,
    ) -> Result<ApiResponse<Package>, SparkError> {
        let data = json!({
            Self::ASSIGN_REAL_PHONE_NUMBER: {
                "subscriber": {
                    "imsi": imsi
                },
                "phoneNumber": sim_number
            }
        });
        self.fetch(data, None).await
    }

    pub async fn expire_all_package(&self, imsi: &str) -> Result<ApiResponse<Package>, SparkError> {
        let data = json!({
            Self::REMOVE_ALL_PACKAGES: {
                "imsi": imsi
            }
        });
        self.fetch(data, Some(Self::REMOVE_ALL_PACKAGES)).await
    }

    pub async fn get_list_packages(&self) -> Result<ApiResponse<Package>, SparkError> {
        let data = json!({
            Self::LIST_PACKAGES: {}
        });
        self.fetch(data, Some(Self::LIST_PACKAGES)).await
    }

    pub async fn get_sim_packages(&self, imsi: &str) -> Result<ApiResponse<Package>, SparkError> {
        let data = json!({
            Self::GET_SIM_PACKAGES: {
                "imsi": imsi
            }
        });
        self.fetch(data, Some(Self::GET_SIM_PACKAGES)).await
    }

    pub async fn get_esim_history(&self, sim_id: &str) -> Result<ApiResponse<Package>, SparkError> {
        let data = json!({
            "getSimProviderStatus": sim_id
        });
        self.fetch(data, None).await
    }

    // Implementing a builder pattern for configuration
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.config.max_retries = max_retries;
        self
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.config.base_url = base_url;
        self
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_spark_client() -> Result<(), SparkError> {
        let client = SparkClient::new(None)?
            .with_timeout(Duration::from_secs(60))
            .with_max_retries(5);

        let sim_info = client.get_sim_info("123456789").await?;
        assert!(sim_info.success);

        Ok(())
    }
}

// Example of how to use the improved client:
/*
#[tokio::main]
async fn main() -> Result<(), SparkError> {
    // Create a custom configuration
    let config = SparkConfig {
        api_token: "your_token".to_string(),
        base_url: "https://custom-url.com".to_string(),
        timeout: Duration::from_secs(60),
        max_retries: 5,
    };

    // Initialize client with custom config
    let client = SparkClient::new(Some(config))?;

    // Or use default configuration with builder pattern
    let client = SparkClient::new(None)?
        .with_timeout(Duration::from_secs(60))
        .with_max_retries(5);

    // Get SIM information
    let sim_info = client.get_sim_info("123456789").await?;
    println!("{:?}", sim_info);

    Ok(())
}
*/