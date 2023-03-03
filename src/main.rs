pub mod hello {
    tonic::include_proto!("helloworld");
}
use hello::{greeter_server::Greeter, HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

#[derive(Default)]
struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello::HelloReply {
            message: format!(
                "Hello, {}",
                request.into_inner().name.unwrap_or("".to_owned())
            ),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let greeter = hello::greeter_server::GreeterServer::new(MyGreeter::default());

    println!("GreeterServer listening on {}", addr);

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(greeter)
        .serve(addr)
        .await?;

    Ok(())
}
