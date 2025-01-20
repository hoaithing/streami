use crate::sparks::serializers::{AddSimCreditRequest, ApiResponse, AssignPackageRequest, GetSimInfoRequest, Package, SimInfo};

use crate::sparks::sparks_client::SparkClient;
use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

// Shared application state
pub struct AppState {
    pub spark_client: Arc<SparkClient>,
}

// Handler functions
pub async fn add_sim_credit(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AddSimCreditRequest>,
) -> Json<ApiResponse<()>> {
    let result = state
        .spark_client
        .add_sim_credit(&req.sub_id, req.amount, req.description.unwrap_or("".to_string()).as_str())
        .await
        .unwrap_or_else(|e| ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        });
    Json(result)
}

// pub async fn set_bitrate(
//     State(state): State<Arc<AppState>>,
//     Json(req): Json<SetBitrateRequest>,
// ) -> Json<ApiResponse<()>> {
//     let result = state
//         .spark_client
//         .set_bitrate(&req.imsi, req.bitrate)
//         .await
//         .unwrap_or_else(|e| ApiResponse {
//             success: false,
//             data: None,
//             error: Some(e.to_string()),
//         });
//     Json(result)
// }

pub async fn get_sim_info(
    State(state): State<Arc<AppState>>,
    Json(req): Json<GetSimInfoRequest>,
) -> Json<ApiResponse<SimInfo>> {
    let result = state
        .spark_client
        .get_sim_info(&req.imsi)
        .await
        .unwrap_or_else(|e| ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        });
    Json(result)
}

pub async fn assign_package(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AssignPackageRequest>,
) -> Json<ApiResponse<Package>> {
    let result = state
        .spark_client
        .assign_package(&req.imsi, &req.package_id, req.days, req.active_date)
        .await
        .unwrap_or_else(|e| ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        });
    Json(result)
}

pub async fn assign_daily_package(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AssignPackageRequest>,
) -> Json<ApiResponse<Package>> {
    let result = state
        .spark_client
        .assign_daily_package(&req.imsi, &req.package_id, None)
        .await
        .unwrap_or_else(|e| ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        });
    Json(result)
}

pub async fn expire_all_package(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AssignPackageRequest>,
) -> Json<ApiResponse<Package>> {
    let result = state
        .spark_client
        .expire_all_package(&req.imsi)
        .await
        .unwrap_or_else(|e| ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        });
    Json(result)
}

pub async fn get_list_packages(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Vec<Package>>> {
    let result = state
        .spark_client
        .get_list_packages()
        .await
        .unwrap_or_else(|e| ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        });
    Json(result)
}


pub async fn get_esim_history(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AssignPackageRequest>,
) -> Json<ApiResponse<Package>> {
    let result = state
        .spark_client
        .get_esim_history(&req.imsi)
        .await
        .unwrap_or_else(|e| ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        });
    Json(result)
}
