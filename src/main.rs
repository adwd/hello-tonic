pub mod hello {
    tonic::include_proto!("helloworld");
}
use std::{pin::Pin, time::Duration};

use hello::{
    greeter_server::Greeter, oneof_reply::Payload, HelloReply, HelloRequest, OneofReply,
    OneofRequest,
};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{codegen::futures_core::Stream, transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

#[derive(Default)]
struct MyGreeter {}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send>>;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("SayHello");
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello::HelloReply {
            message: format!(
                "Hello, {}",
                request.into_inner().name.unwrap_or("".to_owned())
            ),
        };
        Ok(Response::new(reply))
    }

    type StreamingHelloStream = ResponseStream;

    async fn streaming_hello(
        &self,
        req: Request<HelloRequest>,
    ) -> Result<Response<Self::StreamingHelloStream>, Status> {
        println!("StreamingHello");
        println!("Got a request from {:?}", req.remote_addr());

        // creating infinite stream with requested message
        let repeat = std::iter::repeat(HelloReply {
            message: req.into_inner().name.unwrap_or("Tonic".to_owned()),
        });
        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::StreamingHelloStream
        ))
    }

    async fn oneof_hello(
        &self,
        req: Request<OneofRequest>,
    ) -> Result<Response<OneofReply>, Status> {
        let message = req.into_inner().message;
        println!("{message:?}");
        match message {
            Some(m) if !m.is_empty() => Ok(Response::new(OneofReply {
                message: "str response".into(),
                payload: Some(Payload::Str(format!("hello, {m}"))),
            })),
            _ => Ok(Response::new(OneofReply {
                message: "int response".into(),
                payload: Some(Payload::I32(0)),
            })),
        }
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
