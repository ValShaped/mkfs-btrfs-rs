//! # Creates a Btrfs filesystem.
//! Requires [`btrfs-progs`].
//!
//! A wrapper around [`mkfs.btrfs`].
//!
//! Use `FormatterOptions` to specify the options you want to format with, then
//! format with `.build().format();`
//!
//! See usage for [`mkfs.btrfs`] for more details.
//!
//! # Examples
//! ```
//! # use mkfs_btrfs_rs::Error;
//! use mkfs_btrfs_rs::format::{
//!     ChecksumAlgorithm::Crc32c,
//!     DataProfile,
//!     Formatter,
//! };
//! // Configure a formatter
//! let formatter = Formatter::options()
//!     // These are all optional
//!     .byte_count(536_870_912_u64)?
//!     .checksum(Crc32c)?
//!     .data(DataProfile::Dup)?
//!     .features(["mixed-bg"])?
//!     .force()?              // true if called
//!     .label("label")?
//!     .metadata(DataProfile::Dup)?
//!     .mixed()?              // true if called
//!     .no_discard()?         // true if called
//!     .nodesize(4096_usize)?
//!     .rootdir("./testdir")?
//!     .runtime_features(["quota"])?
//!     .sectorsize(4096_usize)?
//!     .shrink()?             // true if called
//!     .uuid("73e1b7e2-a3a8-49c2-b258-06f01a889bba")?
//!     // build the Formatter
//!     .build();
//! // Format a device
//! formatter.format("./test.btrfs")?;
//! # Ok::<(), Error>(())
//! ```
//! [`btrfs-progs`]: https://btrfs.readthedocs.io/en/latest/Introduction.html
//! [`mkfs.btrfs`]: https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html

use crate::{Error::*, Result};
use std::{
    ffi::OsString,
    io::Result as IoResult,
    path::Path,
    process::{Command, Output},
};

pub const RUNTIME_FEATURES: [&str; 2] = ["quota", "free-space-tree"];

/// Represents the set of valid (meta)data profiles.
/// ```sh
/// mkfs.btrfs --data ( raid0 | raid1 | ... )
/// ```
#[derive(Copy, Clone, Debug)]
pub enum DataProfile {
    Raid0,
    Raid1,
    Raid1c3,
    Raid1c4,
    Raid5,
    Raid6,
    Raid10,
    Single,
    Dup,
}

impl std::fmt::Display for DataProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DataProfile::*;
        let data_profile: &str = match *self {
            Raid0 => "raid0",
            Raid1 => "raid1",
            Raid1c3 => "raid1c3",
            Raid1c4 => "raid1c4",
            Raid5 => "raid5",
            Raid6 => "raid6",
            Raid10 => "raid10",
            Single => "single",
            Dup => "dup",
        };
        write!(f, "{data_profile}")
    }
}

/// Represents the set of valid block checksum algorithms.
/// ```sh
/// mkfs.btrfs --checksum [ crc32c | xxhash | sha256 | blake2 ]
/// ```
#[derive(Clone, Copy, Debug)]
pub enum ChecksumAlgorithm {
    Crc32c,
    XxHash,
    Sha256,
    Blake2,
}
impl std::fmt::Display for ChecksumAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ChecksumAlgorithm::*;
        let algorithm: &str = match *self {
            Crc32c => "crc32c",
            XxHash => "xxhash",
            Sha256 => "sha256",
            Blake2 => "blake2",
        };
        write!(f, "{algorithm}")
    }
}

/// It's like an Option, but THICC
#[derive(Clone, Debug, Default)]
enum FormatOpt {
    #[default]
    None,
    List(Vec<String>),
}

impl std::fmt::Display for FormatOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatOpt::None => write!(f, "None"),
            FormatOpt::List(arg) => write!(f, "{}", arg.join(",")),
        }
    }
}

/// Represents options for [`mkfs.btrfs`](https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html#options).
#[derive(Clone, Debug, Default)]
pub struct FormatterOptions {
    byte_count: Option<OsString>,       // Uint
    checksum: Option<OsString>,         // Csum
    data: Option<OsString>,             // Data
    features: Option<OsString>,         // List
    force: Option<OsString>,            // Bool
    label: Option<OsString>,            // Text
    metadata: Option<OsString>,         // Data
    mixed: Option<OsString>,            // Bool
    no_discard: Option<OsString>,       // Bool
    nodesize: Option<OsString>,         // Uint
    rootdir: Option<OsString>,          // Path
    runtime_features: Option<OsString>, // List
    sectorsize: Option<OsString>,       // Uint
    shrink: Option<OsString>,           // Bool
    uuid: Option<OsString>,             // Uuid
}

