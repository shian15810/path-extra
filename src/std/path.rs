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

    fn create_dir_all(&self) -> io::Result<()>;

    fn create_dir(&self) -> io::Result<()>;
    fn create_dir_if_not_exists(&self) -> io::Result<Option<()>>;

    fn write_new(&self, contents: impl AsRef<[u8]>) -> io::Result<()>;
    fn write_new_if_not_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<()>>;

    fn write(&self, contents: impl AsRef<[u8]>) -> io::Result<()>;
    fn write_if_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<()>>;

    fn append(&self, contents: impl AsRef<[u8]>) -> io::Result<()>;
    fn append_if_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<()>>;

    fn read(&self) -> io::Result<Vec<u8>>;
    fn read_if_exists(&self) -> io::Result<Option<Vec<u8>>>;

    fn read_to_string(&self) -> io::Result<String>;
    fn read_to_string_if_exists(&self) -> io::Result<Option<String>>;

    fn copy(&self, to: impl AsRef<Self>) -> io::Result<u64>;
    fn copy_if_exists(&self, to: impl AsRef<Self>) -> io::Result<Option<u64>>;

    fn rename(&self, to: impl AsRef<Self>) -> io::Result<()>;
    fn rename_if_exists(&self, to: impl AsRef<Self>) -> io::Result<Option<()>>;

    fn remove_file(&self) -> io::Result<()>;
    fn remove_file_if_exists(&self) -> io::Result<Option<()>>;

    fn remove_dir(&self) -> io::Result<()>;
    fn remove_dir_if_exists(&self) -> io::Result<Option<()>>;

    fn remove_dir_all(&self) -> io::Result<()>;
    fn remove_dir_all_if_exists(&self) -> io::Result<Option<()>>;

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
    fn create_dir_all(&self) -> io::Result<()> {
        fs::create_dir_all(self)
    }

    #[inline]
    fn create_dir(&self) -> io::Result<()> {
        fs::create_dir(self)
    }

    #[inline]
    fn create_dir_if_not_exists(&self) -> io::Result<Option<()>> {
        match self.create_dir() {
            Ok(()) => Ok(Some(())),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn write_new(&self, contents: impl AsRef<[u8]>) -> io::Result<()> {
        let mut options = File::options();

        options.write(true).create_new(true);

        let mut file = options.open(self)?;

        file.write_all(contents.as_ref())
    }

    #[inline]
    fn write_new_if_not_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<()>> {
        match self.write_new(contents) {
            Ok(()) => Ok(Some(())),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn write(&self, contents: impl AsRef<[u8]>) -> io::Result<()> {
        fs::write(self, contents)
    }

    #[inline]
    fn write_if_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<()>> {
        let mut options = File::options();

        options.write(true).truncate(true);

        match options.open(self) {
            Ok(mut file) => {
                file.write_all(contents.as_ref())?;

                Ok(Some(()))
            },
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn append(&self, contents: impl AsRef<[u8]>) -> io::Result<()> {
        let mut options = File::options();

        options.append(true).create(true);

        let mut file = options.open(self)?;

        file.write_all(contents.as_ref())
    }

    #[inline]
    fn append_if_exists(&self, contents: impl AsRef<[u8]>) -> io::Result<Option<()>> {
        let mut options = File::options();

        options.append(true);

        match options.open(self) {
            Ok(mut file) => {
                file.write_all(contents.as_ref())?;

                Ok(Some(()))
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
    fn copy(&self, to: impl AsRef<Self>) -> io::Result<u64> {
        fs::copy(self, to)
    }

    #[inline]
    fn copy_if_exists(&self, to: impl AsRef<Self>) -> io::Result<Option<u64>> {
        match self.copy(to) {
            Ok(bytes_copied) => Ok(Some(bytes_copied)),
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
    fn rename(&self, to: impl AsRef<Self>) -> io::Result<()> {
        fs::rename(self, to)
    }

    #[inline]
    fn rename_if_exists(&self, to: impl AsRef<Self>) -> io::Result<Option<()>> {
        match self.rename(to) {
            Ok(()) => Ok(Some(())),
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
    fn remove_file(&self) -> io::Result<()> {
        fs::remove_file(self)
    }

    #[inline]
    fn remove_file_if_exists(&self) -> io::Result<Option<()>> {
        match self.remove_file() {
            Ok(()) => Ok(Some(())),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn remove_dir(&self) -> io::Result<()> {
        fs::remove_dir(self)
    }

    #[inline]
    fn remove_dir_if_exists(&self) -> io::Result<Option<()>> {
        match self.remove_dir() {
            Ok(()) => Ok(Some(())),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn remove_dir_all(&self) -> io::Result<()> {
        fs::remove_dir_all(self)
    }

    #[inline]
    fn remove_dir_all_if_exists(&self) -> io::Result<Option<()>> {
        match self.remove_dir_all() {
            Ok(()) => Ok(Some(())),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn hard_link(&self, link: impl AsRef<Self>) -> io::Result<()> {
        fs::hard_link(self, link)
    }

    #[inline]
    fn hard_link_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let temp = TempPath::try_from_path(link.as_ref())?;

        self.hard_link(temp.path())?;

        temp.persist(link)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink(&self, link: impl AsRef<Self>) -> io::Result<()> {
        unix::fs::symlink(self, link)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let temp = TempPath::try_from_path(link.as_ref())?;

        self.symlink(temp.path())?;

        temp.persist(link)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_absolute(&self, link: impl AsRef<Self>) -> io::Result<()> {
        #[expect(unstable_name_collisions)]
        let absolute = self.absolute()?;

        absolute.symlink(link)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_absolute_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        #[expect(unstable_name_collisions)]
        let absolute = self.absolute()?;

        absolute.symlink_atomic(link)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_relative(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let base = link.as_ref().base()?;

        let relative = diff_paths(self, base)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to diff paths"))?;

        relative.symlink(link)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_relative_atomic(&self, link: impl AsRef<Self>) -> io::Result<()> {
        let base = link.as_ref().base()?;

        let relative = diff_paths(self, base)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to diff paths"))?;

        relative.symlink_atomic(link)
    }

    #[cfg(unix)]
    #[inline]
    fn set_permissions(&self, permissions: Permissions) -> io::Result<()> {
        fs::set_permissions(self, permissions)
    }

    #[cfg(unix)]
    #[inline]
    fn set_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = fs::metadata(self)?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions_mode);

        self.set_permissions(permissions)
    }

    #[cfg(unix)]
    #[inline]
    fn add_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = fs::metadata(self)?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() | permissions_mode);

        self.set_permissions(permissions)
    }

    #[cfg(unix)]
    #[inline]
    fn remove_permissions_mode(&self, permissions_mode: u32) -> io::Result<()> {
        let metadata = fs::metadata(self)?;

        let mut permissions = metadata.permissions();

        permissions.set_mode(permissions.mode() & !permissions_mode);

        self.set_permissions(permissions)
    }
}
