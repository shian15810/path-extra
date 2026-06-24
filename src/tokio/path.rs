use std::{
    borrow::Cow,
    ffi::OsStr,
    fs::Metadata,
    io,
    path::{self, Path, PathBuf},
};
#[cfg(unix)]
use std::{fs::Permissions, os::unix::fs::PermissionsExt as _};

#[cfg(unix)]
use pathdiff::diff_paths;
use tokio::fs::{self, ReadDir};

use super::temp_path::TempPath;

#[expect(async_fn_in_trait)]
pub trait PathExt {
    fn absolute(&self) -> io::Result<PathBuf>;

    fn base(&self) -> io::Result<&Self>;
    fn with_base(&self, base: impl AsRef<Self>) -> Cow<'_, Self>
    where
        Self: ToOwned;

    fn file_suffix(&self) -> Option<&OsStr>;

    async fn metadata_async(&self) -> io::Result<Metadata>;
    async fn symlink_metadata_async(&self) -> io::Result<Metadata>;
    async fn canonicalize_async(&self) -> io::Result<PathBuf>;
    async fn read_link_async(&self) -> io::Result<PathBuf>;
    async fn read_dir_async(&self) -> io::Result<ReadDir>;

    async fn exists_async(&self) -> bool;
    async fn is_file_async(&self) -> bool;
    async fn is_dir_async(&self) -> bool;
    async fn is_symlink_async(&self) -> bool;

    async fn try_exists_async(&self) -> io::Result<bool>;
    async fn try_is_file_async(&self) -> io::Result<bool>;
    async fn try_is_dir_async(&self) -> io::Result<bool>;
    async fn try_is_symlink_async(&self) -> io::Result<bool>;

    async fn metadata_if_exists(&self) -> io::Result<Option<Metadata>>;
    async fn symlink_metadata_if_exists(&self) -> io::Result<Option<Metadata>>;
    async fn canonicalize_if_exists(&self) -> io::Result<Option<PathBuf>>;
    async fn read_link_if_exists(&self) -> io::Result<Option<PathBuf>>;
    async fn read_dir_if_exists(&self) -> io::Result<Option<ReadDir>>;

    async fn exists_nofollow(&self) -> bool;
    async fn is_file_nofollow(&self) -> bool;
    async fn is_dir_nofollow(&self) -> bool;

    async fn try_exists_nofollow(&self) -> io::Result<bool>;
    async fn try_is_file_nofollow(&self) -> io::Result<bool>;
    async fn try_is_dir_nofollow(&self) -> io::Result<bool>;

    async fn is_read_dir_empty(&self) -> io::Result<bool>;
    async fn is_read_dir_empty_if_exists(&self) -> io::Result<Option<bool>>;

    async fn hard_link(&self, link: impl AsRef<Self>) -> io::Result<()>;
    async fn hard_link_atomic(&self, link: impl AsRef<Self>) -> io::Result<()>;

    #[cfg(unix)]
    async fn symlink(&self, link: impl AsRef<Self>) -> io::Result<()>;
    #[cfg(unix)]
    async fn symlink_atomic(&self, link: impl AsRef<Self>) -> io::Result<()>;

    #[cfg(unix)]
    async fn symlink_absolute(&self, link: impl AsRef<Self>) -> io::Result<()>;
    #[cfg(unix)]
    async fn symlink_absolute_atomic(&self, link: impl AsRef<Self>) -> io::Result<()>;

    #[cfg(unix)]
    async fn symlink_relative(&self, link: impl AsRef<Self>) -> io::Result<()>;
    #[cfg(unix)]
    async fn symlink_relative_atomic(&self, link: impl AsRef<Self>) -> io::Result<()>;

    #[cfg(unix)]
    async fn set_permissions(&self, permissions: Permissions) -> io::Result<()>;
    #[cfg(unix)]
    async fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
    #[cfg(unix)]
    async fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
    #[cfg(unix)]
    async fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
}

impl PathExt for Path {
    fn absolute(&self) -> io::Result<PathBuf> {
        path::absolute(self)
    }

    fn base(&self) -> io::Result<&Self> {
        self.parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Path has no parent"))
    }

