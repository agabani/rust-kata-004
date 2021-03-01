use crate::command::Command;
use crate::configuration::{Configuration, HiddenService};
use crate::scheduler::Scheduler;
use crate::tor_rc::TorRc;

/// Interface with server
pub struct Controller {
    scheduler: Scheduler,
    tor_rc: TorRc,
}

impl Controller {
    pub fn new(command: Command) -> Self {
        Self {
            scheduler: Scheduler::new(command, "tor.pid"),
            tor_rc: TorRc::new("torrc"),
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
