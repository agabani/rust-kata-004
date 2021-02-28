use crate::command::Command;
use crate::scheduler::Scheduler;

/// Interface with server
pub struct Controller {
    scheduler: Scheduler,
}

impl Controller {
    pub fn new(command: Command) -> Self {
        Self {
            scheduler: Scheduler::new(command, "tor.pid"),
        }
    }

    pub fn start(&mut self) {
        let _ = self.scheduler.start();
    }

    pub async fn stop(&mut self) {
        let _ = self.scheduler.stop().await;
    }

    pub fn create_hidden_service(&mut self) {
        self.scheduler.reload();
    }

    pub fn delete_hidden_service(&mut self) {
        self.scheduler.reload();
    }
}
