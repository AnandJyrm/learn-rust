use tonic::{transport::Server, Request, Response, Status};

use grpc_clock::grpc_clock_service_server::{GrpcClockService, GrpcClockServiceServer};
use grpc_clock::{Empty, GrpcClockTime};

pub mod grpc_clock {
    tonic::include_proto!("grpc_clock");
}

#[derive(Debug, Default)]
pub struct MyGrpcClock {}

#[tonic::async_trait]
impl GrpcClockService for MyGrpcClock {
    async fn get(&self, _request: Request<Empty>) -> Result<Response<GrpcClockTime>, Status> {
        println!("get request");
        Ok(Response::new(GrpcClockTime {
            minute: 2,
            second: 2,
        }))
    }
    async fn set(&self, request: Request<GrpcClockTime>) -> Result<Response<Empty>, Status> {
        let current_request: GrpcClockTime = request.into_inner();
        println!(
            "set request: {}m {}s",
            current_request.minute, current_request.second
        );
        Ok(Response::new(Empty {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let my_clock = MyGrpcClock::default();

    Server::builder()
        .add_service(GrpcClockServiceServer::new(my_clock))
        .serve(addr)
        .await?;

    Ok(())
}

// TODO: implement
// 1) common variable time
// 2) implement timer to increment time
// 3) run timer and grpc in concurrently
