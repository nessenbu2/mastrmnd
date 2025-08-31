fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile_protos(&["proto/mastrmnd.proto"], &["proto"])
        .expect("Failed to compile protos");
    println!("cargo:rerun-if-changed=proto/mastrmnd.proto");
}