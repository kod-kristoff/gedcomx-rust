use crate::common::TextValue;

#[derive(Debug, Clone)]
pub struct Agent {
    id: String,
    names: Vec<TextValue>,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            names: Vec::new(),
        }
    }
}

impl Agent {
    pub fn id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.names.push(name.into());
        self
    }
}
