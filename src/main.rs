use std::sync::Arc;
use axum::routing::post;
use axum::{extract::DefaultBodyLimit, routing::get, Router};
use streami::sims::sim_api::{get_sims_api, list_product_api, upload};
use streami::sparks::sparks_api::{add_sim_credit, assign_daily_package, assign_package, expire_all_package, get_esim_history, get_list_packages, get_sim_info, AppState};
use streami::utils::{create_pool, get_file_content};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{span, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use streami::sparks::sparks_client::SparkClient;
use tracing_appender::{non_blocking::NonBlocking, rolling};
use tracing_appender::non_blocking::WorkerGuard;

pub fn setup_logging() -> WorkerGuard {
    // Only set up logging if it hasn't been set up before
    static INIT: std::sync::Once = std::sync::Once::new();
    // Set up file appender with rotation
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = NonBlocking::new(file_appender);
    let _guard = _guard;

    INIT.call_once(|| {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));
        let fmt_layer = fmt::layer()
            .with_file(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false);
            // .pretty();

        if let Err(e) = tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .with(fmt::Layer::new().with_ansi(false).with_writer(non_blocking))
            .try_init()
        {
            eprintln!("Warning: Failed to initialize tracing subscriber: {}", e);
        }
    });
    _guard
}

#[tokio::main]
async fn main() {
    let _guard = setup_logging();
    // CORS configuration
    tracing::info!("Logging initialized!");
    let pool = create_pool().await;
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let spark_client = SparkClient::new(None).expect("Failed to create Spark client");
    let shared_state = Arc::new(AppState {
        spark_client: Arc::new(spark_client),
    });

    // Build our application with routes
    let app = Router::new()
        .route("/upload", post(upload))
        .route("/file", get(get_file_content))
        .route("/sims", get(get_sims_api))
        .route("/products", get(list_product_api))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .with_state(pool)
        .route("/sim/credit", post(add_sim_credit))
        .route("/sim/info", post(get_sim_info))
        .route("/packages", get(get_list_packages))
        .route("/package/assign", post(assign_package))
        .route("/package/assign_daily", post(assign_daily_package))
        .route("/package/expire", post(expire_all_package))
        .route("/esim/history", post(get_esim_history))
        .with_state(shared_state)
        // .route("/sim/bitrate", post(set_bitrate))
        // .route("/sim/status", post(change_sim_status))
        // .route("/sim/number", post(change_sim_number))
        // .route("/sim/packages", post(get_sim_packages))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Run it with hyper on localhost:8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
