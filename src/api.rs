use crate::serializers::{Pagination, SimResponse};
use crate::utils::get_sims;
use axum::{
    extract::Query,
    http::StatusCode,
    Json,
};

pub async fn list_sims(
    Query(pagination): Query<Pagination>,
) -> Result<Json<SimResponse>, (StatusCode, String)> {
    let res = get_sims(pagination.page as i64,
                       pagination.page_size as i64).expect("TODO: panic message");
    let total = &res.len();
    Ok(Json(SimResponse {
        count: total.clone() as i64,
        next: "",
        prev: None,
        results: res,
    }))
}
