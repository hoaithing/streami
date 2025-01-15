use crate::serializers::{CustomResponse, PaginatedSimResponse, DefaultQuery, Product};
use crate::utils::{extract_data, get_products, get_sims_from_db, save_csv_data_to_db};
use axum::extract::{Multipart, Query};
use axum::{debug_handler, Json};
use sqlx::{Pool, Postgres};

pub async fn get_sims_api(Query(filters): Query<DefaultQuery>) -> Json<PaginatedSimResponse> {
    let (total, sims) = get_sims_from_db(filters).await;
    Json(PaginatedSimResponse {
        total,
        results: sims,
    })
}

pub async fn products_api(Query(filters): Query<DefaultQuery>) -> Json<Vec<Product>> {
    let products = get_products(filters).await;
    Json(products)
}

// Handler for file upload
#[debug_handler]
pub async fn upload(
    _pool: axum::extract::State<Pool<Postgres>>,
    multipart: Multipart,
) -> Json<CustomResponse> {
    let data = extract_data(multipart).await;
    if data.is_err() {
        return Json(CustomResponse::default());
    }
    let (provider, file_path, esim) = data.unwrap();
    save_csv_data_to_db(file_path, esim, provider).await;
    Json(CustomResponse::default())
}
