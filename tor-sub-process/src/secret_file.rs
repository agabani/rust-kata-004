use std::fmt::Formatter;
use std::path::{Path, PathBuf};

/// Interface for Secret Files in Hidden Service Directories.
pub struct SecretFile {
    relative_path: PathBuf,
    contents: Vec<u8>,
}

impl std::fmt::Display for SecretFile {
    /// Formats the SecretFile without exposing the contents.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({} bytes)",
            &self.relative_path.to_str().unwrap(),
            &self.contents.len()
        )
    }
}

impl SecretFile {
    /// Creates a `SecretFile` from a relative path and contents.
    ///
    /// Fails if absolute path is provided.
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

    /// Gets the relative path of the SecretFile.
    pub fn relative_path(&self) -> &Path {
        self.relative_path.as_path()
    }

    /// Gets the contents of the secretFile.
    pub fn contents(&self) -> &[u8] {
        &self.contents
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_absolute_path_fails() {
        let absolute_path = if cfg!(target_family = "unix") {
            "/absolute/path/hostname"
        } else {
            "C:\\absolute\\path\\hostname"
        };

        let path = PathBuf::from(absolute_path);
        let result = SecretFile::from(path, vec![]);

        assert!(result.is_err())
    }

    #[test]
    fn from_relative_path_succeeds() {
        let relative_path = if cfg!(target_family = "unix") {
            "relative/path/hostname"
        } else {
            "relative\\path\\hostname"
        };

        let path = PathBuf::from(relative_path);
        let result = SecretFile::from(path, vec![1, 2, 3]);

        assert!(result.is_ok());

        let file = result.unwrap();

        assert_eq!(relative_path, file.relative_path().to_str().unwrap());
        assert_eq!(vec![1, 2, 3], file.contents())
    }

    #[test]
    fn display_omits_contents() {
        let relative_path = if cfg!(target_family = "unix") {
            "relative/path/hostname"
        } else {
            "relative\\path\\hostname"
        };

        let path = PathBuf::from(relative_path);
        let file = SecretFile::from(path, vec![1, 2, 3]).unwrap();

        assert_eq!(&format!("{} (3 bytes)", relative_path), &file.to_string())
    }
}
