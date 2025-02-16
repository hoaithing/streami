use std::env;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::Duration;
use tracing::{info, warn};
use crate::sparks::serializers::{ApiResponse, Package, SimInfo, SparkError};

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
        uri: &str,
    ) -> Result<ApiResponse<T>, SparkError> {
        let mut retries = 0;
        let url = format!("{}?token={}", self.config.base_url, self.config.api_token);

        loop {
            info!("[Spark] Fetching: {} Data: {}", uri, data);
            let resp = self.client
                .post(&url)
                .json(&data)
                .header("Content-Type", "application/json")
                .send()
                .await;
            match resp {
                Ok(response) => {
                    let status = response.status();
                    if !status.is_success() {
                        return Err(SparkError::ResponseError(format!("HTTP error: {}", status)));
                    }

                    let response_json: Value = response.json().await.map_err(SparkError::NetworkError)?;

                    if let Some(status) = response_json.get("status") {
                        return if status["code"].as_str() == Some("0") {
                            if !uri.is_empty() {
                                if let Some(data) = response_json.get(uri) {
                                    return Ok(ApiResponse {
                                        success: true,
                                        data: Some(serde_json::from_value(data.clone()).map_err(|e| SparkError::ResponseError(e.to_string()))?),
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
                    // sleep 1 sec before retry
                    tokio::time::sleep(Duration::from_secs(1)).await;
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
        self.fetch(data, "").await
    }
    
    pub async fn get_sim_info(&self, imsi: &str) -> Result<ApiResponse<SimInfo>, SparkError> {
        let data = json!({
            Self::GET_SIM_INFO: {
                "imsi": imsi,
                "onlySubsInfo": true
            }
        });
        self.fetch(data, Self::GET_SIM_INFO).await
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
        self.fetch(data, Self::ASSIGN_PACKAGE).await
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
        self.fetch(data, Self::ASSIGN_DAILY_PACKAGE).await
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
        self.fetch(data, "").await
    }

    pub async fn expire_all_package(&self, imsi: &str) -> Result<ApiResponse<Package>, SparkError> {
        let data = json!({
            Self::REMOVE_ALL_PACKAGES: {
                "imsi": imsi
            }
        });
        self.fetch(data, Self::REMOVE_ALL_PACKAGES).await
    }

    pub async fn get_list_packages(&self) -> Result<ApiResponse<Vec<Package>>, SparkError> {
        let data = json!({
            Self::LIST_PACKAGES: {}
        });
        self.fetch(data, Self::LIST_PACKAGES).await
    }

    pub async fn get_sim_packages(&self, imsi: &str) -> Result<ApiResponse<Package>, SparkError> {
        let data = json!({
            Self::GET_SIM_PACKAGES: {
                "imsi": imsi
            }
        });
        self.fetch(data, Self::GET_SIM_PACKAGES).await
    }

    pub async fn get_esim_history(&self, sim_id: &str) -> Result<ApiResponse<Package>, SparkError> {
        let data = json!({
            "getSimProviderStatus": sim_id
        });
        self.fetch(data, "").await
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
