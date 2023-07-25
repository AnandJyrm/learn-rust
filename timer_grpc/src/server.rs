use grpc_clock::grpc_clock_service_server::{GrpcClockService, GrpcClockServiceServer};
use grpc_clock::{Empty, GrpcClockTime};
use rand::{distributions::Alphanumeric, Rng};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use tonic::{transport::Server, Request, Response, Status};

pub mod grpc_clock {
    tonic::include_proto!("grpc_clock");
}

#[derive(Debug, Default)]
pub struct MyGrpcClock {
    pub grpc_clock: Arc<Mutex<GrpcClockTime>>,
}

#[tonic::async_trait]
impl GrpcClockService for MyGrpcClock {
    async fn get(&self, _request: Request<Empty>) -> Result<Response<GrpcClockTime>, Status> {
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        println!("get request {id}");
        let grpc_clock_unlocked = self.grpc_clock.lock().unwrap();
        println!("got lock {id}");
        Ok(Response::new(GrpcClockTime {
            minute: grpc_clock_unlocked.minute,
            second: grpc_clock_unlocked.second,
        }))
    }
    async fn set(&self, request: Request<GrpcClockTime>) -> Result<Response<Empty>, Status> {
        let current_request: GrpcClockTime = request.into_inner();
        println!(
            "set request: {}m {}s",
            current_request.minute, current_request.second
        );
        let mut grpc_clock_unlocked = self.grpc_clock.lock().unwrap();
        grpc_clock_unlocked.minute = current_request.minute;
        grpc_clock_unlocked.second = current_request.second;
        Ok(Response::new(Empty {}))
    }
}

async fn update_minutes(grpc_clock: Arc<Mutex<GrpcClockTime>>) {
    loop {
        sleep(Duration::from_secs(60)).await;
        let mut grpc_clock_unlocked = grpc_clock.lock().unwrap();
        grpc_clock_unlocked.minute = (grpc_clock_unlocked.minute + 1) % 60;
    }
}

async fn update_seconds(grpc_clock: Arc<Mutex<GrpcClockTime>>) {
    loop {
        sleep(Duration::from_secs(1)).await;
        let mut grpc_clock_unlocked = grpc_clock.lock().unwrap();
        grpc_clock_unlocked.second = (grpc_clock_unlocked.second + 1) % 60;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let current_grpc_clock = Arc::new(Mutex::new(GrpcClockTime {
        minute: 0,
        second: 0,
    }));
    tokio::spawn(update_minutes(current_grpc_clock.clone()));
    tokio::spawn(update_seconds(current_grpc_clock.clone()));

    Server::builder()
        .concurrency_limit_per_connection(20)
        .add_service(GrpcClockServiceServer::new(MyGrpcClock {
            grpc_clock: Arc::clone(&current_grpc_clock),
        }))
        .serve(addr)
        .await?;

    Ok(())
}
