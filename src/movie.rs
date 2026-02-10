//! Module that defines a movie.

use core::str::FromStr as _;

use crate::{
    config::{Config, InvalidConfigError},
    inputs::{Inputs, InvalidInputsError},
};
use std::{fs::File, io::Read as _, path::Path};

use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use tar::{Archive, Builder, Header};

/// An error while loading a movie file.
#[derive(Debug)]
pub enum LoadError {
    /// An error occurred while opening a file.
    FileError(std::io::Error),
    /// The file is not a `tar.gz` archive.
    InvalidArchive,
    /// An extra file is in the archive.
    ExtraEntry,
    /// A file is missing in the archive.
    InsufficientEntry,
    /// `Config` is incorrect.
    InvalidConfig(InvalidConfigError),
    /// `Inputs` is incorrect.
    InvalidInputs(InvalidInputsError),
}

/// A libTAS movie.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LibTASMovie {
    /// Config corresponding to `config.ini`.
    pub config: Config,
    /// Movie inputs corresponding to `inputs`.
    pub inputs: Inputs,
    /// Annotations corresponding to `annotations.txt`.
    pub annotations: String,
    /// TAS editor information corresponding to `editor.ini` (TODO).
    pub editor: String,
}

impl LibTASMovie {
    pub(crate) fn load_config(&mut self, string: &str) -> Result<(), InvalidConfigError> {
        match Config::from_str(string) {
            Ok(config) => {
                self.config = config;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub(crate) fn load_inputs(&mut self, string: &str) -> Result<(), InvalidInputsError> {
        match Inputs::from_str(string) {
            Ok(inputs) => {
                self.inputs = inputs;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub(crate) fn load_annotations(&mut self, string: &str) {
        string.clone_into(&mut self.annotations);
    }

    pub(crate) fn load_editor(&mut self, string: &str) {
        string.clone_into(&mut self.editor);
    }

    /// Saves the TAS into a byte sequence representing the `.ltm` file.
    pub fn compress(&self) -> std::io::Result<Vec<u8>> {
        let bytes = vec![];
        let enc = GzEncoder::new(bytes, Compression::default());
        let mut tar = Builder::new(enc);

        let mut header = Header::new_gnu();
        for (file_name, data) in [
            ("config.ini", &self.config.to_string()),
            ("inputs", &self.inputs.to_string()),
            ("annotations.txt", &self.annotations),
            ("editor.ini", &self.editor),
        ] {
            header.set_path(file_name)?;
            header.set_size(data.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append(&header, data.as_bytes())?;
        }

        let enc = tar.into_inner()?;
        enc.finish()
    }

    /// Saves the TAS into `path`.
    pub fn save_to_path<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let data = self.compress()?;
        std::fs::write(path, data)
    }
}

/// Loads a movie file in `path`.
///
/// # Example
/// ```ignore
/// use libtas_movie::load::load_movie;
/// let movie = load_movie("path/to/tas.ltm").unwrap();
/// ```
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
                if let Err(err) = movie.load_config(&string) {
                    return Err(LoadError::InvalidConfig(err));
                }
            }
            Ok(path) if path.as_os_str() == "inputs" => {
                loaded[1] = true;
                if let Err(err) = movie.load_inputs(&string) {
                    return Err(LoadError::InvalidInputs(err));
                }
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
    if loaded.as_slice() != [true, true, true, true] {
        return Err(LoadError::InsufficientEntry);
    }

    Ok(movie)
}
