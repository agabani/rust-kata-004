pub struct Pid {
    path: String,
}

impl Pid {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn read(&self) -> Result<Option<u32>, std::io::Error> {
        if !std::path::Path::new(&self.path).exists() {
            return Ok(None);
        }

        let pid = std::fs::read_to_string(&self.path)?
            .parse::<u32>()
            .expect("Failed to parse PID.");
        Ok(Some(pid))
    }

    pub fn update(&self, pid: u32) -> Result<(), std::io::Error> {
        std::fs::write(&self.path, pid.to_string())?;
        Ok(())
    }

    pub fn reset(&self) -> Result<(), std::io::Error> {
        std::fs::write(&self.path, "0")?;
        Ok(())
    }
}

impl Drop for Pid {
    fn drop(&mut self) {
        if std::path::Path::new(&self.path).exists() {
            std::fs::remove_file(&self.path).expect("Failed to delete PID file.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn read_reads_none_if_file_does_not_exist() {
        // Arrange
        let path = format!("test-{}.pid", Faker.fake::<String>());
        let pid = Pid::new(&path);

        // Act
        let result = pid.read().expect("Failed to read pid.");

        // Assert
        assert_eq!(None, result);
    }

    #[test]
    fn read_reads_value_if_file_exist() {
        // Arrange
        let path = format!("test-{}.pid", Faker.fake::<String>());
        let value = Faker.fake::<u32>();
        std::fs::write(&path, value.to_string()).expect("Failed to create test file.");
        let pid = Pid::new(&path);

        // Act
        let result = pid.read().expect("Failed to read pid.");

        // Assert
        assert_eq!(Some(value), result);
    }

    #[test]
    fn update_creates_file_if_file_does_not_exist() {
        // Arrange
        let path = format!("test-{}.pid", Faker.fake::<String>());
        let value = Faker.fake::<u32>();
        let pid = Pid::new(&path);

        // Act
        pid.update(value).expect("Failed to update pid.");

        // Assert
        let actual = std::fs::read_to_string(&path)
            .expect("Failed to read test file.")
            .parse::<u32>()
            .expect("Failed to parse PID.");
        assert_eq!(value, actual);
    }

    #[test]
    fn update_updates_file_if_file_exist() {
        // Arrange
        let path = format!("test-{}.pid", Faker.fake::<String>());
        let value = Faker.fake::<u32>();
        std::fs::write(&path, Faker.fake::<u32>().to_string())
            .expect("Failed to create test file.");
        let pid = Pid::new(&path);

        // Act
        pid.update(value).expect("Failed to update pid.");

        // Assert
        let actual = std::fs::read_to_string(&path)
            .expect("Failed to read test file.")
            .parse::<u32>()
            .expect("Failed to parse PID.");
        assert_eq!(value, actual);
    }

    #[test]
    fn reset_creates_file_if_file_does_not_exist() {
        // Arrange
        let path = format!("test-{}.pid", Faker.fake::<String>());
        let pid = Pid::new(&path);

        // Act
        pid.reset().expect("Failed to reset pid.");

        // Assert
        let actual = std::fs::read_to_string(&path)
            .expect("Failed to read test file.")
            .parse::<u32>()
            .expect("Failed to parse PID.");
        assert_eq!(0, actual);
    }

    #[test]
    fn reset_updates_file_if_file_exist() {
        // Arrange
        let path = format!("test-{}.pid", Faker.fake::<String>());
        std::fs::write(&path, Faker.fake::<u32>().to_string())
            .expect("Failed to create test file.");
        let pid = Pid::new(&path);

        // Act
        pid.reset().expect("Failed to reset pid.");

        // Assert
        let actual = std::fs::read_to_string(&path)
            .expect("Failed to read test file.")
            .parse::<u32>()
            .expect("Failed to parse PID.");
        assert_eq!(0, actual);
    }

    #[test]
    fn drop_does_nothing_if_file_does_not_exist() {
        // Arrange
        let path = format!("test-{}.pid", Faker.fake::<String>());
        let pid = Pid::new(&path);

        // Act
        std::mem::drop(pid);
    }

    #[test]
    fn drop_deletes_file_if_file_exist() {
        // Arrange
        let path = format!("test-{}.pid", Faker.fake::<String>());
        let pid = Pid::new(&path);

        // Act
        std::mem::drop(pid);

        // Assert
        assert_eq!(false, std::path::Path::new(&path).exists())
    }
}
