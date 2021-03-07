use std::path::{Path, PathBuf};

use super::command::Command;
use super::configuration::Configuration;
use super::hidden_service_directory::HiddenServiceDirectory;
use super::scheduler::Scheduler;
use super::tor_rc::TorRc;
use crate::pid::Pid;
use crate::secret_file::SecretFile;

/// Interface with server
pub struct Controller {
    scheduler: Scheduler,
    tor_rc: TorRc,
    hidden_service_directory: HiddenServiceDirectory,
}

impl Controller {
    pub fn new(program: &str, working_directory: &Path, no_window_support: bool) -> Self {
        let pid_path = working_directory.join("tor.pid");
        let tor_rc_path = working_directory.join("torrc");
        let hidden_service_dir = working_directory;

        let command = Command::new(program, tor_rc_path.to_str().unwrap(), no_window_support);

        Self {
            scheduler: Scheduler::new(command, pid_path),
            tor_rc: TorRc::new(tor_rc_path),
            hidden_service_directory: HiddenServiceDirectory::new(hidden_service_dir.to_path_buf()),
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

    pub fn backup(&self, configuration: &Configuration) -> Vec<SecretFile> {
        //self.hidden_service_directory.get_secret_files()
        vec![]
    }

    pub fn create_hidden_service(&mut self) {
        self.scheduler.reload();
    }

    pub fn delete_hidden_service(&mut self) {
        self.scheduler.reload();
    }
}
