use std::{
    borrow::Cow,
    ffi::OsStr,
    fs::{self, File, Metadata, ReadDir},
    io::{self, Write},
    path::{self, Path, PathBuf},
};
#[cfg(unix)]
use std::{
    fs::Permissions,
    os::unix::{self, fs::PermissionsExt as _},
};

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

    fn create_dir_all(&self) -> io::Result<&Self>;

    fn create_dir(&self) -> io::Result<&Self>;
    fn create_dir_if_not_exists(&self) -> io::Result<Option<&Self>>;

    fn write_new(&self, contents: impl AsRef<[u8]>) -> io::Result<&Self>;
    fn write_new_if_not_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<&Self>>;

    fn write(&self, contents: impl AsRef<[u8]>) -> io::Result<&Self>;
    fn write_if_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<&Self>>;

    fn append(&self, contents: impl AsRef<[u8]>) -> io::Result<&Self>;
    fn append_if_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<&Self>>;

    fn read(&self) -> io::Result<Vec<u8>>;
    fn read_if_exists(&self) -> io::Result<Option<Vec<u8>>>;

    fn read_to_string(&self) -> io::Result<String>;
    fn read_to_string_if_exists(&self) -> io::Result<Option<String>>;

    fn copy(&self, to: impl AsRef<Self>) -> io::Result<&Self>;
    fn copy_if_exists(&self, to: impl AsRef<Self>) -> io::Result<Option<&Self>>;

    fn rename(&self, to: impl AsRef<Self>) -> io::Result<&Self>;
    fn rename_if_exists(&self, to: impl AsRef<Self>) -> io::Result<Option<&Self>>;

    fn remove_file(&self) -> io::Result<&Self>;
    fn remove_file_if_exists(&self) -> io::Result<Option<&Self>>;

    fn remove_dir(&self) -> io::Result<&Self>;
    fn remove_dir_if_exists(&self) -> io::Result<Option<&Self>>;

    fn remove_dir_all(&self) -> io::Result<&Self>;
    fn remove_dir_all_if_exists(&self) -> io::Result<Option<&Self>>;

    fn hard_link(&self, link: impl AsRef<Self>) -> io::Result<&Self>;
    fn hard_link_atomic(&self, link: impl AsRef<Self>) -> io::Result<&Self>;

    #[cfg(unix)]
    fn symlink(&self, link: impl AsRef<Self>) -> io::Result<&Self>;
    #[cfg(unix)]
    fn symlink_atomic(&self, link: impl AsRef<Self>) -> io::Result<&Self>;

    #[cfg(unix)]
    fn symlink_absolute(&self, link: impl AsRef<Self>) -> io::Result<&Self>;
    #[cfg(unix)]
    fn symlink_absolute_atomic(&self, link: impl AsRef<Self>) -> io::Result<&Self>;

    #[cfg(unix)]
    fn symlink_relative(&self, link: impl AsRef<Self>) -> io::Result<&Self>;
    #[cfg(unix)]
    fn symlink_relative_atomic(&self, link: impl AsRef<Self>) -> io::Result<&Self>;

    #[cfg(unix)]
    fn set_permissions(&self, permissions: Permissions) -> io::Result<&Self>;

    #[cfg(unix)]
    fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<&Self>;
    #[cfg(unix)]
    fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<&Self>;
    #[cfg(unix)]
    fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<&Self>;
}

impl PathExt for Path {
    #[inline]
    fn absolute(&self) -> io::Result<PathBuf> {
        path::absolute(self)
    }

    #[inline]
    fn base(&self) -> io::Result<&Self> {
        self.parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Path has no parent"))
    }

