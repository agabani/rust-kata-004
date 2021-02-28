use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct HttpServerConfiguration {
    pub host: String,
    pub port: u16,
}

impl HttpServerConfiguration {
    pub fn tcp_listener(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(format!("{}:{}", self.host, self.port))
    }
}
