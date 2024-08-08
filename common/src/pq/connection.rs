use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::pq::protocol::{MessageCode, Message};
use crate::Result;

pub async fn recv<T>(stream: &mut TcpStream) -> Result<Message<T>>
where
    T: MessageCode + serde::de::DeserializeOwned + std::fmt::Debug,
{
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;

    let message = serde_json::from_slice(&buffer[..n])? ;
    Ok(message)
}

pub async fn send<T>(stream: &mut TcpStream, message: &Message<T>) -> Result<()>
where
    T: MessageCode + serde::Serialize + std::fmt::Debug
{
    let response = serde_json::to_vec(&message)?;
    stream.write_all(response.as_slice()).await?;
    Ok(())
}