impl FormatterOptions {
    /// Specify the size of each device, as seen by the filesystem.
    ///
    /// # Example
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .byte_count(536_870_912_u64)?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn byte_count(mut self, byte_count: u64) -> Result<Self> {
        self.byte_count = Some(OsString::from(format!("--byte-count={byte_count}")));
        Ok(self)
    }
    /// Specify the checksum algorithm (as ChecksumAlgorithm.)
    ///
    /// # Example
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::{
    /// *,
    /// ChecksumAlgorithm::Crc32c
    /// };
    /// Formatter::options()
    ///     .checksum(Crc32c)?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn checksum(mut self, checksum: ChecksumAlgorithm) -> Result<Self> {
        self.checksum = Some(OsString::from(format!("--checksum={checksum}")));
        Ok(self)
    }
    /// Specify the profile for data block groups (as DataProfile.)
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::{DataProfile, Formatter};
    /// Formatter::options()
    ///     .data(DataProfile::Dup)?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn data(mut self, data: DataProfile) -> Result<Self> {
        self.data = Some(OsString::from(format!("--data={data}")));
        Ok(self)
    }
    /// Set mkfs-time features. Unset features by prefixing them with '^'.
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .features(["mixed-bg"])?;
    /// # Ok::<(), Error>(())
    /// ```
    // TODO: Verify features.
    // ? mkfs.btrfs verifies them again later, so is that even necessary?
    pub fn features<'a>(mut self, features: impl IntoIterator<Item = &'a str>) -> Result<Self> {
        self.features = Some(OsString::from(format!(
            "--features={}",
            FormatOpt::List(
                features
                    .into_iter()
                    .map(|x| -> String { x.to_owned() })
                    .collect()
            )
        )));
        Ok(self)
    }
    /// Force-format the device, even if an existing filesystem is present.
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .force()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn force(mut self) -> Result<Self> {
        self.force = Some(OsString::from("--force"));
        Ok(self)
    }
    /// Set the partition label.
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .label("ExampleLabel")?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn label(mut self, label: &str) -> Result<Self> {
        if label.len() > 255 {
            return Err(ArgumentError(format!(
                "label cannot be longer than 255 bytes: {}, {label}",
                label.len()
            )));
        }
        self.label = Some(OsString::from(format!("--label={label}")));
        Ok(self)
    }
    /// Specify the profile for metadata block groups (as DataProfile.)
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::{DataProfile, Formatter};
    /// Formatter::options()
    ///     .metadata(DataProfile::Dup)?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn metadata(mut self, metadata: DataProfile) -> Result<Self> {
        self.metadata = Some(OsString::from(format!("--metadata={metadata}")));
        Ok(self)
    }
    /// Enable mixing of data and metadata blocks
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .mixed()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn mixed(mut self) -> Result<Self> {
        self.mixed = Some(OsString::from("--mixed"));
        Ok(self)
    }
    /// Disable implicit TRIM of storage device.
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .no_discard()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn no_discard(mut self) -> Result<Self> {
        self.no_discard = Some(OsString::from("--nodiscard"));
        Ok(self)
    }
    /// Specify the size of a b-tree node
    ///
    /// `nodesize must be a power of 2 less than 2^14
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .label("ExampleLabel")?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn nodesize(mut self, nodesize: usize) -> Result<Self> {
        if nodesize.is_power_of_two() && nodesize <= 16384 {
            self.nodesize = Some(OsString::from(format!("--nodesize={nodesize}")));
            Ok(self)
        } else {
            Err(ArgumentError(format!(
                "node_size ( = {nodesize} )\nMust be a power of 2, and <= 16384"
            )))
        }
    }
    /// Specify a directory containing data to copy into the btrfs filesystem.
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .rootdir("./testdir")?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn rootdir<P: AsRef<Path>>(mut self, rootdir: P) -> Result<Self> {
        // make sure the rootdir is a valid Path
        rootdir.as_ref().try_exists()?;
        let rootdir = format!("--rootdir={}", rootdir.as_ref().display());
        self.rootdir = Some(OsString::from(rootdir));
        Ok(self)
    }
    /// Set runtime features.
    /// Unset features by prefixing them with '^'.
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .runtime_features(["quota"])?;
    /// # Ok::<(), Error>(())
    /// ```
    // TODO: Verify runtime features? is that even necessary?
    pub fn runtime_features<'a>(
        mut self,
        features: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self> {
        self.runtime_features = Some(OsString::from(format!(
            "--runtime-features={}",
            FormatOpt::List(
                features
                    .into_iter()
                    .map(|x| -> String { x.to_owned() })
                    .collect(),
            )
        )));
        Ok(self)
    }
    /// Set sector size.
    ///
    /// *If set to a value unsupported by the current kernel,*
    /// *the resulting volume will not be mountable.*
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .sectorsize(4096_usize)?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn sectorsize(mut self, sectorsize: usize) -> Result<Self> {
        self.sectorsize = Some(OsString::from(format!("--sectorsize={sectorsize}")));
        Ok(self)
    }
    /// If the specified device is a file, and the `rootdir` option is specified,
    /// shrink the file to the minimum required size
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .shrink()?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn shrink(mut self) -> Result<Self> {
        self.shrink = Some(OsString::from("--shrink"));
        Ok(self)
    }
    /// Set the partition UUID
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .uuid("73e1b7e2-a3a8-49c2-b258-06f01a889bba")?;
    /// # Ok::<(), Error>(())
    /// ```
    // TODO: Verify UUIDs (with external crate?)
    pub fn uuid(mut self, uuid: &str) -> Result<Self> {
        self.uuid = Some(OsString::from(format!("--uuid={uuid}")));
        Ok(self)
    }

    /// Convert self into args (AKA `Vec<OsString>`)
    fn to_args(&self) -> Vec<OsString> {
        let mut args = vec![];
        for option in [
            &self.byte_count,
            &self.checksum,
            &self.data,
            &self.features,
            &self.force,
            &self.label,
            &self.metadata,
            &self.mixed,
            &self.no_discard,
            &self.nodesize,
            &self.rootdir,
            &self.runtime_features,
            &self.sectorsize,
            &self.shrink,
            &self.uuid,
        ] {
            if let Some(arg) = option.as_ref() {
                args.push(arg.clone());
            }
        }
        args
    }

    /// Dump FormatterOptions as they'll be passed to mkfs.btrfs
    ///
    /// # Examples
    /// ```
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .dump_args();
    /// ```
    pub fn dump_args(self) -> Self {
        println!("{:#?}", self.to_args());
        self
    }

    /// Bake FormatterOptions into a Formatter
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// Formatter::options()
    ///     .label("my-Btrfs-volume")?
    ///     .rootdir("./testdir")?
    ///     .shrink()?
    ///     .build();
    /// # Ok::<(), Error>(())
    /// ```
    pub fn build(&self) -> Formatter {
        let args = self.to_args();
        Formatter { args }
    }
}

/// Formats anything that can be Btrfs-formatted.
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Formatter {
    args: Vec<OsString>,
}

impl Formatter {
    /// Specify FormatterOptions first, then build a formatter
    ///
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::Formatter;
    /// let options = Formatter::options()
    /// /* set options here...*/;
    /// options.build().format("./test.btrfs")?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn options() -> FormatterOptions {
        FormatterOptions::default()
    }
    /// Format a device with mkfs.btrfs
    ///
    /// # Examples
    /// ```
    /// # use mkfs_btrfs_rs::Error;
    /// use mkfs_btrfs_rs::format::*;
    /// Formatter::options()
    ///     .label("my-Btrfs-volume")?
    ///     .rootdir("./testdir")?
    ///     .shrink()?
    ///     .build()
    ///     .format("./test.btrfs")?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn format<P: AsRef<Path>>(mut self, device: P) -> IoResult<Output> {
        device.as_ref().try_exists()?;
        self.args.push(OsString::from(device.as_ref()));
        // FIXME: Parse the output of mkfs.btrfs and send it back properly.
        Command::new("mkfs.btrfs").args(self.args).output()
    }
}
