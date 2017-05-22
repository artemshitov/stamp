use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use error::Error;

pub fn read_file<T: AsRef<Path>>(path: T) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;
    let file_len = file.metadata()?.len();
    let mut buf = Vec::with_capacity(file_len as usize + 1);
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn read_all_files<T: AsRef<Path>>(root: T) -> Result<Vec<(PathBuf, Vec<u8>)>, Error> {
    let mut result = Vec::new();
    for dir_entry in WalkDir::new(&root) {
        let entry = dir_entry?;
        let path = entry.path();
        if path.is_file() {
            let file = read_file(path)?;
            let path = path.strip_prefix(&root)?;
            result.push((path.to_owned(), file));
        }
    }
    Ok(result)
}

pub fn write_file(path: &Path, content: &[u8]) -> Result<(), Error> {
    create_dir_all(path.to_owned().parent().ok_or(Error::DirNotFound)?)?;
    let mut file = File::create(path)?;
    file.write_all(content)?;
    Ok(())
}
