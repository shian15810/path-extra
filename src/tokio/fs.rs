#[cfg(unix)]
use std::os::unix::fs::PermissionsExt as _;
use std::{io, path::Path};

use tokio::fs::{File, OpenOptions};

#[expect(async_fn_in_trait)]
pub trait FileExt {
    async fn open_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    async fn create_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    async fn create_new_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    #[cfg(unix)]
    async fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
    #[cfg(unix)]
    async fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
    #[cfg(unix)]
    async fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
}

impl FileExt for File {
    #[inline]
    async fn open_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::open(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    async fn create_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        let mut options = Self::options();

        options.write(true).create_new(true);

        options.open_if_not_exists(path).await
    }

    #[inline]
    async fn create_new_if_not_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::create_new(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[cfg(unix)]
    #[inline]
    async fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata().await?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions_mode);

        self.set_permissions(permissions).await
    }

    #[cfg(unix)]
    #[inline]
    async fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata().await?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() | permissions_mode);

        self.set_permissions(permissions).await
    }

    #[cfg(unix)]
    #[inline]
    async fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata().await?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() & !permissions_mode);

        self.set_permissions(permissions).await
    }
}

#[expect(async_fn_in_trait)]
pub trait OpenOptionsExt {
    async fn open_if_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>>;

    async fn open_if_not_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>>;
}

impl OpenOptionsExt for OpenOptions {
    #[inline]
    async fn open_if_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>> {
        match self.open(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    async fn open_if_not_exists(&self, path: impl AsRef<Path>) -> io::Result<Option<File>> {
        match self.open(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }
}
