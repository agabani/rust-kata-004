use crate::configuration::Configuration;
use crate::routes::{health_liveness, health_readiness};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

pub async fn run(overrides: &[(&str, &str)]) -> (Server, u16) {
    let configuration = Configuration::load(overrides).expect("Failed to read configuration.");

    let listener = configuration
        .http_server
        .tcp_listener()
        .expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || {
        App::new().service(
            web::scope("/health")
                .route("/liveness", web::get().to(health_liveness))
                .route("/readiness", web::to(health_readiness)),
        )
    })
    .listen(listener)
    .expect("Failed to bind address.")
    .run();

    (server, port)
}
