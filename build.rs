fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Adding to `PATH` binary from `protoc_bin_vendored`.
    // `protoc_bin_vendored` works well on local machines, but not so much within `Docker`, so installing the binary is still needed.
    let env_path = std::env::var("PATH")?;
    let path_buf = protoc_bin_vendored::protoc_bin_path()?.parent().expect("Valid parent path").to_path_buf();
    let new_env_path = vec![std::env::split_paths(&env_path).collect(), vec![path_buf], ].into_iter().flatten();
    let os_path = std::env::join_paths(new_env_path)?;
    std::env::set_var("PATH", os_path);

    // Compiling `.proto`s
    tonic_build::compile_protos("proto/authentication.proto")?;
    Ok(())
}
