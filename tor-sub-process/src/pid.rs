use std::fs;
use std::io;
use std::path::PathBuf;

pub struct Pid {
    path: PathBuf,
}

impl Pid {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn read(&self) -> Result<Option<u32>, io::Error> {
        if !self.path.exists() {
            return Ok(None);
        }

        let pid = fs::read_to_string(&self.path)?
            .parse::<u32>()
            .expect("Failed to parse PID.");
        Ok(Some(pid))
    }

    pub fn update(&self, pid: u32) -> Result<(), io::Error> {
        fs::write(&self.path, pid.to_string())?;
        Ok(())
    }

    pub fn reset(&self) -> Result<(), io::Error> {
        fs::write(&self.path, "0")?;
        Ok(())
    }
}

impl Drop for Pid {
    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_file(&self.path).expect("Failed to delete PID file.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};
    use std::mem;

    #[test]
    fn read_reads_none_if_file_does_not_exist() {
        // Arrange
        let path = path();
        let pid = Pid::new(PathBuf::from(&path));

        // Act
        let result = pid.read().expect("Failed to read pid.");

        // Assert
        assert_eq!(None, result);
    }

    #[test]
    fn read_reads_value_if_file_exist() {
        // Arrange
        let path = path();
        let value = Faker.fake::<u32>();
        fs::write(&path, value.to_string()).expect("Failed to create test file.");
        let pid = Pid::new(PathBuf::from(&path));

        // Act
        let result = pid.read().expect("Failed to read pid.");

        // Assert
        assert_eq!(Some(value), result);
    }

    #[test]
    fn update_creates_file_if_file_does_not_exist() {
        // Arrange
        let path = path();
        let value = Faker.fake::<u32>();
        let pid = Pid::new(PathBuf::from(&path));

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
        let path = path();
        let value = Faker.fake::<u32>();
        fs::write(&path, Faker.fake::<u32>().to_string()).expect("Failed to create test file.");
        let pid = Pid::new(PathBuf::from(&path));

        // Act
        pid.update(value).expect("Failed to update pid.");

        // Assert
        let actual = fs::read_to_string(&path)
            .expect("Failed to read test file.")
            .parse::<u32>()
            .expect("Failed to parse PID.");
        assert_eq!(value, actual);
    }

    #[test]
    fn reset_creates_file_if_file_does_not_exist() {
        // Arrange
        let path = path();
        let pid = Pid::new(PathBuf::from(&path));

        // Act
        pid.reset().expect("Failed to reset pid.");

        // Assert
        let actual = fs::read_to_string(&path)
            .expect("Failed to read test file.")
            .parse::<u32>()
            .expect("Failed to parse PID.");
        assert_eq!(0, actual);
    }

    #[test]
    fn reset_updates_file_if_file_exist() {
        // Arrange
        let path = path();
        fs::write(&path, Faker.fake::<u32>().to_string()).expect("Failed to create test file.");
        let pid = Pid::new(PathBuf::from(&path));

        // Act
        pid.reset().expect("Failed to reset pid.");

        // Assert
        let actual = fs::read_to_string(&path)
            .expect("Failed to read test file.")
            .parse::<u32>()
            .expect("Failed to parse PID.");
        assert_eq!(0, actual);
    }

    #[test]
    fn drop_does_nothing_if_file_does_not_exist() {
        // Arrange
        let path = path();
        let pid = Pid::new(PathBuf::from(&path));

        // Act
        mem::drop(pid);
    }

    #[test]
    fn drop_deletes_file_if_file_exist() {
        // Arrange
        let path = path();
        let pid = Pid::new(PathBuf::from(&path));

        // Act
        mem::drop(pid);

        // Assert
        assert_eq!(false, path.exists())
    }

    fn path() -> PathBuf {
        PathBuf::from(format!("test-{}.pid", Faker.fake::<String>()))
    }
}
