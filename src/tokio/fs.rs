#[cfg(unix)]
use std::os::unix::{self, fs::PermissionsExt as _};
use std::{fs::Permissions, io, path::Path};

use tokio::fs::{File, OpenOptions};
#[cfg(unix)]
use tokio::task;

#[trait_variant::make(Send)]
pub trait FileExt {
    async fn create_new_if_not_exists(path: impl AsRef<Path> + Send) -> io::Result<Option<Self>>
    where
        Self: Sized;

    async fn create_if_not_exists(path: impl AsRef<Path> + Send) -> io::Result<Option<Self>>
    where
        Self: Sized;

    async fn open_if_exists(path: impl AsRef<Path> + Send) -> io::Result<Option<Self>>
    where
        Self: Sized;

    async fn with_permissions(&self, perm: Permissions) -> io::Result<&Self>;

    async fn with_permissions_readonly(&self, readonly: bool) -> io::Result<&Self>;

    #[cfg(unix)]
    async fn with_permissions_mode(&self, mode: u32) -> io::Result<&Self>;
    #[cfg(unix)]
    async fn add_permissions_mode(&self, mode: u32) -> io::Result<&Self>;
    #[cfg(unix)]
    async fn remove_permissions_mode(&self, mode: u32) -> io::Result<&Self>;

    #[cfg(unix)]
    async fn chown(&self, uid: Option<u32>, gid: Option<u32>) -> io::Result<&Self>;
}

impl FileExt for File {
    #[inline]
    async fn create_new_if_not_exists(path: impl AsRef<Path> + Send) -> io::Result<Option<Self>> {
        match Self::create_new(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    async fn create_if_not_exists(path: impl AsRef<Path> + Send) -> io::Result<Option<Self>> {
        let mut options = Self::options();

        options.write(true).create_new(true);

        options.open_if_not_exists(path).await
    }

    #[inline]
    async fn open_if_exists(path: impl AsRef<Path> + Send) -> io::Result<Option<Self>> {
        match Self::open(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    async fn with_permissions(&self, perm: Permissions) -> io::Result<&Self> {
        self.set_permissions(perm).await?;

        Ok(self)
    }

    #[inline]
    async fn with_permissions_readonly(&self, readonly: bool) -> io::Result<&Self> {
        let meta = self.metadata().await?;

        let mut perm = meta.permissions();

        perm.set_readonly(readonly);

        self.with_permissions(perm).await
    }

    #[cfg(unix)]
    #[inline]
    async fn with_permissions_mode(&self, mode: u32) -> io::Result<&Self> {
        let perm = Permissions::from_mode(mode);

        self.with_permissions(perm).await
    }

    #[cfg(unix)]
    #[inline]
    async fn add_permissions_mode(&self, mode: u32) -> io::Result<&Self> {
        let meta = self.metadata().await?;

        let perm = meta.permissions();

        self.with_permissions_mode(perm.mode() | mode).await
    }

    #[cfg(unix)]
    #[inline]
    async fn remove_permissions_mode(&self, mode: u32) -> io::Result<&Self> {
        let meta = self.metadata().await?;

        let perm = meta.permissions();

        self.with_permissions_mode(perm.mode() & !mode).await
    }

    #[cfg(unix)]
    #[inline]
    async fn chown(&self, uid: Option<u32>, gid: Option<u32>) -> io::Result<&Self> {
        let this = self.try_clone().await?;

        task::spawn_blocking(move || unix::fs::fchown(this, uid, gid)).await??;

        Ok(self)
    }
}

#[trait_variant::make(Send)]
pub trait OpenOptionsExt {
    async fn open_if_not_exists(&self, path: impl AsRef<Path> + Send) -> io::Result<Option<File>>;
    async fn open_if_exists(&self, path: impl AsRef<Path> + Send) -> io::Result<Option<File>>;
}

impl OpenOptionsExt for OpenOptions {
    #[inline]
    async fn open_if_not_exists(&self, path: impl AsRef<Path> + Send) -> io::Result<Option<File>> {
        match self.open(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    async fn open_if_exists(&self, path: impl AsRef<Path> + Send) -> io::Result<Option<File>> {
        match self.open(path).await {
            Ok(file) => Ok(Some(file)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }
}
