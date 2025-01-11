
# Arca

Simple, fast, rust-based [sqlite archive](https://www.sqlite.org/sqlar.html) extraction tool.

Proof of concept, not a fully-featured archival tool. Do not use in production.


## Usage

### Compress a directory

```sh
arca create <path>
```

### Extract an archive

```sh
arca extract <archive> <output directory>
```


## Installation

Make sure you have the Rust toolchain [rustup](https://rustup.rs) installed.

Then, install with `cargo`:

```sh
cargo install --git https://github.com/simulationguest/arca
```
