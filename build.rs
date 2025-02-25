use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .compile_protos(
            &["protos/greeter.proto"], // Update the path to the .proto file
            &["protos"],              // Update the search path for proto files
        )?;
    Ok(())
}
