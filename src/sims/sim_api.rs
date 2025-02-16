use crate::sims::constants::{TABLE_PRODUCT, TABLE_SIM};
use crate::sims::serializers::{CustomResponse, DynamicFilters, PaginatedResponse, Product, Sim, XploriStatusCode};
use crate::sims::utils::get_data_from_db;
use crate::utils::{extract_data, save_csv_data_to_db};
use axum::extract::{Multipart, Query, State};
use axum::{debug_handler, Json};
use serde_json::json;
use sqlx::{Pool, Postgres};

pub async fn get_sims_api(
    State(pool): State<Pool<Postgres>>,
    Query(mut filters): Query<DynamicFilters>,
) -> Json<PaginatedResponse<Sim>> {
    let select_columns = [
        "id",
        "sim_id",
        "sim_serial",
        "sim_number",
        "active",
        "esim",
        "status",
        "provider",
    ];

    filters.search_fields = Some(vec!["sim_id".to_string(), "sim_serial".to_string()]);

    let results= get_data_from_db::<Sim>(
        pool,
        filters,
        TABLE_SIM,
        Some(select_columns.join(",").as_str()),
    ).await;

    if let Ok((total, sims)) = results {
        Json(PaginatedResponse {
            total,
            results: sims,
        })
    } else {
        println!("Failed to fetch data from db: {}", results.err().unwrap());
        Json(PaginatedResponse {
            total: 0,
            results: Vec::new(),
        })
    }
}

pub async fn list_product_api(
    State(pool): State<Pool<Postgres>>,
    Query(mut filters): Query<DynamicFilters>,
) -> Json<PaginatedResponse<Product>> {
    filters.search_fields = Some(vec!["name".to_string(), "sku".to_string()]);
    let results = get_data_from_db::<Product>(pool, filters, TABLE_PRODUCT, None).await;
    if let Ok((total, products)) = results {
        Json(PaginatedResponse {
            total,
            results: products,
        })
    } else {
        println!("Failed to fetch data from db: {}", results.err().unwrap());
        Json(PaginatedResponse {
            total: 0,
            results: Vec::new(),
        })
    }
}

#[debug_handler]
pub async fn upload(
    _pool: State<Pool<Postgres>>,
    multipart: Multipart,
) -> Json<CustomResponse> {
    let data = extract_data(multipart).await;
    if data.is_err() {
        return Json(CustomResponse::default());
    }
    let (provider, file_path, esim) = data.unwrap();
    let (success, errors) = save_csv_data_to_db(file_path, esim, provider).await;
    Json(CustomResponse {
        data: json!({
            "success": success,
            "errors": errors,
        }),
        message: "sucess".to_string(),
        status: XploriStatusCode::Success
    })
}
