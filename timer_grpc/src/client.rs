use env_logger;
use grpc_lib::{
    build_get_request, build_set_request, convert_arg_to_u32, get_time_stream,
    GrpcClockServiceClient, CLNTADDR,
};
use log;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GrpcClockServiceClient::connect(CLNTADDR).await?;
    let args: Vec<String> = env::args().collect();
    env_logger::init();
    let _ = match args[1].as_str() {
        "set" => {
            let set_minute = convert_arg_to_u32(&args[2]);
            let set_second = convert_arg_to_u32(&args[3]);
            let request = build_set_request(set_minute, set_second);
            log::info!(target: "client", "SET time: {}m {}s", set_minute, set_second);
            let _response = client.set(request).await?;
        }
        "get" => {
            let request = build_get_request();
            let response = client.get(request).await?;
            log::info!(target: "client", "GOT time: {}", response.into_inner());
        }
        "stream" => {
            log::info!(target: "client", "Requesting Stream");
            get_time_stream(&mut client).await?;
        }
        &_ => todo!(),
    };
    Ok(())
}
