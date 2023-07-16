use std::{collections::HashMap, fmt::Display};

use super::{KeyPath, Value};

pub struct KeySet {
    pub path: KeyPath,
    pub values: HashMap<String, Value>
}

impl Display for KeySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]\n", self.path)?;
        for (key, value) in &self.values {
            write!(f, "{}: {}\n", key, value)?;
        }
        return Ok(());
    }
}

pub struct KeyFile {
    pub name: String,
    pub keys: HashMap<KeyPath, KeySet>
}

impl std::fmt::Display for KeyFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_, keyset) in &self.keys {
            write!(f, "{}\n\n", keyset)?;
        }
        return Ok(());
    }
}