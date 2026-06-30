#[cfg(unix)]
use std::os::unix::{self, fs::PermissionsExt as _};
use std::{
    borrow::Cow,
    ffi::OsStr,
    fs::{self, File, Metadata, Permissions, ReadDir},
    io::{self, Write},
    path::{self, Path, PathBuf},
};

#[cfg(unix)]
use pathdiff::diff_paths;

use super::{fs::FileExt, temp_path::TempPath};

pub trait PathExt {
    fn base(&self) -> io::Result<&Path>;
    fn with_base(&self, base: impl AsRef<Path>) -> Cow<'_, Path>;

    fn file_suffix(&self) -> Option<&OsStr>;

    fn absolute(&self) -> io::Result<PathBuf>;

    fn metadata(&self) -> io::Result<Metadata>;
    fn symlink_metadata(&self) -> io::Result<Metadata>;
    fn canonicalize(&self) -> io::Result<PathBuf>;
    fn read_link(&self) -> io::Result<PathBuf>;
    fn read_dir(&self) -> io::Result<ReadDir>;

    fn try_exists(&self) -> io::Result<bool>;

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

    fn try_is_read_dir_empty(&self) -> io::Result<bool>;
    fn try_is_read_dir_empty_if_exists(&self) -> io::Result<Option<bool>>;

    fn create_new(&self) -> io::Result<File>;
    fn create_new_if_not_exists(&self) -> io::Result<Option<File>>;

    fn create(&self) -> io::Result<File>;
    fn create_if_not_exists(&self) -> io::Result<Option<File>>;

    fn open(&self) -> io::Result<File>;
    fn open_if_exists(&self) -> io::Result<Option<File>>;

    fn read(&self) -> io::Result<Vec<u8>>;
    fn read_if_exists(&self) -> io::Result<Option<Vec<u8>>>;

    fn read_to_string(&self) -> io::Result<String>;
    fn read_to_string_if_exists(&self) -> io::Result<Option<String>>;

    fn create_dir_all(self) -> io::Result<Self>
    where
        Self: Sized;

