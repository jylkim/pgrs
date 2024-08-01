use tokio::io::AsyncReadExt;
use tracing::info;

struct Config {
    ip: String,
    port: String,
}

struct Listener {
    listener: tokio::net::TcpListener,
}

struct Handler {
    socket: tokio::net::TcpStream,
}

impl Config {
    fn new(ip: String, port: String) -> Self {
        Self {
            ip,
            port,
        }
    }

    fn get_addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl Listener {
    fn new(listener: tokio::net::TcpListener) -> Self {
        Self {
            listener,
        }
    }

    async fn run(&self) -> std::io::Result<()> {
        loop {
            info!("waiting for connection...");
            let (socket, _) = self.listener.accept().await?;
            let mut handler = Handler::new(socket);
            tokio::spawn(async move {
                handler.run().await.unwrap();
            });
        }
    }
}

impl Handler {
    fn new(socket: tokio::net::TcpStream) -> Self {
        Self {
            socket,
        }
    }

    async fn run(&mut self) -> std::io::Result<()> {
        let mut buf = [0; 1024];
        let _ = self.socket.read(&mut buf).await?;
        Ok(())
    }
}

// core backend for pgrs.
// invoked by two modules, standalone & backend (foked via master).
pub async fn run() -> std::io::Result<()> {
    // TODO: use config file to get the address
    let config = Config::new("127.0.0.1".into(), "5432".into());
    let addr = config.get_addr();
    info!("starting to listen on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let listener = Listener::new(listener);
    listener.run().await
}
