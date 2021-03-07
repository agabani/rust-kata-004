use crate::command::Command;
use crate::event_loop;
use crate::pid::Pid;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::task::JoinHandle;

/// Represents a long running job lifecycle.
pub struct Scheduler {
    command: Command,
    handle: Option<JoinHandle<()>>,
    pid_path: PathBuf,
    reload: Arc<AtomicBool>,
    terminate: Arc<AtomicBool>,
}

impl Scheduler {
    pub fn new(command: Command, pid_path: PathBuf) -> Self {
        Self {
            command,
            handle: None,
            pid_path,
            reload: Arc::new(AtomicBool::new(false)),
            terminate: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Starts the scheduler event loop.
    pub fn start(&mut self) {
        let pid = Pid::new(PathBuf::from(&self.pid_path));

        if let Some(id) = pid.read().expect("Failed to read PID.") {
            panic!(
                "Failed to start scheduler. Potential zombie process found. This is caused by an unexpected shutdown. Please check PID `{}` is no longer running then delete 'job.pid'.",
                id
            )
        }

        let task = event_loop::event_loop(
            self.command.clone(),
            pid,
            self.reload.clone(),
            self.terminate.clone(),
        );
        let handle = tokio::spawn(task);
        self.handle = Some(handle);
    }

    /// Waits for scheduler to exit completely.
    pub async fn stop(&mut self) -> Result<(), std::io::Error> {
        if let Some(handle) = self.handle.take() {
            self.terminate.swap(true, Ordering::Relaxed);
            let _ = handle.await?;
            return Ok(());
        }

        unimplemented!();
    }

    /// Triggers a reload of the job.
    ///  * Unix: sends reload signal.
    ///  * Windows: recreates the job.
    pub fn reload(&mut self) {
        if self.handle.is_some() {
            self.reload.swap(true, Ordering::Relaxed);
            return;
        }

        unimplemented!();
    }
}
