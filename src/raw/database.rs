use std::{fs::{OpenOptions, self}, path::PathBuf};

use super::{keyfile::KeyFile, lockfile::LockFile, DCONF_PATH};
use std::io::Write;

pub struct Database {
    name: String,
    keyfiles: Vec<KeyFile>,
    lockfiles: Vec<LockFile>,
}

impl Database {
    pub fn get_path(&self) -> PathBuf {
        let mut path = PathBuf::from(DCONF_PATH);
        path.push("db");
        path.push(format!("{}.d", self.name));
        return path;
    }

    pub fn write_keyfiles(&self) -> std::io::Result<()> {
        let mut path = self.get_path();
        fs::create_dir_all(&path)?;

        let mut i = 0;
        for keyfile in &self.keyfiles {
            path.push(format!("{:0>2}_{}", i, keyfile.name));
            let mut f = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&path)?;
            write!(&mut f, "{}", keyfile)?;
            i += 1;
        }
        return Ok(());
    }

    pub fn write_lockfiles(&self) -> std::io::Result<()> {
        let mut path = self.get_path();
        path.push("locks");
        fs::create_dir_all(&path)?;

        let mut i = 0;
        for lockfile in &self.lockfiles {
            path.push(format!("{:0>2}_{}", i, lockfile.name));
            let mut f = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&path)?;
            write!(&mut f, "{}", lockfile)?;
            i += 1;
        }
        return Ok(());
    }
}
