fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/communication.proto")?;
    tonic_build::compile_protos("../protos/module.proto")?;
    Ok(())
}
