use thiserror::Error;

/// A specialized [`Result`] type for Overmount errors.
pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    ArgumentError(String),
}

pub mod format;

#[cfg(test)]
mod tests;
