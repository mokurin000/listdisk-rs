# listdisk-rs

`listdisk-rs` is a Rust project for disk and drive information utilities, featuring multiple example programs and modular features. It is written entirely in Rust.

## Features

- Modular feature flags: volume, logical-drives, freespace, drive-info, serde, and more.
- Supports Windows storage APIs (via `windows-sys`), WMI querying, and Unicode/UTF-16 strings.
- Example binaries for volume listing, drive info, freespace reporting, and full usage.
- Logging support (`log`, `pretty_env_logger`).
- Error handling with `thiserror` and convenience crates for development.

## Getting Started

### Requirements

- Rust (Edition 2024 or later)

### Building

Clone this repo:
```sh
git clone https://github.com/mokurin000/listdisk-rs.git
cd listdisk-rs
```

Build the project:
```sh
cargo build
```

Run a specific example (replace `EXAMPLE` as needed):
```sh
cargo run --example list_volume
```

### Feature flags

Enable features with Cargo's `--features` flag, e.g.:
```sh
cargo run --example list_volume --features volume
```

See the `Cargo.toml` for the complete list of available features.

## Project Structure

- [`src/`](https://github.com/mokurin000/listdisk-rs/tree/main/src): Main source code
- [`examples/`](https://github.com/mokurin000/listdisk-rs/tree/main/examples): Example binaries
- [`Cargo.toml`](https://github.com/mokurin000/listdisk-rs/blob/main/Cargo.toml): Feature and dependency configuration

## Dependencies

Key dependencies:
- [log](https://crates.io/crates/log)
- [utf16string](https://crates.io/crates/utf16string) (optional)
- [windows-sys](https://crates.io/crates/windows-sys) (optional)
- [bitvec](https://crates.io/crates/bitvec) (optional)
- [wmi](https://crates.io/crates/wmi) (optional)
- [serde](https://crates.io/crates/serde) (optional)
- [thiserror](https://crates.io/crates/thiserror)

## License

See `Cargo.toml` for more information about this project and its licensing.

---
*100% Rust. See also the [Cargo.toml](https://github.com/mokurin000/listdisk-rs/blob/main/Cargo.toml) for all configuration details.*