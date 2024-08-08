use crate::tcop::pg;
use crate::config::Config;
use common::Result;
use tracing::info;

async fn initialize() -> Result<()> {
    Ok(())
}

// standalone for pgrs backend
pub async fn run(config: Config) -> Result<()> {
    info!("pg.rs worker(standalone) starting...");
    initialize().await?;
    pg::run(config).await
}
