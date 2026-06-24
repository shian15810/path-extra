use std::{
    io,
    mem,
    path::{Path, PathBuf},
};

use super::path::PathExt as _;

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

    pub(super) async fn persist(mut self, to: impl AsRef<Path>) -> io::Result<()> {
        self.path.rename(to).await?;

        mem::take(&mut self.path);

        mem::forget(self);

        Ok(())
    }
}

mod drop {
    use super::TempPath;
    use crate::std::path::PathExt as _;

    impl Drop for TempPath {
        fn drop(&mut self) {
            if self.path.remove_file().is_err() {
                let _ = self.path.remove_dir_all();
            }
        }
    }
}
