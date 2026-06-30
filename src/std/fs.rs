#[cfg(unix)]
use std::os::unix::{self, fs::PermissionsExt as _};
use std::{
    fs::{File, OpenOptions, Permissions},
    io,
    path::Path,
};

pub trait FileExt {
    fn create_new_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn create_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn open_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn with_permissions(&self, perm: Permissions) -> io::Result<&Self>;

    fn with_permissions_readonly(&self, readonly: bool) -> io::Result<&Self>;

    #[cfg(unix)]
    fn with_permissions_mode(&self, mode: u32) -> io::Result<&Self>;
    #[cfg(unix)]
    fn add_permissions_mode(&self, mode: u32) -> io::Result<&Self>;
    #[cfg(unix)]
    fn remove_permissions_mode(&self, mode: u32) -> io::Result<&Self>;

    #[cfg(unix)]
    fn chown(&self, uid: Option<u32>, gid: Option<u32>) -> io::Result<&Self>;
}

impl FileExt for File {
    #[inline]
    fn create_new_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::create_new(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
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
    fn open_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::open(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn with_permissions(&self, perm: Permissions) -> io::Result<&Self> {
        self.set_permissions(perm)?;

        Ok(self)
    }

    #[inline]
    fn with_permissions_readonly(&self, readonly: bool) -> io::Result<&Self> {
        let meta = self.metadata()?;

        let mut perm = meta.permissions();

        perm.set_readonly(readonly);

        self.with_permissions(perm)
    }

    #[cfg(unix)]
    #[inline]
    fn with_permissions_mode(&self, mode: u32) -> io::Result<&Self> {
        let perm = Permissions::from_mode(mode);

        self.with_permissions(perm)
    }

    #[cfg(unix)]
    #[inline]
    fn add_permissions_mode(&self, mode: u32) -> io::Result<&Self> {
        let meta = self.metadata()?;

        let perm = meta.permissions();

        self.with_permissions_mode(perm.mode() | mode)
    }

    #[cfg(unix)]
    #[inline]
    fn remove_permissions_mode(&self, mode: u32) -> io::Result<&Self> {
        let meta = self.metadata()?;

        let perm = meta.permissions();

        self.with_permissions_mode(perm.mode() & !mode)
    }

    #[cfg(unix)]
    #[inline]
    fn chown(&self, uid: Option<u32>, gid: Option<u32>) -> io::Result<&Self> {
        unix::fs::fchown(self, uid, gid)?;

        Ok(self)
    }
}

pub trait OpenOptionsExt {
    fn open_if_not_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>>;
    fn open_if_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>>;
}

impl OpenOptionsExt for OpenOptions {
    #[inline]
    fn open_if_not_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>> {
        match self.open(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn open_if_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>> {
        match self.open(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }
}
