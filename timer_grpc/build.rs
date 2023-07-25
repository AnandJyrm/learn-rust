use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=$SOURCE_REPO/timer_grpc/proto/clock.proto");

    // protoc doesn't take env variables.
    let env_repo = &env::var("SOURCE_REPO").unwrap();
    let source_path = Path::new(env_repo);
    let proto_dir_path = Path::join(source_path, "timer_grpc/proto/");
    let proto_file_path = Path::join(source_path, "timer_grpc/proto/clock.proto");

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(&[proto_file_path], &[proto_dir_path])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
    Ok(())
}