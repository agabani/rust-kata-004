use signal_hook::{consts, flag};
use std::io::BufRead;
use std::path::{PathBuf, Path};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tor_sub_process::{Command, Configuration, Controller, HiddenService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Arc::new(AtomicBool::new(false));
    register_shutdown_signal(term.clone())?;

    let configuration = Configuration {
        hidden_services: vec![HiddenService {
            service_directory: "/var/tmp/test_service".to_string(),
            service_port: 80,
            host_address: "127.0.0.1".to_string(),
            host_port: 8080,
        }],
    };

    let base_path = PathBuf::from("/var/tmp/tor-sub-process");
    let pid = base_path.join("tor.pid");
    let tor_rc = base_path.join("torrc");
    let hidden_service_dir = base_path;
    let command = create_command(false, &tor_rc);
    let mut controller = Controller::new(
        command,
        pid,
        tor_rc,
        hidden_service_dir,
    );
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
            result => {
                println!("unknown command: {}", result)
            }
        }
    }

    Ok(())
}

fn create_command(use_stub: bool, tor_rc: &Path) -> Command {
    let tor_rc = tor_rc.to_str().unwrap();
    match (cfg!(target_family = "windows"), use_stub) {
        (false, false) => Command::new("tor", tor_rc, false),
        (false, true) => Command::new("./target/debug/tor-stub", tor_rc, false),
        (true, false) => Command::new("./bin/tor/windows/tor.exe", tor_rc, true),
        (true, true) => Command::new("./target/debug/tor-stub.exe", tor_rc, false),
    }
}

fn register_shutdown_signal(term: Arc<AtomicBool>) -> Result<(), std::io::Error> {
    for &term_signal in consts::TERM_SIGNALS {
        flag::register_conditional_shutdown(term_signal, 1, term.clone())?;
        flag::register(term_signal, term.clone())?;
    }
    Ok(())
}
