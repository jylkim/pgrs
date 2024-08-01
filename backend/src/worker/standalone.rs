use crate::worker::pg;
use tracing::info;

async fn initialize() -> std::io::Result<()> {
    Ok(())
}

// standalone for pgrs backend
pub async fn run() -> std::io::Result<()> {
    info!("pg.rs worker(standalone) starting...");
    initialize().await?;
    pg::run().await
}
