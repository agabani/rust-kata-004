use crate::Configuration;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct TorRc {
    path: PathBuf,
}

pub struct TorRcConfiguration {
    pub hidden_services: Vec<TorRcHiddenServiceConfiguration>,
}

pub struct TorRcHiddenServiceConfiguration {
    pub service_directory: PathBuf,
    pub service_port: u16,
    pub host_address: String,
    pub host_port: u16,
}

impl TorRc {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn save(&self, configuration: &TorRcConfiguration) {
        let mut file = File::create(&self.path).expect("Failed to open file.");

        let hidden_services = configuration
            .hidden_services
            .iter()
            .map(|hidden_service| {
                format!(
                    r#"HiddenServiceDir {}
HiddenServicePort {} {}:{}"#,
                    hidden_service.service_directory.to_str().unwrap(),
                    hidden_service.service_port,
                    hidden_service.host_address,
                    hidden_service.host_port
                )
            })
            .collect::<Vec<String>>();

        let contents = hidden_services.join("\n");
        file.write(contents.as_bytes())
            .expect("Failed to write to file.");
    }
}

impl Drop for TorRc {
    fn drop(&mut self) {
        if std::path::Path::new(&self.path).exists() {
            std::fs::remove_file(&self.path).expect("Failed to delete TorRc file.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HiddenService;
    use fake::{Fake, Faker};

    #[test]
    fn update_creates_file_if_file_does_not_exist() {
        // Arrange
        let service_directory = Faker.fake::<String>();
        let service_port = Faker.fake::<u16>();
        let host_address = Faker.fake::<String>();
        let host_port = Faker.fake::<u16>();

        let path = path();

        let value = TorRcConfiguration {
            hidden_services: vec![TorRcHiddenServiceConfiguration {
                service_directory: PathBuf::from(service_directory),
                service_port,
                host_address: host_address.clone(),
                host_port,
            }],
        };
        let tor_rc = TorRc::new(PathBuf::from(&path));

        // Act
        tor_rc.save(&value);

        // Assert
        let actual = std::fs::read_to_string(&path).expect("Failed to read test file.");
        assert_eq!(
            format!(
                r#"HiddenServiceDir {}
HiddenServicePort {} {}:{}"#,
                &service_directory, &service_port, &host_address, &host_port
            ),
            actual
        );
    }

    #[test]
    fn drop_does_nothing_if_file_does_not_exist() {
        // Arrange
        let path = path();
        let tor_rc = TorRc::new(PathBuf::from(path));

        // Act
        std::mem::drop(tor_rc);
    }

    #[test]
    fn drop_deletes_file_if_file_exist() {
        // Arrange
        let path = path();
        let tor_rc = TorRc::new(PathBuf::from(&path));

        // Act
        std::mem::drop(tor_rc);

        // Assert
        assert_eq!(false, std::path::Path::new(&path).exists())
    }

    fn path() -> String {
        if cfg!(target_family = "unix") {
            format!("/var/tmp/test-{}.torrc", Faker.fake::<String>())
        } else {
            format!("test-{}.torrc", Faker.fake::<String>())
        }
    }
}
