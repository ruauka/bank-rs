fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&["proto/bank.proto"], &["../."])
        .unwrap_or_else(|err| panic!("protobuf compilation failed: {}", err));

    // tonic_build::compile_protos("../proto/bank.proto")?;
    Ok(())
}
