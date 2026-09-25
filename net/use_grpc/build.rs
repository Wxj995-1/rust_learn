use std::env;
use std::process::Command;

extern crate protoc_rust_grpc;

fn protoc_available() -> bool {
    let protoc = env::var_os("PROTOC").unwrap_or_else(|| "protoc".into());
    Command::new(protoc)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn main() {
    println!("cargo:rerun-if-changed=foobar.proto");
    println!("cargo:rerun-if-changed=build.rs");

    if !protoc_available() {
        println!(
            "cargo:warning=protoc not found, using the pre-generated src/foobar.rs and src/foobar_grpc.rs"
        );
        return;
    }

    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .input("foobar.proto")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}
