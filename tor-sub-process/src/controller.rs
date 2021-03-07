use std::collections::HashMap;
use std::path::PathBuf;

use super::command::Command;
use super::configuration::Configuration;
use super::hidden_service_directory::HiddenServiceDirectory;
use super::scheduler::Scheduler;
use super::tor_rc::TorRc;
use crate::secret_file::SecretFile;
use crate::tor_rc::{TorRcConfiguration, TorRcHiddenServiceConfiguration};
use crate::HiddenService;

/// Interface with server
pub struct Controller {
    scheduler: Scheduler,
    tor_rc: TorRc,
    hidden_service_directory: HiddenServiceDirectory,
    working_directory: PathBuf,
}

impl Controller {
    pub fn new(program: &str, working_directory: PathBuf, no_window_support: bool) -> Self {
        let pid_path = working_directory.join("tor.pid");
        let tor_rc_path = working_directory.join("torrc");
        let hidden_service_dir = working_directory.clone();

        let command = Command::new(program, tor_rc_path.to_str().unwrap(), no_window_support);

        Self {
            scheduler: Scheduler::new(command, pid_path),
            tor_rc: TorRc::new(tor_rc_path),
            hidden_service_directory: HiddenServiceDirectory::new(hidden_service_dir.to_path_buf()),
            working_directory,
        }
    }

    pub fn start(&mut self) {
        let _ = self.scheduler.start();
    }

    pub async fn stop(&mut self) {
        let _ = self.scheduler.stop().await;
    }

    pub fn update(&mut self, configuration: &Configuration) {
        let tor_rc_configuration = TorRcConfiguration {
            hidden_services: configuration
                .hidden_services
                .iter()
                .map(|hidden_service| TorRcHiddenServiceConfiguration {
                    service_directory: self.working_directory.join(&hidden_service.service_name),
                    service_port: hidden_service.service_port,
                    host_address: hidden_service.host_address.clone(),
                    host_port: hidden_service.host_port,
                })
                .collect(),
        };

        self.tor_rc.save(&tor_rc_configuration);
        self.scheduler.reload();
    }

    pub fn backup(&self, hidden_services: &[HiddenService]) -> HashMap<String, Vec<SecretFile>> {
        hidden_services
            .iter()
            .map(|hidden_service| {
                let files = self
                    .hidden_service_directory
                    .get_secret_files(&hidden_service.service_name);
                let service_name = hidden_service.service_name.clone();
                (service_name, files)
            })
            .collect()
    }

    pub fn restore(&self, hidden_service: &HiddenService, secret_files: &[SecretFile]) {
        self.hidden_service_directory
            .save_secret_files(&hidden_service.service_name, secret_files);
    }

    pub fn create_hidden_service(&mut self) {
        self.scheduler.reload();
    }

    pub fn delete_hidden_service(&mut self) {
        self.scheduler.reload();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test() {
        // Arrange
        let mut controller = controller();

        // Act
        controller.start();
        sleep(Duration::from_millis(1000)).await;
        controller.stop().await;
        sleep(Duration::from_millis(1000)).await;
        controller.start();
        sleep(Duration::from_millis(1000)).await;
        controller.stop().await;
        sleep(Duration::from_millis(1000)).await;

        // Assert
        /* TODO: resolve bug
         *      Starting a controller after being stopped once does not spawn sub process.
         *      Update tor-stub to output events to file so integration tests can Assert.
         *      "cargo test --package tor-sub-process controller -- --nocapture"
         */
    }

    fn controller() -> Controller {
        let path = std::path::PathBuf::from("/var/tmp/test-tor-sub-process-integration");

        let controller = Controller::new("../target/debug/tor-stub --no-wait", path, false);

        controller
    }
}
