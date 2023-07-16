use std::{collections::HashSet, fmt::Display};

use super::KeyURI;

pub struct LockFile {
    pub name: String,
    pub locks: HashSet<KeyURI>
}

impl Display for LockFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for lock in &self.locks {
            write!(f, "{}\n", lock)?;
        }
        return Ok(());
    }
}