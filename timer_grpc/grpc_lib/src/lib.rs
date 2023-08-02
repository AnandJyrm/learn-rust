use grpc_clock::grpc_clock_service_client::GrpcClockServiceClient as GeneratedGrpcClockServiceClient;
use grpc_clock::grpc_clock_service_server::{GrpcClockService, GrpcClockServiceServer};
use grpc_clock::{Empty as GeneratedEmpty, GrpcClockTime as GeneratedGrpcClockTime};
use log;
use std::error::Error;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::UnboundedReceiverStream, Stream};
use tonic::transport::server::Router;
use tonic::transport::Channel;
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
        log::info!(target: "server", "GET request");
        Ok(Response::new(GrpcClockTime {
            minute: current_clock_unlocked.minute,
            second: current_clock_unlocked.second,
        }))
    }
    async fn set(&self, request: Request<GrpcClockTime>) -> Result<Response<Empty>, Status> {
        let current_request: GrpcClockTime = request.into_inner();
        log::info!(target: "server", "SET request {}", current_request);
        let mut current_clock_unlocked = self.current_clock.lock().unwrap();
        current_clock_unlocked.minute = current_request.minute;
        current_clock_unlocked.second = current_request.second;
        Ok(Response::new(Empty {}))
    }

    type StreamTimeStream = Pin<Box<dyn Stream<Item = Result<GrpcClockTime, Status>> + Send>>;

    async fn stream_time(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::StreamTimeStream>, Status> {
        log::info!(target: "server", "STREAM start");
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let current_clock_unlocked = GrpcClockTime {
                    minute: 0,
                    second: 0,
                };
                log::info!(target: "server", "STREAM sent time {}", current_clock_unlocked);
                let _ = tx.send(Ok(current_clock_unlocked.clone()));
            }
        });
        let response_stream = UnboundedReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(response_stream) as Self::StreamTimeStream
        ))
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

pub async fn get_time_stream(
    client: &mut GrpcClockServiceClient<Channel>,
) -> Result<(), Box<dyn Error>> {
    let mut stream = client
        .stream_time(Request::new(Empty {}))
        .await?
        .into_inner();

    while let Some(time) = stream.message().await? {
        log::info!(target: "client", "Time = {}", time);
    }

    Ok(())
}
