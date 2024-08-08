use common::telemetry;
use common::Result;
use pgrs::tcop;
use pgrs::master;
use pgrs::config::get_config;

use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // print help
    // TODO: use clap crate
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "-h" || arg == "--help" {
            help();
            return Ok(());
        } else if arg == "--version" {
            version();
            return Ok(());
        }
    }
    // initialize
    telemetry::init_tracing();
    info!("pg.rs server starting...");
    let config = get_config();
    //startup
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "-sa" || arg == "--standalone" {
            tcop::standalone_run(config).await?;
        }
    } else {
        master::run(config).await?;
    }
    info!("pg.rs server stopped. finalizing...");
    Ok(())
}

fn help() {
    println!("pg.rs server.");
    println!("Usage:");
    println!("  pgrs [options]");
    println!("Options:");
    println!("  -h, --help\t\tShow this help message and exit.");
    println!("  -sa, --standalone\tRun as a single process.");
}

fn version() {
    println!("pg.rs server version: {}", env!("CARGO_PKG_VERSION"));
}
