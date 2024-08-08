use crate::pq::Connection;
use crate::config::Config;
use common::{Result, DEFAULT_HOST, DEFAULT_PORT};
use common::pq::{RequestMessage, ResponseCode, ResponseMessage};

use tracing::info;

struct Listener {
    listener: tokio::net::TcpListener,
}

struct Handler {
    connection: Connection,
}

impl Listener {
    fn new(listener: tokio::net::TcpListener) -> Self {
        Self {
            listener,
        }
    }

    async fn run(&self) -> Result<()> {
        loop {
            info!("server waiting for connection...");
            let (socket, _) = self.listener.accept().await?;
            info!("server connection accepted...");
            Handler::new(socket)
                .run()
                .await?;
        }
    }
}

impl Handler {
    fn new(socket: tokio::net::TcpStream) -> Self {
        Self {
            connection: Connection::new(socket),
        }
    }

    async fn run(&mut self) -> Result<()> {
        info!("server start receiving...");
        let req_msg: RequestMessage = self.connection.recieve().await?;
        info!("server received message: {:?}", req_msg);
        let res_msg = self.handle(req_msg);
        info!("server response message: {:?}", res_msg);
        self.connection.response(&res_msg).await?;
        info!("server response sent...");
        Ok(())
    }

    fn handle(&self, request: RequestMessage) -> ResponseMessage {
        ResponseMessage {
            code: ResponseCode::Complete,
            payload: request.payload.clone(),
        }
    }
}

// core backend for pgrs.
// invoked by two modules, standalone & backend (foked via master).
pub async fn run() -> Result<()> {
    // TODO: use config file to get the address
    let config = Config::new(DEFAULT_HOST.into(), DEFAULT_PORT);
    let addr = config.address();
    info!("starting to listen on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let listener = Listener::new(listener);
    listener.run().await
}

