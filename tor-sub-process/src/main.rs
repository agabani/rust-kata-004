use signal_hook::{consts, flag};
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tor_sub_process::{Command, Configuration, Controller, HiddenService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Arc::new(AtomicBool::new(false));
    register_shutdown_signal(term.clone())?;

    let working_directory = PathBuf::from("/var/tmp/tor-sub-process");

    let configuration = Configuration {
        hidden_services: vec![HiddenService {
            service_directory: "/var/tmp/test_service".to_string(),
            service_port: 80,
            host_address: "127.0.0.1".to_string(),
            host_port: 8080,
        }],
    };

    let (command, no_window_support) = create_command(false);
    let mut controller = Controller::new(command, &working_directory, no_window_support);
    controller.update(&configuration);

    controller.start();
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
            "backup" => {
                let files = controller
                    .backup(&configuration)
                    .iter()
                    .map(|file| format!("{}", file))
                    .collect::<Vec<_>>();
                println!("{:?}", files);
            }
            result => {
                println!("unknown command: {}", result)
            }
        }
    }

    Ok(())
}

fn create_command<'a>(use_stub: bool) -> (&'a str, bool) {
    match (cfg!(target_family = "windows"), use_stub) {
        (false, false) => ("tor", false),
        (false, true) => ("./target/debug/tor-stub", false),
        (true, false) => ("./bin/tor/windows/tor.exe", true),
        (true, true) => ("./target/debug/tor-stub.exe", false),
    }
}

fn register_shutdown_signal(term: Arc<AtomicBool>) -> Result<(), std::io::Error> {
    for &term_signal in consts::TERM_SIGNALS {
        flag::register_conditional_shutdown(term_signal, 1, term.clone())?;
        flag::register(term_signal, term.clone())?;
    }
    Ok(())
}
