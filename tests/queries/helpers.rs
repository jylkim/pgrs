use pgrs::tcop::standalone_run;
use pgrs::config::get_config;
use common::Result;
use tokio::net::TcpStream;

pub async fn spawn_server() -> String {
    let config = get_config();
    let addr = config.address();
    tokio::spawn(async move {
        standalone_run(config).await.unwrap();
    });
    addr
}

pub async fn connect_server(addr: &str) -> Result<TcpStream> {
    let mut retries = 0;
    let max_retry = 5;
    loop {
        match TcpStream::connect(addr).await {
            Ok(stream) => return Ok(stream),
            Err(e) => {
                if retries > max_retry {
                    return Err(e.into());
                }
                retries += 1;
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    };
}
