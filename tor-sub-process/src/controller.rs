use std::path::PathBuf;

use super::command::Command;
use super::configuration::Configuration;
use super::scheduler::Scheduler;
use super::tor_rc::TorRc;

/// Interface with server
pub struct Controller {
    scheduler: Scheduler,
    tor_rc: TorRc,
}

impl Controller {
    pub fn new(command: Command) -> Self {
        Self {
            scheduler: Scheduler::new(command, PathBuf::from("tor.pid")),
            tor_rc: TorRc::new(PathBuf::from("torrc")),
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