    fn create_dir(self) -> io::Result<Self>
    where
        Self: Sized;
    fn create_dir_if_not_exists(self) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn write_new(self, contents: impl AsRef<[u8]>) -> io::Result<Self>
    where
        Self: Sized;
    fn write_new_if_not_exists(self, contents: impl AsRef<[u8]>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn write(self, contents: impl AsRef<[u8]>) -> io::Result<Self>
    where
        Self: Sized;
    fn write_if_exists(self, contents: impl AsRef<[u8]>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn append(self, contents: impl AsRef<[u8]>) -> io::Result<Self>
    where
        Self: Sized;
    fn append_if_exists(self, contents: impl AsRef<[u8]>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn copy(self, to: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;
    fn copy_if_exists(self, to: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn rename(self, to: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;
    fn rename_if_exists(self, to: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn remove_file(self) -> io::Result<Self>
    where
        Self: Sized;
    fn remove_file_if_exists(self) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn remove_dir(self) -> io::Result<Self>
    where
        Self: Sized;
    fn remove_dir_if_exists(self) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn remove_dir_all(self) -> io::Result<Self>
    where
        Self: Sized;
    fn remove_dir_all_if_exists(self) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn hard_link(self, link: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;
    fn hard_link_if_exists(self, link: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;
    fn hard_link_atomic(self, link: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;
    fn hard_link_atomic_if_exists(self, link: impl AsRef<Path>) -> io::Result<Option<Self>>
    where
        Self: Sized;

    #[cfg(unix)]
    fn symlink(self, link: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;
    #[cfg(unix)]
    fn symlink_atomic(self, link: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;

    #[cfg(unix)]
    fn symlink_absolute(self, link: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;
    #[cfg(unix)]
    fn symlink_absolute_atomic(self, link: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;

    #[cfg(unix)]
    fn symlink_relative(self, link: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;
    #[cfg(unix)]
    fn symlink_relative_atomic(self, link: impl AsRef<Path>) -> io::Result<Self>
    where
        Self: Sized;

    fn set_permissions(self, perm: Permissions) -> io::Result<Self>
    where
        Self: Sized;
    fn set_permissions_if_exists(self, perm: Permissions) -> io::Result<Option<Self>>
    where
        Self: Sized;

    fn set_permissions_readonly(self, readonly: bool) -> io::Result<Self>
    where
        Self: Sized;
    fn set_permissions_readonly_if_exists(self, readonly: bool) -> io::Result<Option<Self>>
    where
        Self: Sized;

    #[cfg(unix)]
    fn set_permissions_mode(self, mode: u32) -> io::Result<Self>
    where
        Self: Sized;
    #[cfg(unix)]
    fn set_permissions_mode_if_exists(self, mode: u32) -> io::Result<Option<Self>>
    where
        Self: Sized;
    #[cfg(unix)]
    fn add_permissions_mode(self, mode: u32) -> io::Result<Self>
    where
        Self: Sized;
    #[cfg(unix)]
    fn add_permissions_mode_if_exists(self, mode: u32) -> io::Result<Option<Self>>
    where
        Self: Sized;
    #[cfg(unix)]
    fn remove_permissions_mode(self, mode: u32) -> io::Result<Self>
    where
        Self: Sized;
    #[cfg(unix)]
    fn remove_permissions_mode_if_exists(self, mode: u32) -> io::Result<Option<Self>>
    where
        Self: Sized;
}

impl<T: AsRef<Path>> PathExt for T {
    #[inline]
    fn base(&self) -> io::Result<&Path> {
        self.as_ref()
            .parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Path has no parent"))
    }

    #[inline]
    fn with_base(&self, base: impl AsRef<Path>) -> Cow<'_, Path> {
        if self.as_ref().is_relative() {
            let path = base.as_ref().join(self);

            Cow::Owned(path)
        } else {
            Cow::Borrowed(self.as_ref())
        }
    }

    #[inline]
    fn file_suffix(&self) -> Option<&OsStr> {
        let file_name = self.as_ref().file_name()?;
        let file_prefix = self.as_ref().file_prefix()?;

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
    fn absolute(&self) -> io::Result<PathBuf> {
        path::absolute(self)
    }

    #[inline]
    fn metadata(&self) -> io::Result<Metadata> {
        self.as_ref().metadata()
    }

    #[inline]
    fn symlink_metadata(&self) -> io::Result<Metadata> {
        self.as_ref().symlink_metadata()
    }

    #[inline]
    fn canonicalize(&self) -> io::Result<PathBuf> {
        self.as_ref().canonicalize()
    }

    #[inline]
    fn read_link(&self) -> io::Result<PathBuf> {
        self.as_ref().read_link()
    }

    #[inline]
    fn read_dir(&self) -> io::Result<ReadDir> {
        self.as_ref().read_dir()
    }

    #[inline]
    fn try_exists(&self) -> io::Result<bool> {
        self.as_ref().try_exists()
    }

    #[inline]
    fn try_is_file(&self) -> io::Result<bool> {
        self.metadata_if_exists()
            .map(|meta_opt| meta_opt.map(|meta| meta.is_file()).unwrap_or(false))
    }

    #[inline]
    fn try_is_dir(&self) -> io::Result<bool> {
        self.metadata_if_exists()
            .map(|meta_opt| meta_opt.map(|meta| meta.is_dir()).unwrap_or(false))
    }

    #[inline]
    fn try_is_symlink(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists()
            .map(|meta_opt| meta_opt.map(|meta| meta.is_symlink()).unwrap_or(false))
    }

    #[inline]
    fn metadata_if_exists(&self) -> io::Result<Option<Metadata>> {
        match self.metadata() {
            Ok(meta) => Ok(Some(meta)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn symlink_metadata_if_exists(&self) -> io::Result<Option<Metadata>> {
        match self.symlink_metadata() {
            Ok(meta) => Ok(Some(meta)),
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
            .map(|meta| meta.is_file())
            .unwrap_or(false)
    }

    #[inline]
    fn is_dir_nofollow(&self) -> bool {
        self.symlink_metadata()
            .map(|meta| meta.is_dir())
            .unwrap_or(false)
    }

    #[inline]
    fn try_exists_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists()
            .map(|meta_opt| meta_opt.is_some())
    }

    #[inline]
    fn try_is_file_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists()
            .map(|meta_opt| meta_opt.map(|meta| meta.is_file()).unwrap_or(false))
    }

    #[inline]
    fn try_is_dir_nofollow(&self) -> io::Result<bool> {
        self.symlink_metadata_if_exists()
            .map(|meta_opt| meta_opt.map(|meta| meta.is_dir()).unwrap_or(false))
    }

    #[inline]
    fn try_is_read_dir_empty(&self) -> io::Result<bool> {
        let mut entries = self.read_dir()?;

        entries
            .next()
            .transpose()
            .map(|entry_opt| entry_opt.is_none())
    }

    #[inline]
    fn try_is_read_dir_empty_if_exists(&self) -> io::Result<Option<bool>> {
        match self.try_is_read_dir_empty() {
            Ok(is_empty) => Ok(Some(is_empty)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn create_new(&self) -> io::Result<File> {
        File::create_new(self)
    }

    #[inline]
    fn create_new_if_not_exists(&self) -> io::Result<Option<File>> {
        File::create_new_if_not_exists(self)
    }

    #[inline]
    fn create(&self) -> io::Result<File> {
        File::create(self)
    }

    #[inline]
    fn create_if_not_exists(&self) -> io::Result<Option<File>> {
        File::create_if_not_exists(self)
    }

    #[inline]
    fn open(&self) -> io::Result<File> {
        File::open(self)
    }

    #[inline]
    fn open_if_exists(&self) -> io::Result<Option<File>> {
        File::open_if_exists(self)
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
    fn create_dir_all(self) -> io::Result<Self> {
        fs::create_dir_all(self.as_ref())?;

        Ok(self)
    }

    #[inline]
    fn create_dir(self) -> io::Result<Self> {
        fs::create_dir(self.as_ref())?;

        Ok(self)
    }

    #[inline]
    fn create_dir_if_not_exists(self) -> io::Result<Option<Self>> {
        match self.as_ref().create_dir() {
            Ok(_) => Ok(Some(self)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn write_new(self, contents: impl AsRef<[u8]>) -> io::Result<Self> {
        let mut options = File::options();

        options.write(true).create_new(true);

        let mut file = options.open(self.as_ref())?;

        file.write_all(contents.as_ref())?;

        Ok(self)
    }

    #[inline]
    fn write_new_if_not_exists(self, contents: impl AsRef<[u8]>) -> io::Result<Option<Self>> {
        match self.as_ref().write_new(contents) {
            Ok(_) => Ok(Some(self)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn write(self, contents: impl AsRef<[u8]>) -> io::Result<Self> {
        fs::write(self.as_ref(), contents)?;

        Ok(self)
    }

    #[inline]
    fn write_if_exists(self, contents: impl AsRef<[u8]>) -> io::Result<Option<Self>> {
        let mut options = File::options();

        options.write(true).truncate(true);

        match options.open(self.as_ref()) {
            Ok(mut file) => {
                file.write_all(contents.as_ref())?;

                Ok(Some(self))
            },
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn append(self, contents: impl AsRef<[u8]>) -> io::Result<Self> {
        let mut options = File::options();

        options.append(true).create(true);

        let mut file = options.open(self.as_ref())?;

        file.write_all(contents.as_ref())?;

        Ok(self)
    }

    #[inline]
    fn append_if_exists(self, contents: impl AsRef<[u8]>) -> io::Result<Option<Self>> {
        let mut options = File::options();

        options.append(true);

        match options.open(self.as_ref()) {
            Ok(mut file) => {
                file.write_all(contents.as_ref())?;

                Ok(Some(self))
            },
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn copy(self, to: impl AsRef<Path>) -> io::Result<Self> {
        fs::copy(self.as_ref(), to)?;

        Ok(self)
    }

    #[inline]
    fn copy_if_exists(self, to: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match self.as_ref().copy(to) {
            Ok(_) => Ok(Some(self)),
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
    fn rename(self, to: impl AsRef<Path>) -> io::Result<Self> {
        fs::rename(self.as_ref(), to)?;

        Ok(self)
    }

    #[inline]
    fn rename_if_exists(self, to: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match self.as_ref().rename(to) {
            Ok(_) => Ok(Some(self)),
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
    fn remove_file(self) -> io::Result<Self> {
        fs::remove_file(self.as_ref())?;

        Ok(self)
    }

    #[inline]
    fn remove_file_if_exists(self) -> io::Result<Option<Self>> {
        match self.as_ref().remove_file() {
            Ok(_) => Ok(Some(self)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn remove_dir(self) -> io::Result<Self> {
        fs::remove_dir(self.as_ref())?;

        Ok(self)
    }

    #[inline]
    fn remove_dir_if_exists(self) -> io::Result<Option<Self>> {
        match self.as_ref().remove_dir() {
            Ok(_) => Ok(Some(self)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn remove_dir_all(self) -> io::Result<Self> {
        fs::remove_dir_all(self.as_ref())?;

        Ok(self)
    }

    #[inline]
    fn remove_dir_all_if_exists(self) -> io::Result<Option<Self>> {
        match self.as_ref().remove_dir_all() {
            Ok(_) => Ok(Some(self)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn hard_link(self, link: impl AsRef<Path>) -> io::Result<Self> {
        fs::hard_link(self.as_ref(), link)?;

        Ok(self)
    }

    #[inline]
    fn hard_link_if_exists(self, link: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match self.as_ref().hard_link(link) {
            Ok(_) => Ok(Some(self)),
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
    fn hard_link_atomic(self, link: impl AsRef<Path>) -> io::Result<Self> {
        let temp = TempPath::try_from_path(link.as_ref())?;

        self.as_ref().hard_link(temp.path())?;

        temp.persist(link)?;

        Ok(self)
    }

    #[inline]
    fn hard_link_atomic_if_exists(self, link: impl AsRef<Path>) -> io::Result<Option<Self>> {
        match self.as_ref().hard_link_atomic(link) {
            Ok(_) => Ok(Some(self)),
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

    #[cfg(unix)]
    #[inline]
    fn symlink(self, link: impl AsRef<Path>) -> io::Result<Self> {
        unix::fs::symlink(self.as_ref(), link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_atomic(self, link: impl AsRef<Path>) -> io::Result<Self> {
        let temp = TempPath::try_from_path(link.as_ref())?;

        self.as_ref().symlink(temp.path())?;

        temp.persist(link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_absolute(self, link: impl AsRef<Path>) -> io::Result<Self> {
        #[expect(unstable_name_collisions)]
        let absolute = self.as_ref().absolute()?;

        absolute.symlink(link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_absolute_atomic(self, link: impl AsRef<Path>) -> io::Result<Self> {
        #[expect(unstable_name_collisions)]
        let absolute = self.as_ref().absolute()?;

        absolute.symlink_atomic(link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_relative(self, link: impl AsRef<Path>) -> io::Result<Self> {
        let base = link.base()?;

        let relative = diff_paths(self.as_ref(), base)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to diff paths"))?;

        relative.symlink(link)?;

        Ok(self)
    }

    #[cfg(unix)]
    #[inline]
    fn symlink_relative_atomic(self, link: impl AsRef<Path>) -> io::Result<Self> {
        let base = link.base()?;

        let relative = diff_paths(self.as_ref(), base)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Failed to diff paths"))?;

        relative.symlink_atomic(link)?;

        Ok(self)
    }

    #[inline]
    fn set_permissions(self, perm: Permissions) -> io::Result<Self> {
        fs::set_permissions(self.as_ref(), perm)?;

        Ok(self)
    }

    #[inline]
    fn set_permissions_if_exists(self, perm: Permissions) -> io::Result<Option<Self>> {
        match self.as_ref().set_permissions(perm) {
            Ok(_) => Ok(Some(self)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn set_permissions_readonly(self, readonly: bool) -> io::Result<Self> {
        let meta = self.metadata()?;

        let mut perm = meta.permissions();

        perm.set_readonly(readonly);

        self.set_permissions(perm)
    }

    #[inline]
    fn set_permissions_readonly_if_exists(self, readonly: bool) -> io::Result<Option<Self>> {
        match self.as_ref().set_permissions_readonly(readonly) {
            Ok(_) => Ok(Some(self)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[cfg(unix)]
    #[inline]
    fn set_permissions_mode(self, mode: u32) -> io::Result<Self> {
        let perm = Permissions::from_mode(mode);

        self.set_permissions(perm)
    }

    #[cfg(unix)]
    #[inline]
    fn set_permissions_mode_if_exists(self, mode: u32) -> io::Result<Option<Self>> {
        match self.as_ref().set_permissions_mode(mode) {
            Ok(_) => Ok(Some(self)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[cfg(unix)]
    #[inline]
    fn add_permissions_mode(self, mode: u32) -> io::Result<Self> {
        let meta = self.metadata()?;

        let perm = meta.permissions();

        self.set_permissions_mode(perm.mode() | mode)
    }

    #[cfg(unix)]
    #[inline]
    fn add_permissions_mode_if_exists(self, mode: u32) -> io::Result<Option<Self>> {
        match self.as_ref().add_permissions_mode(mode) {
            Ok(_) => Ok(Some(self)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[cfg(unix)]
    #[inline]
    fn remove_permissions_mode(self, mode: u32) -> io::Result<Self> {
        let meta = self.metadata()?;

        let perm = meta.permissions();

        self.set_permissions_mode(perm.mode() & !mode)
    }

    #[cfg(unix)]
    #[inline]
    fn remove_permissions_mode_if_exists(self, mode: u32) -> io::Result<Option<Self>> {
        match self.as_ref().remove_permissions_mode(mode) {
            Ok(_) => Ok(Some(self)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }
}

macro_rules! define_option_path_ext {
    (
        $(
            $(#[$meta:meta])*
            fn $method:ident(self $(, $arg:ident: $ty:ty)* $(,)*) -> $ret:ty;
        )*
    ) => {
        pub trait OptionPathExt<T> {
            $(
                $(#[$meta])*
                fn $method(self $(, $arg: $ty)*) -> $ret;
            )*
        }

        impl<T: PathExt> OptionPathExt<T> for Option<T> {
            $(
                #[inline]
                $(#[$meta])*
                fn $method(self $(, $arg: $ty)*) -> $ret {
                    match self {
                        Some(path) => path.$method($($arg),*).map(Some),
                        None => Ok(None),
                    }
                }
            )*
        }
    };
}

define_option_path_ext! {
    fn metadata(self) -> io::Result<Option<Metadata>>;
    fn symlink_metadata(self) -> io::Result<Option<Metadata>>;
    fn canonicalize(self) -> io::Result<Option<PathBuf>>;
    fn read_link(self) -> io::Result<Option<PathBuf>>;
    fn read_dir(self) -> io::Result<Option<ReadDir>>;

    fn try_exists(self) -> io::Result<Option<bool>>;

    fn try_is_file(self) -> io::Result<Option<bool>>;
    fn try_is_dir(self) -> io::Result<Option<bool>>;
    fn try_is_symlink(self) -> io::Result<Option<bool>>;

    fn try_exists_nofollow(self) -> io::Result<Option<bool>>;
    fn try_is_file_nofollow(self) -> io::Result<Option<bool>>;
    fn try_is_dir_nofollow(self) -> io::Result<Option<bool>>;

    fn try_is_read_dir_empty(self) -> io::Result<Option<bool>>;

    fn create_new(self) -> io::Result<Option<File>>;

    fn create(self) -> io::Result<Option<File>>;

    fn open(self) -> io::Result<Option<File>>;

    fn read(self) -> io::Result<Option<Vec<u8>>>;

    fn read_to_string(self) -> io::Result<Option<String>>;

    fn create_dir_all(self) -> io::Result<Option<T>>;

    fn create_dir(self) -> io::Result<Option<T>>;

    fn write_new(self, contents: impl AsRef<[u8]>) -> io::Result<Option<T>>;

    fn write(self, contents: impl AsRef<[u8]>) -> io::Result<Option<T>>;

    fn append(self, contents: impl AsRef<[u8]>) -> io::Result<Option<T>>;

    fn copy(self, to: impl AsRef<Path>) -> io::Result<Option<T>>;

    fn rename(self, to: impl AsRef<Path>) -> io::Result<Option<T>>;

    fn remove_file(self) -> io::Result<Option<T>>;

    fn remove_dir(self) -> io::Result<Option<T>>;

    fn remove_dir_all(self) -> io::Result<Option<T>>;

    fn hard_link(self, link: impl AsRef<Path>) -> io::Result<Option<T>>;
    fn hard_link_atomic(self, link: impl AsRef<Path>) -> io::Result<Option<T>>;

    #[cfg(unix)]
    fn symlink(self, link: impl AsRef<Path>) -> io::Result<Option<T>>;
    #[cfg(unix)]
    fn symlink_atomic(self, link: impl AsRef<Path>) -> io::Result<Option<T>>;

    #[cfg(unix)]
    fn symlink_absolute(self, link: impl AsRef<Path>) -> io::Result<Option<T>>;
    #[cfg(unix)]
    fn symlink_absolute_atomic(self, link: impl AsRef<Path>) -> io::Result<Option<T>>;

    #[cfg(unix)]
    fn symlink_relative(self, link: impl AsRef<Path>) -> io::Result<Option<T>>;
    #[cfg(unix)]
    fn symlink_relative_atomic(self, link: impl AsRef<Path>) -> io::Result<Option<T>>;

    fn set_permissions(self, perm: Permissions) -> io::Result<Option<T>>;

    fn set_permissions_readonly(self, readonly: bool) -> io::Result<Option<T>>;

    #[cfg(unix)]
    fn set_permissions_mode(self, mode: u32) -> io::Result<Option<T>>;
    #[cfg(unix)]
    fn add_permissions_mode(self, mode: u32) -> io::Result<Option<T>>;
    #[cfg(unix)]
    fn remove_permissions_mode(self, mode: u32) -> io::Result<Option<T>>;
}
