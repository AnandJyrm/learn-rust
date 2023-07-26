use grpc_clock::grpc_clock_service_client::GrpcClockServiceClient as GeneratedGrpcClockServiceClient;
use grpc_clock::grpc_clock_service_server::{GrpcClockService, GrpcClockServiceServer};
use grpc_clock::{Empty as GeneratedEmpty, GrpcClockTime as GeneratedGrpcClockTime};
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use tonic::transport::server::Router;
use tonic::{transport::Server, Request, Response, Status};

pub mod grpc_clock {
    tonic::include_proto!("grpc_clock");
}

pub const SERVADDR: std::net::SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 50051);
pub const CLNTADDR: &str = "http://127.0.0.1:50051";

pub type GrpcClockTime = GeneratedGrpcClockTime;

pub type Empty = GeneratedEmpty;
pub type GrpcClockServiceClient<T> = GeneratedGrpcClockServiceClient<T>;

impl fmt::Display for GrpcClockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}m {}s", self.minute, self.second)
    }
}

#[derive(Debug, Default)]
pub struct MyGrpcClock {
    pub current_clock: Arc<Mutex<GrpcClockTime>>,
}

#[tonic::async_trait]
impl GrpcClockService for MyGrpcClock {
    async fn get(&self, _request: Request<Empty>) -> Result<Response<GrpcClockTime>, Status> {
        let current_clock_unlocked = self.current_clock.lock().unwrap();
        Ok(Response::new(GrpcClockTime {
            minute: current_clock_unlocked.minute,
            second: current_clock_unlocked.second,
        }))
    }
    async fn set(&self, request: Request<GrpcClockTime>) -> Result<Response<Empty>, Status> {
        let current_request: GrpcClockTime = request.into_inner();
        let mut current_clock_unlocked = self.current_clock.lock().unwrap();
        current_clock_unlocked.minute = current_request.minute;
        current_clock_unlocked.second = current_request.second;
        Ok(Response::new(Empty {}))
    }
}

pub fn build_server(grpc_clock_arc: Arc<Mutex<GrpcClockTime>>) -> Router {
    Server::builder()
        .concurrency_limit_per_connection(20)
        .add_service(GrpcClockServiceServer::new(MyGrpcClock {
            current_clock: grpc_clock_arc,
        }))
}

pub fn build_get_request() -> tonic::Request<Empty> {
    tonic::Request::new(Empty {})
}

pub fn build_set_request(set_minute: u32, set_second: u32) -> tonic::Request<GrpcClockTime> {
    tonic::Request::new(GrpcClockTime {
        minute: set_minute,
        second: set_second,
    })
}

pub fn convert_arg_to_u32(input_str: &str) -> u32 {
    input_str.to_string().parse::<u32>().unwrap()
}