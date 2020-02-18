use async_stream;
use futures_0_3::future::{self, try_join_all};
use futures_0_3::stream::StreamExt;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::task::spawn;
use tokio::time::{self, delay_for};
use tonic::transport::Channel;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    FmtSubscriber,
};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

struct State {
    request_count: AtomicUsize,
    failed_requests: AtomicUsize,
    in_flight: AtomicUsize,
    max_age: AtomicUsize,
    request_time: AtomicUsize,
    semaphore: tokio::sync::Semaphore,
}

impl State {
    fn new(max_in_flight: usize) -> Self {
        State {
            request_count: AtomicUsize::new(0),
            failed_requests: AtomicUsize::new(0),
            in_flight: AtomicUsize::new(0),
            max_age: AtomicUsize::new(0),
            request_time: AtomicUsize::new(0),
            semaphore: tokio::sync::Semaphore::new(max_in_flight),
        }
    }
}

async fn do_work(mut client: GreeterClient<tonic::transport::Channel>, state: Arc<State>) {
    let start = Instant::now();
    state.in_flight.fetch_add(1, Ordering::SeqCst);

    let request = tonic::Request::new(HelloRequest {
        // name: "Tonic".into(),
        name: 1,
    });

    let response = client.say_hello(request).await;
    match response {
        Ok(_response) => {
            state.request_count.fetch_add(1, Ordering::SeqCst);
        }
        Err(e) => {
            state.failed_requests.fetch_add(1, Ordering::SeqCst);
            println!("{}", e);
        }
    }

    let age = Instant::now().duration_since(start).as_micros() as usize;
    state.request_time.fetch_add(age, Ordering::SeqCst);

    if age > state.max_age.load(Ordering::SeqCst) {
        state.max_age.store(age, Ordering::SeqCst)
    }

    state.in_flight.fetch_sub(1, Ordering::SeqCst);
}

async fn do_work_stream(
    mut client: GreeterClient<tonic::transport::Channel>,
    state: Arc<State>,
) -> Result<(), tonic::Status> {
    let req_state = state.clone();

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    let request_stream = async_stream::stream! {
        let mut seq = 0;
        loop {
            req_state.semaphore.acquire().await.forget();
            req_state.in_flight.fetch_add(1, Ordering::SeqCst);
            let start = Instant::now();
            let req = HelloRequest {
                // name: "Tonic".into(),
                name: seq as u64,
            };
            seq += 1;
            tx.send(start).ok();
            yield req;
        }
    };

    let response = client.say_hello_stream(request_stream).await?;

    response
        .into_inner()
        .zip(rx)
        .enumerate()
        .for_each(move |(i, (r, t))| {
            state.semaphore.add_permits(1);
            match r {
                Ok(response) => {
                    assert_eq!(response.message, i as u64);
                    state.request_count.fetch_add(1, Ordering::SeqCst);
                }
                Err(e) => {
                    state.failed_requests.fetch_add(1, Ordering::SeqCst);
                    println!("{}", e);
                }
            }
            let age = Instant::now().duration_since(t).as_micros() as usize;
            state.request_time.fetch_add(age, Ordering::SeqCst);

            if age > state.max_age.load(Ordering::SeqCst) {
                state.max_age.store(age, Ordering::SeqCst)
            }

            state.in_flight.fetch_sub(1, Ordering::SeqCst);
            future::ready(())
        })
        .await;

    Ok(())
}

async fn work_loop(
    client: GreeterClient<tonic::transport::Channel>,
    state: Arc<State>,
) -> Result<(), tonic::Status> {
    loop {
        do_work_stream(client.clone(), state.clone()).await?;
    }
}

async fn log_loop(state: Arc<State>) {
    let mut last_request_count = 0;
    let mut last = Instant::now();
    let start = Instant::now();
    loop {
        delay_for(Duration::from_secs(1)).await;
        let now = Instant::now();
        let elapsed = now.duration_since(last).as_millis() as f64 / 1000.0;
        let request_count = state.request_count.load(Ordering::SeqCst);
        let req_sec = (request_count - last_request_count) as f64 / elapsed;
        let start_elapsed = now.duration_since(start).as_secs();
        let total_req_sec = request_count / start_elapsed as usize;
        let avg_time = if request_count > last_request_count {
            state.request_time.load(Ordering::SeqCst) / (request_count - last_request_count)
        } else {
            0
        };

        let failed_requests = state.failed_requests.load(Ordering::SeqCst);
        let in_flight = state.in_flight.load(Ordering::SeqCst);
        let max_age = state.max_age.load(Ordering::SeqCst);
        println!(
            "{} total requests ({}/sec last 1 sec) ({}/sec total). last log {} sec ago. {} failed, {} in flight, {} µs max, {} µs avg response time",
            request_count, req_sec, total_req_sec, elapsed, failed_requests, in_flight, max_age, avg_time
        );
        last_request_count = request_count;
        last = now;
        state.max_age.store(0, Ordering::SeqCst);
        state.request_time.store(0, Ordering::SeqCst);
    }
}

// pub fn configure_logging() -> Result<(), Box<dyn std::error::Error>> {
//     fern::Dispatch::new()
//         .format(|out, message, record| {
//             out.finish(format_args!(
//                 "{}[{}][{}:{}] {}",
//                 chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
//                 record.level(),
//                 record.target(),
//                 record.line().unwrap_or(0),
//                 message
//             ));
//         })
//         .level(log::LevelFilter::Info)
//         .level_for("discovery", log::LevelFilter::Trace)
//         .level_for("hyper", log::LevelFilter::Warn)
//         .level_for("tokio_core", log::LevelFilter::Warn)
//         .level_for("tokio_reactor", log::LevelFilter::Warn)
//         .level_for("h2", log::LevelFilter::Warn)
//         .level_for("tower_buffer", log::LevelFilter::Warn)
//         .chain(std::io::stdout())
//         .apply()?;
//     Ok(())
// }

fn configure_tracing(state: Arc<State>) -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::ERROR.into());
    // .add_directive("discovery=trace".parse()?)
    // .add_directive("hyper=warn".parse()?)
    // .add_directive("tokio_core=warn".parse()?)
    // .add_directive("tokio_reactor=warn".parse()?)
    // .add_directive("h2=warn".parse()?);
    // .add_directive("tower_buffer=warn".parse()?);

    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    // let counter = Counter { counts: state.counts.clone() };
    // let subscriber = Registry::default();
    // let subscriber = Registry::default().with(counter);
    // tracing_log::LogTracer::init().map_err(Box::new)?;
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[tokio::main(core_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(State::new(100));
    configure_tracing(state.clone())?;

    let log_state = state.clone();
    spawn(async move {
        log_loop(log_state).await;
    });

    let endpoints = (0..4).map(|_| Channel::from_static("http://[::1]:50052"));
    let channel = Channel::balance_list(endpoints);
    let client = GreeterClient::new(channel);

    let mut futs = vec![];
    for _ in 0..4 {
        // let client = GreeterClient::connect("http://[::1]:50051").await?;
        futs.push(spawn(work_loop(client.clone(), state.clone())));
    }

    try_join_all(futs).await?;

    Ok(())
}
