use crate::sales::serializers::SalePartnerList;
use crate::sims::constants::TABLE_SALE_PARTNER;
use crate::sims::serializers::{DynamicFilters, PaginatedResponse};
use crate::sims::utils::get_data_from_db;
use axum::extract::{Query, State};
use axum::Json;
use sqlx::{Pool, Postgres};

pub async fn get_sales_partner_api(
    State(pool): State<Pool<Postgres>>,
    Query(mut filters): Query<DynamicFilters>,
) -> Json<PaginatedResponse<SalePartnerList>> {
    let select_columns = [
        "id",
        "booking_id",
        "email",
        "name",
        "source",
        "product",
        "package_id",
        "created",
        "booking_date",
        "sim_number",
        "uid",
    ];

    filters.search_fields = Some(vec!["booking_id".to_string(), "email".to_string()]);

    let results = get_data_from_db::<SalePartnerList>(
        pool,
        filters,
        TABLE_SALE_PARTNER,
        Some(select_columns.join(",").as_str()),
    )
    .await;

    if let Ok((total, sales)) = results {
        Json(PaginatedResponse {
            total,
            results: sales,
        })
    } else {
        tracing::warn!("Failed to fetch data from db: {}", results.err().unwrap());
        Json(PaginatedResponse {
            total: 0,
            results: Vec::new(),
        })
    }
}
