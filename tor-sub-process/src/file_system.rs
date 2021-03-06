use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct HiddenServiceDirectoryFile {
    pub relative_path: PathBuf,
    pub contents: Vec<u8>,
}

impl HiddenServiceDirectoryFile {
    pub fn from(relative_path: PathBuf, contents: Vec<u8>) -> Result<Self, ()> {
        if relative_path.is_relative() {
            Ok(Self {
                relative_path,
                contents,
            })
        } else {
            Err(())
        }
    }
}

pub struct HiddenServiceDirectory {
    base_path: PathBuf,
}

impl HiddenServiceDirectory {
    pub fn new(path: PathBuf) -> Self {
        Self { base_path: path }
    }

    pub fn get_secret_files(&self) -> Vec<HiddenServiceDirectoryFile> {
        let paths = std::fs::read_dir(&self.base_path)
            .unwrap()
            .map(|parent| parent.map(|child| child.path()).unwrap());

        let file_paths = paths.filter(|path| path.is_file());

        let files = file_paths.map(|file| {
            HiddenServiceDirectoryFile::from(
                PathBuf::from(file.file_name().unwrap()),
                std::fs::read(file).unwrap(),
            )
            .unwrap()
        });

        files.collect::<Vec<_>>()
    }

    pub fn save_secret_files(&self, secret_files: &[HiddenServiceDirectoryFile]) {
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

        println!("{:?}", vec);
    }
}
