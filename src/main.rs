use rust_kata_004::{run, telemetry};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init(telemetry::configure("info"));

    let (server, _, drainer) = run(&[]).await;

    tokio::select! {
        _ = drainer => {
            println!("kubernetes controller drained.")
        },
        _ = server => {
            println!("actix-web server exited.")
        },
    }

    Ok(())
}
