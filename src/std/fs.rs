#[cfg(unix)]
use std::os::unix::fs::PermissionsExt as _;
use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

pub trait FileExt {
    fn open_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn create_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn create_new_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    #[cfg(unix)]
    fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
    #[cfg(unix)]
    fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
    #[cfg(unix)]
    fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
}

impl FileExt for File {
    #[inline]
    fn open_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::open(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn create_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        let mut options = Self::options();

        options.write(true).create_new(true);

        options.open_if_not_exists(path)
    }

    #[inline]
    fn create_new_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::create_new(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[cfg(unix)]
    #[inline]
    fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata()?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions_mode);

        self.set_permissions(permissions)
    }

    #[cfg(unix)]
    #[inline]
    fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata()?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() | permissions_mode);

        self.set_permissions(permissions)
    }

    #[cfg(unix)]
    #[inline]
    fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata()?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() & !permissions_mode);

        self.set_permissions(permissions)
    }
}

pub trait OpenOptionsExt {
    fn open_if_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>>;

    fn open_if_not_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>>;
}

impl OpenOptionsExt for OpenOptions {
    #[inline]
    fn open_if_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>> {
        match self.open(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn open_if_not_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>> {
        match self.open(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }
}
