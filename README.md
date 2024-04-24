# Reproduction for `cargo set-version`

This repo demonstrates a situation where `cargo set-version` (from `cargo edit`) fails
to propagate version updates across a workspace. The issue was verified
with `cargo edit` version `0.12.2`.

## Reproduction:

1. Install [`cargo edit`](https://github.com/killercup/cargo-edit), this was tested with version 0.12.2
2. Run `cargo set-version 0.2.0`

# Expected

The manifest `./bin/Cargo.toml` should have both its dependencies updated to version `0.2.0`.

```toml
[package]
name = "foo_bin"
version = "0.2.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
eternaltwin_core = { version = "0.2.0" }
eternaltwin_config = { version = "0.2.0", path = "../eternaltwin_config" }
```

# Actual

The manifest `./bin/Cargo.toml` should only has its dependency using `path = ..` updated to version `0.2.0`.
Notice that the line `eternaltwin_core = { version = "0.1.0" }` is different (version `0.1.0` still used).

```toml
[package]
name = "foo_bin"
version = "0.2.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
eternaltwin_core = { version = "0.1.0" }
eternaltwin_config = { version = "0.2.0", path = "../eternaltwin_config" }
```
