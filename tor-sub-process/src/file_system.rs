use std::path::PathBuf;

use crate::libc_wrapper::set_mode_600;
use crate::secret_file::SecretFile;

pub struct HiddenServiceDirectory {
    base_path: PathBuf,
}

impl HiddenServiceDirectory {
    pub fn new(path: PathBuf) -> Self {
        Self { base_path: path }
    }

    pub fn get_secret_files(&self) -> Vec<SecretFile> {
        let paths = std::fs::read_dir(&self.base_path)
            .unwrap()
            .map(|parent| parent.map(|child| child.path()).unwrap());

        let file_paths = paths.filter(|path| path.is_file());

        let files = file_paths.map(|file| {
            SecretFile::from(
                PathBuf::from(file.file_name().unwrap()),
                std::fs::read(file).unwrap(),
            )
            .unwrap()
        });

        files.collect::<Vec<_>>()
    }

    pub fn save_secret_files(&self, secret_files: &[SecretFile]) {
        for secret_file in secret_files {
            let path = &self.base_path.join(secret_file.relative_path());
            std::fs::write(path, secret_file.contents()).unwrap();
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
        let test_directory = TestDirectory::new();
        let filename_1: String = Faker.fake();
        let contents_1: Vec<u8> = Faker.fake();
        let filename_2: String = Faker.fake();
        let contents_2: Vec<u8> = Faker.fake();
        let filename_3: String = Faker.fake();
        let contents_3: Vec<u8> = Faker.fake();
        test_directory.create_file(&filename_1, &contents_1);
        test_directory.create_file(&filename_2, &contents_2);
        test_directory.create_file(&filename_3, &contents_3);

        let directory = HiddenServiceDirectory::new(test_directory.path().to_path_buf());

        // Act
        let files = directory.get_secret_files();

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
        let test_directory = TestDirectory::new();
        let filename_1: String = Faker.fake();
        let contents_1: Vec<u8> = Faker.fake();
        let filename_2: String = Faker.fake();
        let contents_2: Vec<u8> = Faker.fake();
        let filename_3: String = Faker.fake();
        let contents_3: Vec<u8> = Faker.fake();

        let directory = HiddenServiceDirectory::new(test_directory.path().to_path_buf());

        // Act
        directory.save_secret_files(&vec![
            SecretFile::from(PathBuf::from(&filename_1), contents_1.clone()).unwrap(),
            SecretFile::from(PathBuf::from(&filename_2), contents_2.clone()).unwrap(),
            SecretFile::from(PathBuf::from(&filename_3), contents_3.clone()).unwrap(),
        ]);

        // Assert
        let mode = if cfg!(target_family = "unix") {
            0o600
        } else {
            0o666
        };

        let (file_1, mode_1) = test_directory.read_file(filename_1.as_str());
        assert_eq!(contents_1, file_1);
        assert_eq!(mode, mode_1);
        let (file_2, mode_2) = test_directory.read_file(filename_2.as_str());
        assert_eq!(contents_2, file_2);
        assert_eq!(mode, mode_2);
        let (file_3, mode_3) = test_directory.read_file(filename_3.as_str());
        assert_eq!(contents_3, file_3);
        assert_eq!(mode, mode_3);
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
            std::fs::create_dir_all(&self.path).unwrap();
            set_mode_700(&self.path.to_str().unwrap()).unwrap();
            self.path.as_path()
        }

        fn create_file(&self, filename: &str, contents: &[u8]) {
            std::fs::create_dir_all(&self.path).unwrap();
            let path = &self.path.join(filename);
            std::fs::write(path, contents).unwrap();
            set_mode_600(path.to_str().unwrap()).unwrap();
        }

        fn read_file(&self, filename: &str) -> (Vec<u8>, u16) {
            let path = &self.path.join(filename);
            let contents = std::fs::read(path).unwrap();
            let mode = get_mode(path.to_str().unwrap()).unwrap();
            (contents, mode)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.path).unwrap();
        }
    }
}
