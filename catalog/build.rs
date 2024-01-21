fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/catalog/v1/catalog.proto")?;
    Ok(())
}
