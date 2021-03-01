use crate::Configuration;
use std::fs::File;
use std::io::Write;

pub struct TorRc {
    path: String,
}

impl TorRc {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn save(&self, configuration: &Configuration) {
        let mut file = File::create(&self.path).expect("Failed to open file.");

        let hidden_services = configuration
            .hidden_services
            .iter()
            .map(|hidden_service| {
                format!(
                    r#"HiddenServiceDir {}
HiddenServicePort {} {}:{}"#,
                    hidden_service.service_directory,
                    hidden_service.service_port,
                    hidden_service.host_address,
                    hidden_service.host_port
                )
            })
            .collect::<Vec<String>>();

        for hidden_service in configuration.hidden_services.iter() {
            std::fs::create_dir(&hidden_service.service_directory)
                .expect("Failed to create directory.");
        }

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

        let path = format!("test-{}.pid", Faker.fake::<String>());

        let value = Configuration {
            hidden_services: vec![HiddenService {
                service_directory: service_directory.clone(),
                service_port,
                host_address: host_address.clone(),
                host_port,
            }],
        };
        let tor_rc = TorRc::new(&path);

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
        let path = format!("test-{}.torrc", Faker.fake::<String>());
        let tor_rc = TorRc::new(&path);

        // Act
        std::mem::drop(tor_rc);
    }

    #[test]
    fn drop_deletes_file_if_file_exist() {
        // Arrange
        let path = format!("test-{}.torrc", Faker.fake::<String>());
        let tor_rc = TorRc::new(&path);

        // Act
        std::mem::drop(tor_rc);

        // Assert
        assert_eq!(false, std::path::Path::new(&path).exists())
    }
}
