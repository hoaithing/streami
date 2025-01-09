use axum::extract::Query;
use axum::Json;
use crate::serializers::{PaginatedSimResponse, SimQuery};
use crate::utils::get_sims_from_db;

pub async fn get_sims(Query(filters): Query<SimQuery>) -> Json<PaginatedSimResponse> {
    let (count, sims) = get_sims_from_db(filters).await;
    Json(PaginatedSimResponse {
        next: "",
        prev: None,
        count,
        results: sims,
    })
}