use crate::serializers::{CustomResponse, PaginatedSimResponse, SimQuery};
use crate::utils::{extract_data, get_sims_from_db, save_csv_data_to_db};
use axum::extract::{Multipart, Query};
use axum::{debug_handler, Json};
use sqlx::{Pool, Postgres};

pub async fn get_sims(Query(filters): Query<SimQuery>) -> Json<PaginatedSimResponse> {
    let (count, sims) = get_sims_from_db(filters).await;
    Json(PaginatedSimResponse {
        count,
        results: sims,
    })
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
