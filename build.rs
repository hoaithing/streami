fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        // .out_dir("src/pb") // This will output generated code to src/pb directory
        .compile_protos(
            &["proto/service.proto"],
            &["proto"], // Include directory
        )?;
    Ok(())
}
