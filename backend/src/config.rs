pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
