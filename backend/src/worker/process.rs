use crate::worker::pg;
use tracing::info;

pub async fn run() -> std::io::Result<()> {
    info!("pg.rs worker(process) starting...");
    pg::run().await
}
