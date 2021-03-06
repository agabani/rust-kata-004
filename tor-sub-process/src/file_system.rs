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

    #[test]
    pub fn test() {
        let directory = HiddenServiceDirectory::new(PathBuf::from("/var/tmp/unit_test_service"));

        let vec = directory.get_secret_files();
    }
}
