use signal_hook::{consts, flag};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tor_sub_process::{Command, Controller};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Arc::new(AtomicBool::new(false));
    let reload = Arc::new(AtomicBool::new(false));
    register_shutdown_signal(term.clone())?;
    register_reload_signal(reload.clone())?;

    event_loop(term, reload).await?;

    Ok(())
}

async fn event_loop(term: Arc<AtomicBool>, reload: Arc<AtomicBool>) -> Result<(), std::io::Error> {
    let mut server = Controller::new(create_command(true));

    server.start();

    loop {
        if term.load(Ordering::Relaxed) {
            server.stop().await;
            return Ok(());
        }

        if reload.swap(false, Ordering::Relaxed) {
            server.create_hidden_service();
        }

        sleep(Duration::from_millis(1000)).await;
    }
}

fn register_shutdown_signal(term: Arc<AtomicBool>) -> Result<(), std::io::Error> {
    for &term_signal in consts::TERM_SIGNALS {
        flag::register_conditional_shutdown(term_signal, 1, term.clone())?;
        flag::register(term_signal, term.clone())?;
    }
    Ok(())
}

#[cfg(target_family = "unix")]
fn register_reload_signal(reload: Arc<AtomicBool>) -> Result<(), std::io::Error> {
    flag::register(consts::SIGHUP, reload)?;
    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
#[cfg(target_family = "windows")]
fn register_reload_signal(_reload: Arc<AtomicBool>) -> Result<(), std::io::Error> {
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
