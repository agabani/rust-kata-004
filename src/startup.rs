use crate::configuration::Configuration;
use crate::kubernetes::Manager;
use crate::routes::{health_liveness, health_readiness};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::future::Future;
use std::pin::Pin;
use tracing_actix_web::TracingLogger;

pub async fn run(
    overrides: &[(&str, &str)],
) -> (Server, u16, Pin<Box<dyn Future<Output = ()> + Send>>) {
    let configuration = Configuration::load(overrides).expect("Failed to read configuration.");

    let listener = configuration
        .http_server
        .tcp_listener()
        .expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();

    let client = kube::Client::try_default()
        .await
        .expect("Failed to create client.");
    let (manager, drainer) = Manager::new(client).await;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .service(
                web::scope("/health")
                    .route("/liveness", web::get().to(health_liveness))
                    .route("/readiness", web::to(health_readiness)),
            )
            .data(manager.clone())
    })
    .listen(listener)
    .expect("Failed to bind address.")
    .run();

    (server, port, drainer)
}
