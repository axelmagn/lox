use std::fmt::Display;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct LoxClass {
    name: String,
}

impl LoxClass {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}
