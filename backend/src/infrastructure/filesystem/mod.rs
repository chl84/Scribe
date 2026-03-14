use std::fs;
use std::io;
use std::path::Path;

pub trait FileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
    fn write_string(&self, path: &Path, contents: &str) -> io::Result<()>;
}

pub struct LocalFileSystem;

impl FileSystem for LocalFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn write_string(&self, path: &Path, contents: &str) -> io::Result<()> {
        fs::write(path, contents)
    }
}
