use crate::worker::pg;
use common::Result;
use tracing::info;

async fn initialize() -> Result<()> {
    Ok(())
}

// standalone for pgrs backend
pub async fn run() -> Result<()> {
    info!("pg.rs worker(standalone) starting...");
    initialize().await?;
    pg::run().await
}

mod tests {
    use super::*;
    use crate::config::Config;
    use common::pq::{send, recv, RequestMessage, RequestCode, ResponseMessage, ResponseCode};
    use tokio::net::TcpStream;
    use tokio::time::{sleep, Duration};


    #[tokio::test]
    async fn echo_test() {
        // arrange
        tokio::spawn(async move {
            run().await.unwrap();
        });
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        let config = Config::new(common::DEFAULT_HOST.into(), common::DEFAULT_PORT);
        let mut client = TcpStream::connect(config.address()).await.unwrap();

        // act
        let message = RequestMessage {
            code: RequestCode::Query,
            payload: "select * from dual".into(),
        };
        send(&mut client, &message).await.unwrap();
        let response = recv(&mut client).await.unwrap();

        // assert
        assert_eq!(ResponseCode::Complete, response.code);
        assert_eq!(message.payload, response.payload);

    }
}
