fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setting `PROTOC` with binary from `protoc_bin_vendored`
    let path_buf = protoc_bin_vendored::protoc_bin_path()?;
    std::env::set_var("PROTOC", path_buf.as_path());

    // Compiling `.proto`s
    tonic_build::compile_protos("proto/authentication.proto")?;
    Ok(())
}
