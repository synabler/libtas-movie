//! Module that defines a movie.

use core::str::FromStr as _;

use crate::{
    config::{Config, InvalidConfigError},
    inputs::{Inputs, InvalidInputsError},
};

/// A libTAS movie.
#[derive(Clone, Debug, Default)]
pub struct LibTASMovie {
    /// Config corresponding to `config.ini`.
    pub config: Config,
    /// Movie inputs corresponding to `inputs`.
    pub inputs: Inputs,
    /// Annotations corresponding to `annotations.txt` (TODO).
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
}
