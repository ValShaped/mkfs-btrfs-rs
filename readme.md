# mkfs-btrfs-rs

A wrapper for `mkfs.btrfs` in type-safe Rust.

> NOT a FFI binding, just command wrapper, to make you feel a bit like you're writing rust.

* If you want to create a btrfs volume, check out the [`Formatter`].

* If you want to see the full list of options, check out [`format::FormatterOptions`].

# Examples
```rust no_run
use mkfs_btrfs_rs::{Formatter, Result};
use std::process::Output;
fn main() -> Result<()> {
    let formatter = Formatter::options()
        // If you provide a rootdir, mkfs.btrfs will copy the stuff in that dir into the new volume
        .rootdir("/")?
        // Labels can be arbitrary UTF-8, max 256 bytes
        .label("My Awesome New Partition")?
        // Mix data and metadata blocks
        .mixed()?
        // Don't force-format
        // .force()?
        .build();
    let Output {
        status: _status,
        stdout: out,
        stderr: err,
    } = formatter.format("/dev/sdxY")?;
    println!(
        "> STDOUT:\n{}\n> STDERR:\n{}",
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    );
    Ok(())
}
```
```rust no_run
use mkfs_btrfs_rs::{Result, Formatter};
fn main() -> Result<()> {
    let formatter = Formatter::options()
        .label("my_awesome_label")?
        .build()
        .format("/tmp/some/file")?;
    Ok(())
}
```