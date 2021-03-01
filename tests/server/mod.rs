pub struct TestServer {
    pub address: String,
}

impl TestServer {
    pub async fn spawn(overrides: &[(&str, &str)]) -> Self {
        let defaults = &[("http_server.port", "0")];

        let (server, port, _) = rust_kata_004::run(&[defaults, overrides].concat()).await;

        let _ = tokio::spawn(server);

        Self {
            address: format!("http://127.0.0.1:{}", port),
        }
    }
}
