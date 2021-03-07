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

    let mut configuration = Configuration {
        hidden_services: vec![
            HiddenService {
                service_name: "test_service_1".to_string(),
                service_port: 80,
                host_address: "127.0.0.1".to_string(),
                host_port: 8081,
            },
            HiddenService {
                service_name: "test_service_2".to_string(),
                service_port: 80,
                host_address: "127.0.0.1".to_string(),
                host_port: 8082,
            },
            HiddenService {
                service_name: "test_service_3".to_string(),
                service_port: 80,
                host_address: "127.0.0.1".to_string(),
                host_port: 8083,
            },
        ],
    };

    let (command, no_window_support) = create_command(false);
    let mut controller = Controller::new(command, working_directory, no_window_support);
    controller.update(&configuration);

    controller.start();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let result = line.split(" ").collect::<Vec<_>>();
        if let Some((&command, args)) = result.split_first() {
            match command {
                "shutdown" => {
                    controller.stop().await;
                    return Ok(());
                }
                "reload" => {
                    controller.create_hidden_service();
                }
                "create_service" => {
                    configuration.hidden_services.push(HiddenService {
                        service_name: args[0].to_string(),
                        service_port: str::parse::<u16>(args[1]).unwrap(),
                        host_address: args[2].to_string(),
                        host_port: str::parse::<u16>(args[3]).unwrap()
                    });

                    controller.update(&configuration);
                }
                "backup" => {
                    let files = controller
                        .backup(&configuration)
                        .iter()
                        .map(|(service, files)| {
                            let files = files
                                .iter()
                                .map(|file| format!("{}", file))
                                .collect::<Vec<_>>();
                            format!("service: {}, files: {:?}", service, files)
                        })
                        .collect::<Vec<_>>();
                    println!("{:?}", files);
                }
                result => {
                    println!("unknown: {}", result);
                    println!("command: {}", command);
                    println!("args: {:?}", args);
                }
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
