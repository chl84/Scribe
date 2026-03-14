use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::infrastructure::filesystem::FileSystem;

#[derive(Default)]
pub struct MemoryFileSystem {
    files: HashMap<PathBuf, String>,
}

impl MemoryFileSystem {
}

impl FileSystem for MemoryFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing file"))
    }

    fn write_string(&self, _path: &Path, _contents: &str) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "immutable memory filesystem",
        ))
    }
}

#[derive(Default)]
pub struct WritableMemoryFileSystem {
    files: Mutex<HashMap<PathBuf, String>>,
}

impl WritableMemoryFileSystem {
    pub fn with_file(path: impl Into<PathBuf>, contents: impl Into<String>) -> Self {
        let mut files = HashMap::new();
        files.insert(path.into(), contents.into());
        Self {
            files: Mutex::new(files),
        }
    }
}

impl FileSystem for WritableMemoryFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .lock()
            .map_err(|_| io::Error::other("lock poisoned"))?
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing file"))
    }

    fn write_string(&self, path: &Path, contents: &str) -> io::Result<()> {
        self.files
            .lock()
            .map_err(|_| io::Error::other("lock poisoned"))?
            .insert(path.to_path_buf(), contents.to_string());
        Ok(())
    }
}

#[derive(Default)]
pub struct FailingWriteFileSystem {
    files: HashMap<PathBuf, String>,
}

impl FailingWriteFileSystem {
    pub fn with_file(path: impl Into<PathBuf>, contents: impl Into<String>) -> Self {
        let mut files = HashMap::new();
        files.insert(path.into(), contents.into());
        Self { files }
    }
}

impl FileSystem for FailingWriteFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing file"))
    }

    fn write_string(&self, _path: &Path, _contents: &str) -> io::Result<()> {
        Err(io::Error::other("write failed"))
    }
}
