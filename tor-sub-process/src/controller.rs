use std::path::PathBuf;

use super::command::Command;
use super::configuration::Configuration;
use super::hidden_service_directory::HiddenServiceDirectory;
use super::scheduler::Scheduler;
use super::tor_rc::TorRc;

/// Interface with server
pub struct Controller {
    scheduler: Scheduler,
    tor_rc: TorRc,
    hidden_service_directory: HiddenServiceDirectory,
}

impl Controller {
    pub fn new(
        command: Command,
        pid: PathBuf,
        tor_rc: PathBuf,
        hidden_service_dir: PathBuf,
    ) -> Self {
        Self {
            scheduler: Scheduler::new(command, pid),
            tor_rc: TorRc::new(tor_rc),
            hidden_service_directory: HiddenServiceDirectory::new(hidden_service_dir),
        }
    }

    pub fn start(&mut self) {
        let _ = self.scheduler.start();
    }

    pub async fn stop(&mut self) {
        let _ = self.scheduler.stop().await;
    }

    pub fn update(&self, configuration: &Configuration) {
        self.tor_rc.save(configuration);
    }

    pub fn create_hidden_service(&mut self) {
        self.scheduler.reload();
    }

    pub fn delete_hidden_service(&mut self) {
        self.scheduler.reload();
    }
}
