//! # Btrfs Tests
//!
//! Tests for overmount::btrfs

use crate::{format::*, Result};

use std::process::Command;

mod checksum {
    use crate::format::ChecksumAlgorithm;
    #[test]
    fn display() {
        assert_eq!("blake2", format!("{}", ChecksumAlgorithm::Blake2));
        assert_eq!("crc32c", format!("{}", ChecksumAlgorithm::Crc32c));
        assert_eq!("sha256", format!("{}", ChecksumAlgorithm::Sha256));
        assert_eq!("xxhash", format!("{}", ChecksumAlgorithm::XxHash));
    }
}

/// Test every single option
#[test]
fn format_start_to_finish() -> Result<()> {
    let path = "/tmp/test.btrfs";
    // create empty file
    Command::new("rm").arg(path).output()?;
    Command::new("truncate")
        .args(["--size=512M", path])
        .output()?;

    let output = Formatter::options()
        .byte_count(536_870_912_u64)
        .expect("536,870,912_u64 is a valid byte_count.")
        .checksum(ChecksumAlgorithm::Crc32c)
        .expect("CRC32C is a valid ChecksumAlgorithm.")
        .data(DataProfile::Dup)
        .expect("Dup is a valid DataProfile.")
        .features(["mixed-bg"])
        .expect("mixed-bg is valid feature.")
        .force()
        .expect("`force` should not fail.")
        .label("label-label")
        .expect("label-label is 11 characters. Max 255.")
        .metadata(DataProfile::Dup)
        .expect("Dup is a valid DataProfile.")
        .mixed()
        .expect("`mixed` should not fail.")
        .no_discard()
        .expect("`no_discard` should not fail.")
        .nodesize(4096_usize)
        .expect("4096 is a valid nodesize")
        .rootdir("src")
        .expect("Path should exist (it's the path of this directory")
        .runtime_features(["quota"])
        .expect("quota is a valid runtime feature")
        .sectorsize(4096_usize)
        .expect("4096 is a valid sectorsize")
        .shrink()
        .expect("`shrink` should never fail.")
        .uuid("73e1b7e2-a3a8-49c2-b258-06f01a889bba")
        .expect("This uuid is of the correct format")
        .dump_args()
        .build()
        .format(path)
        .expect("Format::format should succeed.");

    assert!(
        output.status.success(),
        "> STDOUT:\n{}\n> STDERR:\n{}",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    );
    Command::new("rm").arg(path).output()?;
    Ok(())
}

/// Test very long strings in .label:
#[test]
fn very_long_label() {
    let label = String::from_utf8(vec![b'A'; 256]).unwrap();
    Formatter::options()
        .label(&label)
        .expect_err("Must reject labels greater than 255 bytes");
}
