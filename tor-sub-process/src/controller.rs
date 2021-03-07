use std::path::{Path, PathBuf};

use super::command::Command;
use super::configuration::Configuration;
use super::hidden_service_directory::HiddenServiceDirectory;
use super::scheduler::Scheduler;
use super::tor_rc::TorRc;
use crate::pid::Pid;
use crate::secret_file::SecretFile;
use crate::tor_rc::{TorRcConfiguration, TorRcHiddenServiceConfiguration};
use std::collections::HashMap;

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

    pub fn backup(&self, configuration: &Configuration) -> HashMap<String, Vec<SecretFile>> {
        configuration
            .hidden_services
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

    pub fn create_hidden_service(&mut self) {
        self.scheduler.reload();
    }

    pub fn delete_hidden_service(&mut self) {
        self.scheduler.reload();
    }
}
