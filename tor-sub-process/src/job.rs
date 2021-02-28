use std::process::{ExitStatus, Output};
use tokio::process::{Child, Command};

/// Represents a child process as a single unit of work.
pub struct Job {
    command: Command,
    child: Option<Child>,
}

impl Job {
    pub fn new(command: Command) -> Self {
        Self {
            command,
            child: None,
        }
    }

    /// Starts the job as a child process.
    pub fn start(&mut self) -> Result<(), std::io::Error> {
        if self.child.is_none() {
            let child = self.command.spawn()?;
            self.child = Some(child);
            return Ok(());
        }

        unimplemented!();
    }

    /// Waits for the job to exit completely, returning the status that it exited with.
    pub async fn stop(&mut self) -> Result<Output, std::io::Error> {
        if let Some(child) = self.child.take() {
            if let Some(id) = child.id() {
                signal::sigterm(id);
            }
            let output = child.wait_with_output().await?;
            return Ok(output);
        }

        unimplemented!();
    }

    /// Attempts to collect the exit status of the job if it has already exited.
    pub async fn status(&mut self) -> Result<Option<ExitStatus>, std::io::Error> {
        if let Some(child) = self.child.as_mut() {
            return child.try_wait();
        }

        unimplemented!();
    }

    /// Sends a reload signal to the job.
    #[cfg(target_family = "unix")]
    pub fn reload(&self) {
        if let Some(child) = &self.child {
            if let Some(id) = child.id() {
                signal::sighup(id);
                return;
            }
        }

        unimplemented!();
    }

    /// Gets the process id of the running job.
    pub fn id(&self) -> Option<u32> {
        self.child.as_ref()?.id()
    }
}

/// Safe wrappers for libc interfaces
mod signal {
    #[cfg(target_family = "unix")]
    pub fn sighup(id: u32) {
        unsafe {
            libc::kill(id as i32, signal_hook::consts::signal::SIGHUP);
        }
    }

    #[cfg(target_family = "unix")]
    pub fn sigterm(id: u32) {
        unsafe {
            libc::kill(id as i32, signal_hook::consts::signal::SIGTERM);
        }
    }

    #[cfg(target_family = "windows")]
    pub fn sigterm(id: u32) {
        unsafe {
            libc::signal(signal_hook::consts::signal::SIGTERM, id as usize);
        }
    }
}

/// Windows platform swallows signals from `cargo test` making it difficult for `Job` to send
/// SIGTERM signals to child process.
#[cfg(target_family = "unix")]
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Stdio;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn it_can_send_signals() {
        // Arrange
        let mut job = create_job();

        // Act
        job.start().expect("Failed to start job.");
        sleep(Duration::from_millis(50)).await;
        job.reload();
        sleep(Duration::from_millis(20)).await;
        job.reload();
        sleep(Duration::from_millis(20)).await;
        let output = job.stop().await.expect("Failed to start job.");

        // Assert
        assert_eq!(Some(0), output.status.code());
        assert_eq!(
            r#"
Starting up.
Processing.
Reloading.
Processing.
Reloading.
Processing.
Shutting down.
"#,
            String::from_utf8_lossy(&output.stdout)
        )
    }

    #[tokio::test]
    async fn it_can_query_running_status() {
        // Arrange
        let mut job = create_job();

        // Act
        job.start().expect("Failed to start job.");
        sleep(Duration::from_millis(1000)).await;
        let running_status = job.status().await.expect("Failed to get status.");
        job.stop().await.expect("Failed to start job.");

        // Assert
        assert_eq!(None, running_status);
    }

    #[tokio::test]
    async fn it_can_query_stopped_success_status() {
        // Arrange
        let mut command = Command::new("echo");
        command.stderr(Stdio::null());
        let mut job = Job::new(Command::new("echo"));

        // Act
        job.start().expect("Failed to start job.");
        sleep(Duration::from_millis(1000)).await;
        let running_status = job.status().await.expect("Failed to get status.");
        job.stop().await.expect("Failed to start job.");

        // Assert
        assert_eq!(0, running_status.unwrap().code().unwrap());
    }

    #[tokio::test]
    async fn it_can_query_stopped_failed_status() {
        // Arrange
        let mut command = Command::new("sh");
        command.arg("_echo_").stderr(Stdio::null());
        let mut job = Job::new(command);

        // Act
        job.start().expect("Failed to start job.");
        sleep(Duration::from_millis(1000)).await;
        let running_status = job.status().await.expect("Failed to get status.");
        job.stop().await.expect("Failed to start job.");

        // Assert
        assert_eq!(127, running_status.unwrap().code().unwrap());
    }

    fn create_job() -> Job {
        let path = std::env::current_dir()
            .unwrap()
            .join("../target/debug/tor-stub");

        if !std::path::Path::new(&path).exists() {
            panic!("tor-stub does not exist. Please run cargo build --workspace then try again.");
        }
        let mut command = Command::new(&path);
        command.arg("--no-wait").stdout(Stdio::piped());
        Job::new(command)
    }
}
