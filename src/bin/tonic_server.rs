use futures_0_3::stream::{Stream, StreamExt};
use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status, Streaming};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        // println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            // message: format!("Hello {}!", request.into_inner().name),
            message: request.into_inner().name,
        };
        Ok(Response::new(reply))
    }

    type SayHelloStreamStream =
        Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send + Sync + 'static>>;
    async fn say_hello_stream(
        &self,
        request: Request<Streaming<HelloRequest>>,
    ) -> Result<Response<Self::SayHelloStreamStream>, Status> {
        let resp_stream = request.into_inner().map(|r| {
            r.map(|r| HelloReply {
                // message: format!("Hello {}!", r.name),
                message: r.name,
            })
        });

        Ok(Response::new(
            Box::pin(resp_stream) as Self::SayHelloStreamStream
        ))
    }
}

#[tokio::main(core_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
