use axum::routing::post;
use axum::{extract::DefaultBodyLimit, routing::get, Router};
use streami::utils::get_sims;
use streami::utils::{create_pool, get_file_content, upload};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // CORS configuration
    let pool = create_pool().await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/upload", post(upload))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .route("/api/file", get(get_file_content))
        .route("/api/sims", get(get_sims))
        .layer(cors)
        .with_state(pool);

    // Run it with hyper on localhost:8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
