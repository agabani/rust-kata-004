use std::path::PathBuf;

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
        /* TODO: create files and directories with the following chmod
           -rw------- hostname
           drwx------ authorized_clients
           -rw------- hs_ed25519_public_key
           -rw------- hs_ed25519_secret_key
        */
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    pub struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new() -> Self {
            let path = PathBuf::from(format!("test-{}", Faker.fake::<String>()));
            Self { path }
        }

        fn path(&self) -> &Path {
            std::fs::create_dir_all(&self.path).unwrap();
            self.path.as_path()
        }

        fn create_file(&self, filename: &str, contents: &[u8]) {
            std::fs::create_dir_all(&self.path).unwrap();
            let path = &self.path.join(filename);
            std::fs::write(path, contents).unwrap();
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.path).unwrap();
        }
    }
}
