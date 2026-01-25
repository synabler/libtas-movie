use std::{fs::File, io::Read, path::Path};

use crate::movie::LibTASMovie;
use flate2::read::GzDecoder;
use tar::Archive;

#[derive(Debug)]
pub enum LoadError {
    FileError(std::io::Error),
    InvalidArchive,
    ExtraEntry,
    InsufficientEntry,
}

pub fn load_movie<P: AsRef<Path>>(path: P) -> Result<LibTASMovie, LoadError> {
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

    let mut movie = LibTASMovie::default();
    let mut loaded = [false, false, false, false];
    for entry in entries {
        let Ok(mut entry) = entry else {
            return Err(LoadError::InvalidArchive);
        };

        let mut string = String::new();
        let Ok(_) = entry.read_to_string(&mut string) else {
            return Err(LoadError::InvalidArchive);
        };

        match entry.path() {
            Ok(path) if path.as_os_str() == "config.ini" => {
                loaded[0] = true;
                movie.load_config(&string);
            }
            Ok(path) if path.as_os_str() == "inputs" => {
                loaded[1] = true;
                movie.load_inputs(&string);
            }
            Ok(path) if path.as_os_str() == "annotations.txt" => {
                loaded[2] = true;
                movie.load_annotations(&string);
            }
            Ok(path) if path.as_os_str() == "editor.ini" => {
                loaded[3] = true;
                movie.load_editor(&string);
            }
            Ok(_path) => {
                return Err(LoadError::ExtraEntry);
            }
            _ => {
                return Err(LoadError::InvalidArchive);
            }
        }
    }
    if loaded.as_slice() != &[true, true, true, true] {
        return Err(LoadError::InsufficientEntry);
    }

    Ok(movie)
}
