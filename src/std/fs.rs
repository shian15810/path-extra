#[cfg(unix)]
use std::os::unix::fs::PermissionsExt as _;
use std::{fs::File, io, path::Path};

pub trait FileExt {
    fn open_write(path: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;
    fn open_read_write(path: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;

    fn open_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;
    fn open_write_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;
    fn open_read_write_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
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
    fn open_write(path: impl AsRef<Path>) -> io::Result<Self> {
        Self::options().write(true).open(path)
    }

    fn open_read_write(path: impl AsRef<Path>) -> io::Result<Self> {
        Self::options().read(true).write(true).open(path)
    }

    fn open_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::open(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn open_write_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::open_write(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn open_read_write_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::open_read_write(path) {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[cfg(unix)]
    fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata()?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions_mode);

        self.set_permissions(permissions)
    }

    #[cfg(unix)]
    fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata()?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() | permissions_mode);

        self.set_permissions(permissions)
    }

    #[cfg(unix)]
    fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata()?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() & !permissions_mode);

        self.set_permissions(permissions)
    }
}
