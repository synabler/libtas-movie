#[derive(Clone, Debug, Default)]
pub struct LibTASMovie {
    config: String,
    inputs: String,
    annotations: String,
    editor: String,
}

impl LibTASMovie {
    pub(crate) fn load_config(&mut self, string: &str) {
        self.config = string.to_owned();
    }

    pub(crate) fn load_inputs(&mut self, string: &str) {
        self.inputs = string.to_owned();
    }

    pub(crate) fn load_annotations(&mut self, string: &str) {
        self.annotations = string.to_owned();
    }

    pub(crate) fn load_editor(&mut self, string: &str) {
        self.editor = string.to_owned();
    }
}
