// 
// pub mod example {
//     tonic::include_proto!("example");
// }
// 
// use example::greeter_server::{Greeter, GreeterServer};
// 
// 
// #[derive(Default)]
// pub struct MyGreeter {}
// 
// #[tonic::async_trait]
// impl Greeter for MyGreeter {
//     async fn say_hello(
//         &self,
//         request: Request<HelloRequest>,
//     ) -> Result<Response<HelloResponse>, Status> {
//         let reply = HelloResponse {
//             message: format!("Hello {}!", request.into_inner().name),
//         };
//         Ok(Response::new(reply))
//     }
// }
