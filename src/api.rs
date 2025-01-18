use crate::serializers::{CustomResponse, Product, Sim, PaginatedResponse};
use crate::utils::{extract_data, get_data_from_db, save_csv_data_to_db, DynamicFilters};
use crate::constants::{TABLE_PRODUCT, TABLE_SIM};
use axum::extract::{Multipart, Query};
use axum::{debug_handler, Json};
use sqlx::{Pool, Postgres};


pub async fn get_sims_api(Query(filters): Query<DynamicFilters>) -> Json<PaginatedResponse<Sim>> {
    let results = get_data_from_db::<Sim>(filters, TABLE_SIM, None).await;
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

pub async fn list_product_api(Query(filters): Query<DynamicFilters>) -> Json<PaginatedResponse<Product>> {
    let results = get_data_from_db::<Product>(filters, TABLE_PRODUCT, None).await;
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
