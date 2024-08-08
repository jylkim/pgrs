pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub fn get_config() -> Config {
    Config::new(common::DEFAULT_HOST.into(), common::DEFAULT_PORT)
}
