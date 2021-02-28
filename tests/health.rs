mod server;

use crate::server::TestServer;
use reqwest::Client;

#[actix_rt::test]
async fn health_check_liveness_works() {
    let server = TestServer::spawn(&[]).await;
    let client = Client::new();

    let response = client
        .get(&format!("{}/health/liveness", server.address))
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn health_check_readiness_works() {
    let server = TestServer::spawn(&[]).await;
    let client = Client::new();

    let response = client
        .get(&format!("{}/health/readiness", server.address))
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(200, response.status().as_u16());
}