    fn with_base(&self, base: impl AsRef<Self>) -> Cow<'_, Self> {
        if self.is_relative() {
            let path = base.as_ref().join(self);

            Cow::Owned(path)
        } else {
            Cow::Borrowed(self)
        }
    }

    fn file_suffix(&self) -> Option<&OsStr> {
        let file_name = self.file_name()?;
        let file_prefix = self.file_prefix()?;

        let bytes = file_name.as_encoded_bytes();

        let start = file_prefix.as_encoded_bytes().len() + 1;

        if start <= bytes.len() {
            let slice = &bytes[start..];

            let os_str = unsafe { OsStr::from_encoded_bytes_unchecked(slice) };

            Some(os_str)
        } else {
            None
        }
    }

    async fn metadata_async(&self) -> io::Result<Metadata> {
        fs::metadata(self).await
    }

    async fn symlink_metadata_async(&self) -> io::Result<Metadata> {
        fs::symlink_metadata(self).await
    }

    async fn canonicalize_async(&self) -> io::Result<PathBuf> {
        fs::canonicalize(self).await
    }

    async fn read_link_async(&self) -> io::Result<PathBuf> {
        fs::read_link(self).await
    }

    async fn read_dir_async(&self) -> io::Result<ReadDir> {
        fs::read_dir(self).await
    }

    async fn exists_async(&self) -> bool {
        self.metadata_async().await.is_ok()
    }

    async fn is_file_async(&self) -> bool {
        self.metadata_async()
            .await
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }

    async fn is_dir_async(&self) -> bool {
        self.metadata_async()
            .await
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    }

    async fn is_symlink_async(&self) -> bool {
        self.symlink_metadata_async()
            .await
            .map(|metadata| metadata.is_symlink())
            .unwrap_or(false)
    }

    async fn try_exists_async(&self) -> io::Result<bool> {
        self.metadata_if_exists()
            .await
            .map(|metadata_opt| metadata_opt.is_some())
    }

    async fn try_is_file_async(&self) -> io::Result<bool> {
        self.metadata_if_exists().await.map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_file())
                .unwrap_or(false)
        })
    }

    async fn try_is_dir_async(&self) -> io::Result<bool> {
        self.metadata_if_exists().await.map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_dir())
                .unwrap_or(false)
        })
    }

    async fn try_is_symlink_async(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists().await.map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_symlink())
                .unwrap_or(false)
        })
    }

    async fn metadata_if_exists(&self) -> io::Result<Option<Metadata>> {
        match self.metadata_async().await {
            Ok(metadata) => Ok(Some(metadata)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn symlink_metadata_if_exists(&self) -> io::Result<Option<Metadata>> {
        match self.symlink_metadata_async().await {
            Ok(metadata) => Ok(Some(metadata)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn canonicalize_if_exists(&self) -> io::Result<Option<PathBuf>> {
        match self.canonicalize_async().await {
            Ok(path) => Ok(Some(path)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn read_link_if_exists(&self) -> io::Result<Option<PathBuf>> {
        match self.read_link_async().await {
            Ok(path) => Ok(Some(path)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn read_dir_if_exists(&self) -> io::Result<Option<ReadDir>> {
        match self.read_dir_async().await {
            Ok(entries) => Ok(Some(entries)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn exists_nofollow(&self) -> bool {
        self.symlink_metadata_async().await.is_ok()
    }

    async fn is_file_nofollow(&self) -> bool {
        self.symlink_metadata_async()
            .await
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }

    async fn is_dir_nofollow(&self) -> bool {
        self.symlink_metadata_async()
            .await
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    }

    async fn try_exists_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists()
            .await
            .map(|metadata_opt| metadata_opt.is_some())
    }

    async fn try_is_file_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists().await.map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_file())
                .unwrap_or(false)
        })
    }

    async fn try_is_dir_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists().await.map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_dir())
                .unwrap_or(false)
        })
    }

    async fn is_read_dir_empty(&self) -> io::Result<bool> {
        let mut entries = self.read_dir_async().await?;

        entries
            .next_entry()
            .await
            .map(|entry_opt| entry_opt.is_none())
    }

    async fn is_read_dir_empty_if_exists(&self) -> io::Result<Option<bool>> {
        match self.is_read_dir_empty().await {
            Ok(is_empty) => Ok(Some(is_empty)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn hard_link(&self, link: impl AsRef<Self>) -> io::Result<()> {
        fs::hard_link(self, link).await
    }

    async fn hard_link_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let temp = TempPath::try_from_path(link.as_ref())?;

        self.hard_link(temp.path()).await?;

        temp.persist(link).await
    }

    #[cfg(unix)]
    async fn symlink(&self, link: impl AsRef<Self>) -> io::Result<()> {
        fs::symlink(self, link).await
    }

    #[cfg(unix)]
    async fn symlink_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let temp = TempPath::try_from_path(link.as_ref())?;

        self.symlink(temp.path()).await?;

        temp.persist(link).await
    }

    #[cfg(unix)]
    async fn symlink_absolute(&self, link: impl AsRef<Self>) -> io::Result<()> {
        #[expect(unstable_name_collisions)]
        let absolute = self.absolute()?;

        absolute.symlink(link).await
    }

    #[cfg(unix)]
    async fn symlink_absolute_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        #[expect(unstable_name_collisions)]
        let absolute = self.absolute()?;

        absolute.symlink_atomic(link).await
    }

    #[cfg(unix)]
    async fn symlink_relative(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let base = link.as_ref().base()?;

        let relative = diff_paths(self, base)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to diff paths"))?;

        relative.symlink(link).await
    }

    #[cfg(unix)]
    async fn symlink_relative_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let base = link.as_ref().base()?;

        let relative = diff_paths(self, base)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to diff paths"))?;

        relative.symlink_atomic(link).await
    }

    #[cfg(unix)]
    async fn set_permissions(&self, permissions: Permissions) -> io::Result<()> {
        fs::set_permissions(self, permissions).await
    }

    #[cfg(unix)]
    async fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = fs::metadata(self).await?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions_mode);

        self.set_permissions(permissions).await
    }

    #[cfg(unix)]
    async fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = fs::metadata(self).await?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() | permissions_mode);

        self.set_permissions(permissions).await
    }

    #[cfg(unix)]
    async fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = fs::metadata(self).await?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() & !permissions_mode);

        self.set_permissions(permissions).await
    }
}
