use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    norgopolis_module::Module::start().await.unwrap();

    Ok(())
}
