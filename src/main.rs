use std::sync::Arc;
use axum::routing::post;
use axum::{extract::DefaultBodyLimit, routing::get, Router};
use streami::sims::sim_api::{get_sims_api, list_product_api, upload};
use streami::sparks::sparks_api::{add_sim_credit, assign_daily_package, assign_package, expire_all_package, get_esim_history, get_list_packages, get_sim_info, AppState};
use streami::utils::{create_pool, get_file_content};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use streami::sparks::sparks_client::SparkClient;

use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello");
}


use example::greeter_server::{Greeter, GreeterServer};
use example::{HelloRequest, HelloResponse};
#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let reply = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}



#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_app=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // CORS configuration
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
