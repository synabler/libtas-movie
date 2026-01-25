use std::{fs::File, path::Path};

#[derive(Debug)]
pub enum LoadError {
    FileError(std::io::Error),
}

pub fn load_movie<P: AsRef<Path>>(path: P) -> Result<(), LoadError> {
    match File::open(path) {
        Ok(_file) => {
            return Ok(());
        }
        Err(err) => {
            return Err(LoadError::FileError(err));
        }
    }
}
