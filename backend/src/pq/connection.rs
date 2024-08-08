use tokio::net::TcpStream;
use common::Result;
use common::pq::{RequestMessage, ResponseMessage};

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
           }
    }

    pub async fn recieve(&mut self) -> Result<RequestMessage> {
        common::pq::recv(&mut self.stream).await
    }

    pub async fn response(&mut self, message: &ResponseMessage) -> Result<()> {
        common::pq::send(&mut self.stream, message).await
    }
}
