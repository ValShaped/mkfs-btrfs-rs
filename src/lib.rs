//! Wraps `mkfs.btrfs` for use in rust.
//!
//! NOT a FFI binding, just command wrapper, to make you feel a bit like you're writing rust.
//!
//! If you want to create a btrfs volume, check out the [`Formatter`].
//! If you want to see the options we support, check out [`format::FormatterOptions`]
//!
//! # Examples
//! ```rust,no_run
//! use mkfs_btrfs_rs::{Result, Formatter};
//! fn main() -> Result<()> {
//!     let formatter = Formatter::options()
//!         .label("my_awesome_label")?
//!         .build()
//!         .format("/tmp/some/file")?;
//!     Ok(())
//! }
//! ```

use thiserror::Error;

/// A specialized [`Result`] type for mkfs-btrfs-rs errors.
pub type Result<T> = std::result::Result<T, crate::Error>;

/// [`enum@Error`] type for mkfs-btrfs-rs errors.
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    ArgumentError(String),
}

pub mod format;
pub use format::{ChecksumAlgorithm, DataProfile, Formatter};

#[cfg(test)]
mod tests;
