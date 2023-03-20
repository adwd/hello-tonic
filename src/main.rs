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
use tonic::{
    codegen::futures_core::Stream, transport::Server, Request, Response, Status, Streaming,
};
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
        println!("Got a request from {:?}", req.remote_addr().unwrap());

        let message = req.into_inner().name.unwrap_or("Tonic".to_owned());

        // creating infinite stream with requested message
        let repeat = std::iter::repeat(());
        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);
        let mut count = 0;
        tokio::spawn(async move {
            while let Some(_) = stream.next().await {
                if count >= 20 {
                    break;
                }
                count += 1;
                let item = HelloReply {
                    message: format!("Streaming {}: {}", message, count),
                };
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

    async fn client_streaming_hello(
        &self,
        request_stream: Request<Streaming<HelloRequest>>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("ClientStreamingHello");
        println!(
            "Got a request from {:?}",
            request_stream.remote_addr().unwrap()
        );

        let mut stream = request_stream.into_inner();
        let mut buf = vec![];

        while let Some(req) = stream.next().await {
            match req {
                Ok(hello_req) => {
                    if hello_req.name() == "end" {
                        break;
                    }
                    println!("{}", &hello_req.name());
                    hello_req.name.map(|n| buf.push(n));
                }
                Err(e) => {
                    println!("error: {e:?}");
                    break;
                }
            }
        }

        Ok(Response::new(HelloReply {
            message: format!("received messages: {}", buf.join(", ")),
        }))
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
