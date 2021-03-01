use signal_hook::{consts, flag};
use std::io::{BufRead, Read};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tor_sub_process::{Command, Configuration, Controller, HiddenService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Arc::new(AtomicBool::new(false));
    register_shutdown_signal(term.clone())?;

    let mut configuration = Configuration {
        hidden_services: vec![HiddenService {
            service_directory: "test_service".to_string(),
            service_port: 80,
            host_address: "127.0.0.1".to_string(),
            host_port: 8080,
        }],
    };

    let command = create_command(false);
    let mut controller = Controller::new(command);

    //controller.start();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let result = line.unwrap();
        match result.as_str() {
            "shutdown" => {
                controller.stop().await;
                return Ok(());
            }
            "reload" => {
                controller.create_hidden_service();
            }
            "create_service" => {
                controller.update(&configuration);
            }
            result => {
                println!("unknown command: {}", result)
            }
        }
    }

    Ok(())
}

fn create_command(use_stub: bool) -> Command {
    match (cfg!(target_family = "windows"), use_stub) {
        (false, false) => Command::new("tor", false),
        (false, true) => Command::new("./target/debug/tor-stub", false),
        (true, false) => Command::new("./bin/tor/windows/tor.exe", true),
        (true, true) => Command::new("./target/debug/tor-stub.exe", false),
    }
}

fn register_shutdown_signal(term: Arc<AtomicBool>) -> Result<(), std::io::Error> {
    for &term_signal in consts::TERM_SIGNALS {
        flag::register_conditional_shutdown(term_signal, 1, term.clone())?;
        flag::register(term_signal, term.clone())?;
    }
    Ok(())
}
