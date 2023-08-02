use env_logger;
use grpc_lib::{GrpcClockTime, SERVADDR};
use log;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

async fn update_minutes(grpc_clock: Arc<Mutex<GrpcClockTime>>) {
    loop {
        sleep(Duration::from_secs(60)).await;
        let mut grpc_clock_unlocked = grpc_clock.lock().unwrap();
        log::debug!(target: "server", "Updated clock: {}", grpc_clock_unlocked);
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
    env_logger::init();
    let start_time = grpc_lib::GrpcClockTime {
        minute: 0,
        second: 0,
    };
    log::info!(target: "server", "Started clock: {}", start_time);
    let clock_start = Arc::new(Mutex::new(start_time));
    tokio::spawn(update_minutes(clock_start.clone()));
    tokio::spawn(update_seconds(clock_start.clone()));

    log::info!(target: "server", "Started server");
    grpc_lib::build_server(clock_start.clone())
        .serve(SERVADDR)
        .await?;
    Ok(())
}
