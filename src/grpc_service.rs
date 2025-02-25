use std::collections::HashMap;
use crate::greeter::greeter_server::Greeter;
use crate::greeter::HelloReply;
use crate::greeter::HelloRequest;
use tonic::{Request, Response, Status};
use tracing::info;

#[derive(Debug, Default)]
pub struct MyGreeter {
    pub data: HashMap<String, String>,
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        info!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}