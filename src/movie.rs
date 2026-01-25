use std::str::FromStr;

use crate::{
    config::{Config, InvalidConfig},
    inputs::{Inputs, InvalidInputs},
};

#[derive(Clone, Debug, Default)]
pub struct LibTASMovie {
    pub config: Config,
    pub inputs: Inputs,
    pub annotations: String,
    pub editor: String,
}

impl LibTASMovie {
    pub(crate) fn load_config(&mut self, string: &str) -> Result<(), InvalidConfig> {
        match Config::from_str(string) {
            Ok(config) => {
                self.config = config;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub(crate) fn load_inputs(&mut self, string: &str) -> Result<(), InvalidInputs> {
        match Inputs::from_str(string) {
            Ok(inputs) => {
                self.inputs = inputs;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub(crate) fn load_annotations(&mut self, string: &str) {
        self.annotations = string.to_owned();
    }

    pub(crate) fn load_editor(&mut self, string: &str) {
        self.editor = string.to_owned();
    }
}
