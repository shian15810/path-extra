#[cfg(unix)]
use std::os::unix::fs::PermissionsExt as _;
use std::{io, path::Path};

use tokio::fs::File;

#[expect(async_fn_in_trait)]
pub trait FileExt {
    async fn open_write(path: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;
    async fn open_read_write(path: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;

    async fn open_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;
    async fn open_write_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;
    async fn open_read_write_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>>
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
    async fn open_write(path: impl AsRef<Path>) -> io::Result<Self> {
        Self::options().write(true).open(path).await
    }

    async fn open_read_write(path: impl AsRef<Path>) -> io::Result<Self> {
        Self::options().read(true).write(true).open(path).await
    }

    async fn open_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::open(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn open_write_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::open_write(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn open_read_write_if_exists(path: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match Self::open_read_write(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[cfg(unix)]
    async fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata().await?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions_mode);

        self.set_permissions(permissions).await
    }

    #[cfg(unix)]
    async fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata().await?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() | permissions_mode);

        self.set_permissions(permissions).await
    }

    #[cfg(unix)]
    async fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = self.metadata().await?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() & !permissions_mode);

        self.set_permissions(permissions).await
    }
}
