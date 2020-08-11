use crate::error::StapleError;
use fs2::FileExt;
use std::{
    fs::{File, OpenOptions},
    ops::Deref,
    path::Path,
};

pub struct LockFile(File);

impl Deref for LockFile {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LockFile {
    pub fn new(path: impl AsRef<Path>) -> Result<LockFile, StapleError> {
        let file = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(path.as_ref())?;
        Ok(LockFile(file))
    }
    pub fn lock_file(&self) -> Result<(), StapleError> {
        Ok(self.0.lock_exclusive()?)
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        let _result = self.0.unlock();
    }
}
