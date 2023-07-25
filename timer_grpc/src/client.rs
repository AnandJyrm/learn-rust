use grpc_clock::grpc_clock_service_client::GrpcClockServiceClient;
use grpc_clock::{Empty, GrpcClockTime};
use std::thread::sleep;
use std::time;

pub mod grpc_clock {
    tonic::include_proto!("grpc_clock");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let three_s = time::Duration::from_millis(3000);
    let mut client = GrpcClockServiceClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(Empty {});
    let response = client.get(request).await?;
    let current_time: GrpcClockTime = response.into_inner();
    println!(
        "Current time: {}m {}s",
        current_time.minute, current_time.second
    );

    println!("zzz ... sleep 3 s");
    sleep(three_s);

    let request = tonic::Request::new(Empty {});
    let response = client.get(request).await?;
    let current_time: GrpcClockTime = response.into_inner();
    println!(
        "Current time: {}m {}s",
        current_time.minute, current_time.second
    );

    println!("zzz ... sleep 3 s");
    sleep(three_s);

    let request = tonic::Request::new(GrpcClockTime {
        minute: 0,
        second: 0,
    });
    let _response = client.set(request).await?;
    println!("Reset time to 0m 0s");

    println!("zzz ... sleep 3 s");
    sleep(three_s);

    let request = tonic::Request::new(Empty {});
    let response = client.get(request).await?;
    let current_time: GrpcClockTime = response.into_inner();
    println!(
        "Current time: {}m {}s",
        current_time.minute, current_time.second
    );
    Ok(())
}
