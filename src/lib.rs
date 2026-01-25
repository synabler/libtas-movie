use std::{fs::File, path::Path};

use flate2::read::GzDecoder;
use tar::Archive;

#[derive(Debug)]
pub enum LoadError {
    FileError(std::io::Error),
    InvalidArchive,
}

pub fn load_movie<P: AsRef<Path>>(path: P) -> Result<(), LoadError> {
    // open the movie file as .tar.gz
    let mut archive = match File::open(path) {
        Ok(file) => Archive::new(GzDecoder::new(file)),
        Err(err) => {
            return Err(LoadError::FileError(err));
        }
    };

    let entries = match archive.entries() {
        Ok(entries) => entries,
        Err(err) => {
            return Err(LoadError::FileError(err));
        }
    };

    for entry in entries {
        let Ok(entry) = entry else {
            return Err(LoadError::InvalidArchive);
        };
        println!("{:?}", entry.path());
    }

    return Ok(());
}
