use signal_hook::{consts, flag};
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Arc::new(AtomicBool::new(false));
    let reload = Arc::new(AtomicBool::new(false));
    register_shutdown_signal(term.clone())?;
    register_reload_signal(reload.clone())?;

    event_loop(term, reload).await;

    println!();

    Ok(())
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

async fn event_loop(term: Arc<AtomicBool>, reload: Arc<AtomicBool>) {
    let state = Arc::new(AtomicU32::new(0));

    startup_handler().await;

    loop {
        match term.load(Ordering::Relaxed) {
            true => {
                shutdown_handler().await;
                return;
            }
            false => {}
        }

        match reload.load(Ordering::Relaxed) {
            true => {
                reload_handler().await;
                reload.swap(false, Ordering::SeqCst);
                state.swap(0, Ordering::SeqCst);
            }
            false => {}
        }

        state.swap(
            process(state.load(Ordering::Relaxed)).await,
            Ordering::Relaxed,
        );
        sleep(Duration::from_millis(10)).await;
    }
}

async fn reload_handler() {
    println!();
    print!("Reloading.");
    std::io::stdout().flush().unwrap();
    wait().await;
}

async fn shutdown_handler() {
    println!();
    print!("Shutting down.");
    std::io::stdout().flush().unwrap();
    wait().await;
}

async fn startup_handler() {
    println!();
    print!("Starting up.");
    std::io::stdout().flush().unwrap();
    wait().await;
}

async fn process(state: u32) -> u32 {
    match state {
        0 => {
            println!();
            print!("Processing.");
            std::io::stdout().flush().unwrap();
        }
        100 => {
            if !is_no_wait() {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
        }
        200 => {
            if !is_no_wait() {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
        }
        300 => {
            if !is_no_wait() {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
        }
        400 => {
            if !is_no_wait() {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
        }
        _ => {}
    }

    std::io::stdout().flush().unwrap();

    if is_no_wait() {
        1
    } else {
        match state {
            450 => 0,
            i => i + 1,
        }
    }
}

async fn wait() {
    if is_no_wait() {
        return;
    }

    for _ in 0..2 {
        sleep(Duration::from_secs(1)).await;
        print!(".");
        std::io::stdout().flush().unwrap();
    }
}

fn is_no_wait() -> bool {
    if let Some(arg) = std::env::args().nth(1) {
        if &arg == "--no-wait" {
            return true;
        }
    }
    false
}