    #[inline]
    fn with_base(&self, base: impl AsRef<Self>) -> Cow<'_, Self> {
        if self.is_relative() {
            let path = base.as_ref().join(self);

            Cow::Owned(path)
        } else {
            Cow::Borrowed(self)
        }
    }

    #[inline]
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

    #[inline]
    fn try_is_file(&self) -> io::Result<bool> {
        self.metadata_if_exists().map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_file())
                .unwrap_or(false)
        })
    }

    #[inline]
    fn try_is_dir(&self) -> io::Result<bool> {
        self.metadata_if_exists().map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_dir())
                .unwrap_or(false)
        })
    }

    #[inline]
    fn try_is_symlink(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists().map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_symlink())
                .unwrap_or(false)
        })
    }

    #[inline]
    fn metadata_if_exists(&self) -> io::Result<Option<Metadata>> {
        match self.metadata() {
            Ok(metadata) => Ok(Some(metadata)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn symlink_metadata_if_exists(&self) -> io::Result<Option<Metadata>> {
        match self.symlink_metadata() {
            Ok(metadata) => Ok(Some(metadata)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn canonicalize_if_exists(&self) -> io::Result<Option<PathBuf>> {
        match self.canonicalize() {
            Ok(path) => Ok(Some(path)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_link_if_exists(&self) -> io::Result<Option<PathBuf>> {
        match self.read_link() {
            Ok(path) => Ok(Some(path)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_dir_if_exists(&self) -> io::Result<Option<ReadDir>> {
        match self.read_dir() {
            Ok(entries) => Ok(Some(entries)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn exists_nofollow(&self) -> bool {
        self.symlink_metadata().is_ok()
    }

    #[inline]
    fn is_file_nofollow(&self) -> bool {
        self.symlink_metadata()
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }

    #[inline]
    fn is_dir_nofollow(&self) -> bool {
        self.symlink_metadata()
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    }

    #[inline]
    fn try_exists_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists()
            .map(|metadata_opt| metadata_opt.is_some())
    }

    #[inline]
    fn try_is_file_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists().map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_file())
                .unwrap_or(false)
        })
    }

    #[inline]
    fn try_is_dir_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists().map(|metadata_opt| {
            metadata_opt
                .map(|metadata| metadata.is_dir())
                .unwrap_or(false)
        })
    }

    #[inline]
    fn is_read_dir_empty(&self) -> io::Result<bool> {
        let mut entries = self.read_dir()?;

        entries
            .next()
            .transpose()
            .map(|entry_opt| entry_opt.is_none())
    }

    #[inline]
    fn is_read_dir_empty_if_exists(&self) -> io::Result<Option<bool>> {
        match self.is_read_dir_empty() {
            Ok(is_empty) => Ok(Some(is_empty)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn create_dir_all(&self) -> io::Result<&Self> {
        fs::create_dir_all(self)?;

        Ok(self)
    }

    #[inline]
    fn create_dir(&self) -> io::Result<&Self> {
        fs::create_dir(self)?;

        Ok(self)
    }

    #[inline]
    fn create_dir_if_not_exists(&self) -> io::Result<Option<&Self>> {
        match self.create_dir() {
            Ok(this) => Ok(Some(this)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn write_new(&self, contents: impl AsRef<[u8]>) -> io::Result<&Self> {
        let mut options = File::options();

        options.write(true).create_new(true);

        let mut file = options.open(self)?;

        file.write_all(contents.as_ref())?;

        Ok(self)
    }

    #[inline]
    fn write_new_if_not_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<&Self>> {
        match self.write_new(contents) {
            Ok(this) => Ok(Some(this)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn write(&self, contents: impl AsRef<[u8]>) -> io::Result<&Self> {
        fs::write(self, contents)?;

        Ok(self)
    }

    #[inline]
    fn write_if_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<&Self>> {
        let mut options = File::options();

        options.write(true).truncate(true);

        match options.open(self) {
            Ok(mut file) => {
                file.write_all(contents.as_ref())?;

                Ok(Some(self))
            },
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn append(&self, contents: impl AsRef<[u8]>) -> io::Result<&Self> {
        let mut options = File::options();

        options.append(true).create(true);

        let mut file = options.open(self)?;

        file.write_all(contents.as_ref())?;

        Ok(self)
    }

    #[inline]
    fn append_if_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<&Self>> {
        let mut options = File::options();

        options.append(true);

        match options.open(self) {
            Ok(mut file) => {
                file.write_all(contents.as_ref())?;

                Ok(Some(self))
            },
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read(&self) -> io::Result<Vec<u8>> {
        fs::read(self)
    }

    #[inline]
    fn read_if_exists(&self) -> io::Result<Option<Vec<u8>>> {
        match self.read() {
            Ok(bytes) => Ok(Some(bytes)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_to_string(&self) -> io::Result<String> {
        fs::read_to_string(self)
    }

    #[inline]
    fn read_to_string_if_exists(&self) -> io::Result<Option<String>> {
        match self.read_to_string() {
            Ok(string) => Ok(Some(string)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn copy(&self, to: impl AsRef<Self>) -> io::Result<&Self> {
        fs::copy(self, to)?;

        Ok(self)
    }

    #[inline]
    fn copy_if_exists(&self, to: impl AsRef<Self>) -> io::Result<Option<&Self>> {
        match self.copy(to) {
            Ok(this) => Ok(Some(this)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                if !self.try_exists()? {
                    Ok(None)
                } else {
                    Err(err)
                }
            },
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn rename(&self, to: impl AsRef<Self>) -> io::Result<&Self> {
        fs::rename(self, to)?;

        Ok(self)
    }

    #[inline]
    fn rename_if_exists(&self, to: impl AsRef<Self>) -> io::Result<Option<&Self>> {
        match self.rename(to) {
            Ok(this) => Ok(Some(this)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                if !self.try_exists_nofollow()? {
                    Ok(None)
                } else {
                    Err(err)
                }
            },
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn remove_file(&self) -> io::Result<&Self> {
        fs::remove_file(self)?;

        Ok(self)
    }

    #[inline]
    fn remove_file_if_exists(&self) -> io::Result<Option<&Self>> {
        match self.remove_file() {
            Ok(this) => Ok(Some(this)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn remove_dir(&self) -> io::Result<&Self> {
        fs::remove_dir(self)?;

        Ok(self)
    }

    #[inline]
    fn remove_dir_if_exists(&self) -> io::Result<Option<&Self>> {
        match self.remove_dir() {
            Ok(this) => Ok(Some(this)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn remove_dir_all(&self) -> io::Result<&Self> {
        fs::remove_dir_all(self)?;

        Ok(self)
    }

    #[inline]
    fn remove_dir_all_if_exists(&self) -> io::Result<Option<&Self>> {
        match self.remove_dir_all() {
            Ok(this) => Ok(Some(this)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn hard_link(&self, link: impl AsRef<Self>) -> io::Result<&Self> {
        fs::hard_link(self, link)?;

        Ok(self)
    }

    #[inline]
    fn hard_link_atomic(&self, link: impl AsRef<Self>) -> io::Result<&Self> {
        let temp = TempPath::try_from_path(link.as_ref())?;

        self.hard_link(temp.path())?;

        temp.persist(link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink(&self, link: impl AsRef<Self>) -> io::Result<&Self> {
        unix::fs::symlink(self, link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_atomic(&self, link: impl AsRef<Self>) -> io::Result<&Self> {
        let temp = TempPath::try_from_path(link.as_ref())?;

        self.symlink(temp.path())?;

        temp.persist(link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_absolute(&self, link: impl AsRef<Self>) -> io::Result<&Self> {
        #[expect(unstable_name_collisions)]
        let absolute = self.absolute()?;

        absolute.symlink(link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_absolute_atomic(&self, link: impl AsRef<Self>) -> io::Result<&Self> {
        #[expect(unstable_name_collisions)]
        let absolute = self.absolute()?;

        absolute.symlink_atomic(link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_relative(&self, link: impl AsRef<Self>) -> io::Result<&Self> {
        let base = link.as_ref().base()?;

        let relative = diff_paths(self, base)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to diff paths"))?;

        relative.symlink(link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_relative_atomic(&self, link: impl AsRef<Self>) -> io::Result<&Self> {
        let base = link.as_ref().base()?;

        let relative = diff_paths(self, base)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to diff paths"))?;

        relative.symlink_atomic(link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn set_permissions(&self, permissions: Permissions) -> io::Result<&Self> {
        fs::set_permissions(self, permissions)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<&Self> {
        let metadata = fs::metadata(self)?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions_mode);

        self.set_permissions(permissions)
    }

    #[cfg(unix)]
    #[inline]
    fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<&Self> {
        let metadata = fs::metadata(self)?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() | permissions_mode);

        self.set_permissions(permissions)
    }

    #[cfg(unix)]
    #[inline]
    fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<&Self> {
        let metadata = fs::metadata(self)?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() & !permissions_mode);

        self.set_permissions(permissions)
    }
}
