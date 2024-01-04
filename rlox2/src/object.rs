use std::fmt::Display;

use derive_new::new;

#[derive(Clone, Debug, new, PartialEq, Eq)]
pub enum Obj {
    String(ObjString),
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Obj::String(obj_string) => obj_string.fmt(f),
        }
    }
}

#[derive(Clone, Debug, new, PartialEq, Eq)]
pub struct ObjString(String);

impl Display for ObjString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for ObjString
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
