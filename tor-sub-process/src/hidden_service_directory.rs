use std::fs;
use std::path::PathBuf;

use super::libc_wrapper::{set_mode_600, set_mode_700};
use super::secret_file::SecretFile;

/// Interface for reading and writing files to hidden service directory.
pub struct HiddenServiceDirectory {
    base_path: PathBuf,
}

impl HiddenServiceDirectory {
    /// Creates new `HiddenServiceDirectory` interface.
    pub fn new(path: PathBuf) -> Self {
        Self { base_path: path }
    }

    /// Gets all secret files from directory.
    ///
    /// TODO: Read files from authorized_clients folder. Not needed for current implementation.
    pub fn get_secret_files(&self, service_name: &str) -> Vec<SecretFile> {
        let service_path = self.base_path.join(service_name);

        let paths = fs::read_dir(&service_path)
            .unwrap()
            .map(|parent| parent.map(|child| child.path()).unwrap());

        let file_paths = paths.filter(|path| path.is_file());

        let files = file_paths.map(|file| {
            SecretFile::from(
                PathBuf::from(file.file_name().unwrap()),
                fs::read(file).unwrap(),
            )
            .unwrap()
        });

        files.collect::<Vec<_>>()
    }

    /// Saves all secret files to directory and sets file permissions.
    pub fn save_secret_files(&self, service_name: &str, secret_files: &[SecretFile]) {
        let service_path = self.base_path.join(service_name);

        /// `authorized_clients` folder is created by tor executable, we create this just in case.
        let authorized_clients = &service_path.join("authorized_clients");
        fs::create_dir_all(authorized_clients).unwrap();
        set_mode_700(authorized_clients.to_str().unwrap()).unwrap();

        for secret_file in secret_files {
            let path = &service_path.join(secret_file.relative_path());
            fs::write(path, secret_file.contents()).unwrap();
            set_mode_600(path.to_str().unwrap()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::libc_wrapper::{get_mode, set_mode_600, set_mode_700};
    use fake::{Fake, Faker};
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    pub fn get_secret_files() {
        // Arrange
        let working_directory = TestDirectory::new();
        let service_name: String = Faker.fake();
        let service_directory = working_directory.create_directory(&service_name);

        let filename_1: String = Faker.fake();
        let contents_1: Vec<u8> = Faker.fake();
        let filename_2: String = Faker.fake();
        let contents_2: Vec<u8> = Faker.fake();
        let filename_3: String = Faker.fake();
        let contents_3: Vec<u8> = Faker.fake();
        let sub_directory: String = Faker.fake();
        service_directory.create_file(&filename_1, &contents_1);
        service_directory.create_file(&filename_2, &contents_2);
        service_directory.create_file(&filename_3, &contents_3);
        let _ = service_directory.create_directory(&sub_directory);

        let directory = HiddenServiceDirectory::new(working_directory.path().to_path_buf());

        // Act
        let files = directory.get_secret_files(&service_name);

        // Assert
        assert_eq!(3, files.len());

        let lookup = files
            .iter()
            .map(|file| (file.relative_path().to_str().unwrap(), file))
            .collect::<HashMap<&str, &SecretFile>>();

        let file_1 = lookup.get(filename_1.as_str()).unwrap();
        assert_eq!(filename_1, file_1.relative_path().to_str().unwrap());
        assert_eq!(contents_1, file_1.contents());
        let file_2 = lookup.get(filename_2.as_str()).unwrap();
        assert_eq!(filename_2, file_2.relative_path().to_str().unwrap());
        assert_eq!(contents_2, file_2.contents());
        let file_3 = lookup.get(filename_3.as_str()).unwrap();
        assert_eq!(filename_3, file_3.relative_path().to_str().unwrap());
        assert_eq!(contents_3, file_3.contents());
    }

    #[test]
    pub fn save_secret_files() {
        // Arrange
        let working_directory = TestDirectory::new();
        let service_name: String = Faker.fake();
        let service_directory = working_directory.create_directory(&service_name);
        let filename_1: String = Faker.fake();
        let contents_1: Vec<u8> = Faker.fake();
        let filename_2: String = Faker.fake();
        let contents_2: Vec<u8> = Faker.fake();
        let filename_3: String = Faker.fake();
        let contents_3: Vec<u8> = Faker.fake();

        let directory = HiddenServiceDirectory::new(working_directory.path().to_path_buf());

        // Act
        directory.save_secret_files(
            &service_name,
            &vec![
                SecretFile::from(PathBuf::from(&filename_1), contents_1.clone()).unwrap(),
                SecretFile::from(PathBuf::from(&filename_2), contents_2.clone()).unwrap(),
                SecretFile::from(PathBuf::from(&filename_3), contents_3.clone()).unwrap(),
            ],
        );

        // Assert
        let (file_mode, directory_mode) = if cfg!(target_family = "unix") {
            (0o600, 0o700)
        } else {
            (0o666, 0o777)
        };

        let (file_1, mode_1) = service_directory.read_file(filename_1.as_str());
        assert_eq!(contents_1, file_1);
        assert_eq!(file_mode, mode_1);
        let (file_2, mode_2) = service_directory.read_file(filename_2.as_str());
        assert_eq!(contents_2, file_2);
        assert_eq!(file_mode, mode_2);
        let (file_3, mode_3) = service_directory.read_file(filename_3.as_str());
        assert_eq!(contents_3, file_3);
        assert_eq!(file_mode, mode_3);
        let mode_4 = service_directory.read_directory("authorized_clients");
        assert_eq!(directory_mode, mode_4);
    }

    pub struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new() -> Self {
            let path = if cfg!(target_family = "unix") {
                format!("/var/tmp/test-{}", Faker.fake::<String>())
            } else {
                format!("test-{}", Faker.fake::<String>())
            };
            let path = PathBuf::from(path);
            Self { path }
        }

        fn path(&self) -> &Path {
            self.init();
            self.path.as_path()
        }

        fn create_file(&self, filename: &str, contents: &[u8]) {
            self.init();
            let path = &self.path.join(filename);
            fs::write(path, contents).unwrap();
            set_mode_600(path.to_str().unwrap()).unwrap();
        }

        fn create_directory(&self, path: &str) -> Self {
            self.init();
            let directory = &self.path.join(path);
            fs::create_dir_all(&directory).unwrap();
            set_mode_700(&directory.to_str().unwrap()).unwrap();
            Self {
                path: self.path.join(path),
            }
        }

        fn read_file(&self, filename: &str) -> (Vec<u8>, u16) {
            let path = &self.path.join(filename);
            let contents = fs::read(path).unwrap();
            let mode = get_mode(path.to_str().unwrap()).unwrap();
            (contents, mode)
        }

        fn read_directory(&self, path: &str) -> u16 {
            let path = &self.path.join(path);
            let mode = get_mode(path.to_str().unwrap()).unwrap();
            mode
        }

        fn init(&self) {
            fs::create_dir_all(&self.path).unwrap();
            set_mode_700(&self.path.to_str().unwrap()).unwrap();
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.path).unwrap();
        }
    }
}
