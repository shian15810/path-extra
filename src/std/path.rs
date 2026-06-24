use std::{
    borrow::Cow,
    ffi::OsStr,
    fs::{self, Metadata, ReadDir},
    io,
    path::{self, Path, PathBuf},
};
#[cfg(unix)]
use std::{fs::Permissions, os::unix::fs::PermissionsExt as _};

#[cfg(unix)]
use pathdiff::diff_paths;

use super::temp_path::TempPath;

pub trait PathExt {
    fn absolute(&self) -> io::Result<PathBuf>;

    fn base(&self) -> io::Result<&Self>;
    fn with_base(&self, base: impl AsRef<Self>) -> Cow<'_, Self>
    where
        Self: ToOwned;

    fn file_suffix(&self) -> Option<&OsStr>;

    fn try_is_file(&self) -> io::Result<bool>;
    fn try_is_dir(&self) -> io::Result<bool>;
    fn try_is_symlink(&self) -> io::Result<bool>;

    fn metadata_if_exists(&self) -> io::Result<Option<Metadata>>;
    fn symlink_metadata_if_exists(&self) -> io::Result<Option<Metadata>>;
    fn canonicalize_if_exists(&self) -> io::Result<Option<PathBuf>>;
    fn read_link_if_exists(&self) -> io::Result<Option<PathBuf>>;
    fn read_dir_if_exists(&self) -> io::Result<Option<ReadDir>>;

    fn exists_nofollow(&self) -> bool;
    fn is_file_nofollow(&self) -> bool;
    fn is_dir_nofollow(&self) -> bool;

    fn try_exists_nofollow(&self) -> io::Result<bool>;
    fn try_is_file_nofollow(&self) -> io::Result<bool>;
    fn try_is_dir_nofollow(&self) -> io::Result<bool>;

    fn is_read_dir_empty(&self) -> io::Result<bool>;
    fn is_read_dir_empty_if_exists(&self) -> io::Result<Option<bool>>;

    fn hard_link(&self, link: impl AsRef<Self>) -> io::Result<()>;
    fn hard_link_atomic(&self, link: impl AsRef<Self>) -> io::Result<()>;

    #[cfg(unix)]
    fn symlink(&self, link: impl AsRef<Self>) -> io::Result<()>;
    #[cfg(unix)]
    fn symlink_atomic(&self, link: impl AsRef<Self>) -> io::Result<()>;

    #[cfg(unix)]
    fn symlink_absolute(&self, link: impl AsRef<Self>) -> io::Result<()>;
    #[cfg(unix)]
    fn symlink_absolute_atomic(&self, link: impl AsRef<Self>) -> io::Result<()>;

    #[cfg(unix)]
    fn symlink_relative(&self, link: impl AsRef<Self>) -> io::Result<()>;
    #[cfg(unix)]
    fn symlink_relative_atomic(&self, link: impl AsRef<Self>) -> io::Result<()>;

    #[cfg(unix)]
    fn set_permissions(&self, permissions: Permissions) -> io::Result<()>;
    #[cfg(unix)]
    fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
    #[cfg(unix)]
    fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
    #[cfg(unix)]
    fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()>;
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

    fn try_is_file(&self) -> io::Result<bool> {
        self.metadata_if_exists().map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_file())
                .unwrap_or(false)
        })
    }

    fn try_is_dir(&self) -> io::Result<bool> {
        self.metadata_if_exists().map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_dir())
                .unwrap_or(false)
        })
    }

    fn try_is_symlink(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists().map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_symlink())
                .unwrap_or(false)
        })
    }

    fn metadata_if_exists(&self) -> io::Result<Option<Metadata>> {
        match self.metadata() {
            Ok(metadata) => Ok(Some(metadata)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn symlink_metadata_if_exists(&self) -> io::Result<Option<Metadata>> {
        match self.symlink_metadata() {
            Ok(metadata) => Ok(Some(metadata)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn canonicalize_if_exists(&self) -> io::Result<Option<PathBuf>> {
        match self.canonicalize() {
            Ok(path) => Ok(Some(path)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn read_link_if_exists(&self) -> io::Result<Option<PathBuf>> {
        match self.read_link() {
            Ok(path) => Ok(Some(path)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn read_dir_if_exists(&self) -> io::Result<Option<ReadDir>> {
        match self.read_dir() {
            Ok(entries) => Ok(Some(entries)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn exists_nofollow(&self) -> bool {
        self.symlink_metadata().is_ok()
    }

    fn is_file_nofollow(&self) -> bool {
        self.symlink_metadata()
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }

    fn is_dir_nofollow(&self) -> bool {
        self.symlink_metadata()
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    }

    fn try_exists_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists()
            .map(|metadata_opt| metadata_opt.is_some())
    }

    fn try_is_file_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists().map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_file())
                .unwrap_or(false)
        })
    }

    fn try_is_dir_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists().map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_dir())
                .unwrap_or(false)
        })
    }

    fn is_read_dir_empty(&self) -> io::Result<bool> {
        let mut entries = self.read_dir()?;

        entries
            .next()
            .transpose()
            .map(|entry_opt| entry_opt.is_none())
    }

    fn is_read_dir_empty_if_exists(&self) -> io::Result<Option<bool>> {
        match self.is_read_dir_empty() {
            Ok(is_empty) => Ok(Some(is_empty)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn hard_link(&self, link: impl AsRef<Self>) -> io::Result<()> {
        fs::hard_link(self, link)
    }

    fn hard_link_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let temp = TempPath::try_from_path(link.as_ref())?;

        self.hard_link(temp.path())?;

        temp.persist(link)
    }

    #[cfg(unix)]
    fn symlink(&self, link: impl AsRef<Self>) -> io::Result<()> {
        use std::os::unix::fs;

        fs::symlink(self, link)
    }

    #[cfg(unix)]
    fn symlink_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let temp = TempPath::try_from_path(link.as_ref())?;

        self.symlink(temp.path())?;

        temp.persist(link)
    }

    #[cfg(unix)]
    fn symlink_absolute(&self, link: impl AsRef<Self>) -> io::Result<()> {
        #[expect(unstable_name_collisions)]
        let absolute = self.absolute()?;

        absolute.symlink(link)
    }

    #[cfg(unix)]
    fn symlink_absolute_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        #[expect(unstable_name_collisions)]
        let absolute = self.absolute()?;

        absolute.symlink_atomic(link)
    }

    #[cfg(unix)]
    fn symlink_relative(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let base = link.as_ref().base()?;

        let relative = diff_paths(self, base)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to diff paths"))?;

        relative.symlink(link)
    }

    #[cfg(unix)]
    fn symlink_relative_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let base = link.as_ref().base()?;

        let relative = diff_paths(self, base)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to diff paths"))?;

        relative.symlink_atomic(link)
    }

    #[cfg(unix)]
    fn set_permissions(&self, permissions: Permissions) -> io::Result<()> {
        fs::set_permissions(self, permissions)
    }

    #[cfg(unix)]
    fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = fs::metadata(self)?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions_mode);

        self.set_permissions(permissions)
    }

    #[cfg(unix)]
    fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = fs::metadata(self)?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() | permissions_mode);

        self.set_permissions(permissions)
    }

    #[cfg(unix)]
    fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = fs::metadata(self)?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() & !permissions_mode);

        self.set_permissions(permissions)
    }
}
