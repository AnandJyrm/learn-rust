use grpc_lib::{GrpcClockServiceClient, CLNTADDR, convert_arg_to_u32, build_set_request, build_get_request};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GrpcClockServiceClient::connect(CLNTADDR).await?;
    let args: Vec<String> = env::args().collect();
    let _ = match args[1].as_str() {
        "set" => {
            let set_minute = convert_arg_to_u32(&args[2]);
            let set_second = convert_arg_to_u32(&args[3]);
            let request = build_set_request(set_minute, set_second);
            let _response = client.set(request).await?;
        },
        "get" => {
            let request = build_get_request();
            let response = client.get(request).await?;
            println!("Current time: {}", response.into_inner());
        },
        &_ => todo!(),
    };
    Ok(())
}
