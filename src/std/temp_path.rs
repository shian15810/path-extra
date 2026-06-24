use std::{
    fs,
    io,
    mem,
    path::{Path, PathBuf},
};

pub(super) struct TempPath {
    path: PathBuf,
}

impl TempPath {
    pub(super) fn try_from_path(path: impl Into<PathBuf>) -> io::Result<Self> {
        let mut path = path.into();

        if !path.add_extension("tmp") {
            let err = io::Error::new(io::ErrorKind::InvalidInput, "Invalid path file name");

            return Err(err);
        }

        let this = Self {
            path,
        };

        Ok(this)
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    pub(super) fn persist(mut self, to: impl AsRef<Path>) -> io::Result<()> {
        fs::rename(&self.path, to)?;

        mem::take(&mut self.path);

        mem::forget(self);

        Ok(())
    }
}

impl Drop for TempPath {
    fn drop(&mut self) {
        if fs::remove_file(&self.path).is_err() {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
