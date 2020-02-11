fn main() {
    tonic_build::compile_protos("src/proto/helloworld/helloworld.proto").unwrap();
    let proto_root = "src/proto";
    protoc_grpcio::compile_grpc_protos(
        &["helloworld/helloworld.proto"],
        &[proto_root],
        &proto_root,
        None,
    )
    .expect("Failed to compile gRPC definitions!");
}
