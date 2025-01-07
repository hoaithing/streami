// use crate::serializers::{SimQuery, SimResponse};
// use crate::utils::get_sims;
// use axum::{extract::Query, http::StatusCode, Json, debug_handler};
//
//
// #[debug_handler]
// pub async fn list_sims(
//     Query(query): Query<SimQuery>,
// ) -> Result<Json<SimResponse>, (StatusCode, String)> {
//     let res = get_sims(query.page, query.page_size, query.provider, query.search).expect("TODO: panic message");
//     let total = &res.len();
//     Ok(Json(SimResponse {
//         count: *total as i64,
//         next: "",
//         prev: None,
//         results: res,
//     }))
// }
