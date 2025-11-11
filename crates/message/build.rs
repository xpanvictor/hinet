extern crate prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = "src/pb";
    std::fs::create_dir_all(out_dir)?;
    prost_build::Config::new()
        .out_dir(out_dir)
        .compile_protos(&["src/proto/schema.proto"], &["src/proto"])?;
    Ok(())
}
