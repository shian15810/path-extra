# path-extra

[![github.com](https://img.shields.io/badge/path--extra-blue?logo=github&label=github.com)](https://github.com/shian15810/path-extra)
[![crates.io](https://img.shields.io/crates/v/path-extra?logo=rust&label=crates.io)](https://crates.io/crates/path-extra)
[![docs.rs](https://img.shields.io/docsrs/path-extra?logo=docsdotrs&label=docs.rs)](https://docs.rs/path-extra)

Extra methods on `Path` for ergonomic and chainable `fs` operations.

Stop juggling `fs` functions, start chaining `Path` methods.

Supports `std` (default) and `tokio` (feature flag).

## Problem

The standard library scatters file system operations across `std::fs` as free functions:

```rust
// Path is always the odd one out
let config_file = config_dir.join("config.toml");
fs::create_dir_all(&config_dir)?;
fs::write(&config_file, config)?;
fs::set_permissions(&config_file, permissions)?;
```

## Solution

With `PathExt`, file system operations are now where they belong — on the `Path` itself:

```rust
use path_extra::std::path::PathExt as _;

// Path stays in focus and chains naturally
config_dir.create_dir_all()?
    .join("config.toml")
    .write(config)?
    .set_permissions(permissions)?;
```

## Installation

```shell
# Support std by default
cargo add path-extra

# Enable Tokio support
cargo add path-extra --features=tokio
```

## Features

### Method chaining

Operations on the same `Path` compose elegantly into a single expression.

```rust
let metadata = config_file.metadata()?;

// Atomic file update
config_file.with_extension("tmp")
    .write_new(config)?
    .set_permissions(metadata.permissions())?
    .rename(config_file)?;
```

### Graceful `NotFound` / `AlreadyExists`

Use ergonomic `_if_exists` / `_if_not_exists` variants to deal with `Ok(None)` instead.

```rust
// Before: Tedious and easy to get wrong
let config = match fs::read(&config_file) {
    Ok(config) => config,
    Err(err) if err.kind() == io::ErrorKind::NotFound => DEFAULT_CONFIG,
    Err(err) => return Err(err),
};

// After: Intent is clear, noise is gone
let config = config_file.read_if_exists()?.unwrap_or(DEFAULT_CONFIG);
```

```rust
use path_extra::std::path::{OptionPathExt as _, PathExt as _};

// Idempotent file creation
config_dir
    .create_dir_all()?
    .join("config.toml")
    .write_new_if_not_exists(DEFAULT_CONFIG)?
    .set_permissions_mode(0o644)?;
```

### Symlink flavors

```rust
target.symlink(&link)?;          // Store target as-is
target.symlink_absolute(&link)?; // Resolve target to absolute
target.symlink_relative(&link)?; // Compute relative target from link parent
```

### Atomic linking

```rust
target.symlink_atomic(&link)?;
target.symlink_absolute_atomic(&link)?;
target.symlink_relative_atomic(&link)?;

target.hard_link_atomic(&link)?;
```

### Symlink-aware checks

Both `.exists()` and `.try_exists()` don't work with broken symlinks.

```rust
// Before: Broken symlink
broken.exists()      // false
broken.try_exists()? // false

// After: See the symlink itself
broken.exists_nofollow()      // true
broken.try_exists_nofollow()? // true
```

The following `_nofollow` variants are also provided for convenience.

```rust
// Infallible — Return false on error like .exists()
path.exists_nofollow()
path.is_file_nofollow()
path.is_dir_nofollow()

// Fallible — Propagate errors just like .try_exists()
path.try_exists_nofollow()
path.try_is_file_nofollow()
path.try_is_dir_nofollow()
```

### Tokio support

The standard `Path` only has sync methods, using the wrong ones will block your async runtime.

Enable the `tokio` feature flag and `PathExt` gives you async methods directly on `Path`:

```rust
use path_extra::tokio::path::PathExt as _;

let metadata = config_file.metadata_async().await?;

config_file.with_extension("tmp")
    .write_new(config).await?
    .set_permissions(metadata.permissions()).await?
    .rename(config_file).await?;
```

Import `AsyncPathExt` to chain the entire expression and `.await` only once at the end:

```rust
use path_extra::tokio::path::{AsyncPathExt as _, PathExt as _};

config_file.with_extension("tmp")
    .write_new(config)
    .set_permissions(metadata.permissions())
    .rename(config_file)
    .await?;
```

Both `OptionPathExt` and `AsyncOptionPathExt` are available under `tokio` as well.
