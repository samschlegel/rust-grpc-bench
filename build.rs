fn main() {
    tonic_build::compile_protos("src/proto/helloworld/helloworld.proto").unwrap();
}
