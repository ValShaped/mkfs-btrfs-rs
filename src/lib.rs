#![doc = include_str!("../readme.md")]

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
