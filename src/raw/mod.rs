use std::fmt::Display;
use std::ops::{Deref, DerefMut};

pub mod database;
pub mod keyfile;
pub mod lockfile;

pub const DCONF_PATH: &str = "/etc/dconf/";

pub struct KeyPath(Vec<String>);

impl Deref for KeyPath {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for KeyPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl Display for KeyPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for s in &self.0 {
            buffer.push('/');
            buffer.push_str(&s);
        }
        return write!(f, "{}", &buffer[1..]);
    }
}

pub struct KeyURI {
    pub path: KeyPath,
    pub name: String,
}

impl Display for KeyURI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}/{}", self.path, self.name);
    }
}

pub enum Value {
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match &self {
            Value::String(s) => write!(f, "\'{}\'", s),
        };
    }
}
