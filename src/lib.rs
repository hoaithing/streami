pub mod utils;
pub mod sparks;
pub mod sims;
pub mod sales;

pub mod greeter {
    tonic::include_proto!("greeter");
}

pub mod grpc_service;