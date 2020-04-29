use futures_0_3::stream::{Stream, StreamExt};
use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use opentelemetry::{api::Provider, global, sdk};
use opentelemetry_datadog;
use tracing::info_span;
use tracing_futures::Instrument;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{
    filter::{EnvFilter},
    layer::SubscriberExt,
    registry::Registry,
};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

impl MyGreeter {
    async fn get_reply(req: HelloRequest) -> HelloReply {
        hello_world::HelloReply {
            message: req.name,
        }
    }
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let span = info_span!("say_hello server");
        let reply = async move {
            // This is only in here to try and mimic a prod issue
            Self::get_reply(request.into_inner()).await
        }
        .instrument(span)
        .await;
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
                message: r.name,
            })
        });

        Ok(Response::new(
            Box::pin(resp_stream) as Self::SayHelloStreamStream
        ))
    }
}

fn configure_tracing() -> Result<(), Box<dyn std::error::Error>> {
    // Create datadog exporter to be able to retrieve the collected spans.
    let exporter = opentelemetry_datadog::Exporter::builder()
        .with_trace_addr("127.0.0.1:3022".parse().unwrap())
        .build();

    // Batching is required to offload from the main thread
    let batch = sdk::BatchSpanProcessor::builder(exporter, tokio::spawn, tokio::time::interval)
        .with_max_queue_size(1000000)
        .with_scheduled_delay(std::time::Duration::from_millis(100))
        .with_max_export_batch_size(100000)
        .build();

    // For the demonstration, use `Sampler::Always` sampler to sample all traces. In a production
    // application, use `Sampler::Parent` or `Sampler::Probability` with a desired probability.
    let provider = sdk::Provider::builder()
        .with_batch_exporter(batch)
        .with_config(sdk::Config {
            default_sampler: Box::new(sdk::Sampler::Probability(1.0)),
            ..Default::default()
        })
        .build();
    global::set_provider(provider);

    // Create a new tracer
    let tracer = global::trace_provider().get_tracer("component_name");

    // Create a new OpenTelemetry tracing layer
    let telemetry = OpenTelemetryLayer::with_tracer(tracer);

    let filter = EnvFilter::from_default_env();

    let subscriber = Registry::default()
        .with(telemetry)
        .with(filter);

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[tokio::main(core_threads = 1)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    configure_tracing()?;

    Server::builder()
        .trace_fn(rust_grpc_bench::tonic::extract_span)
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
