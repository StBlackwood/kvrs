fn main() {
    capnpc::CompilerCommand::new()
        .file("schema/kv.capnp")
        .output_path("src/protocol")

        .run()
        .expect("capnp compile failed");
}