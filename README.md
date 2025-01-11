
# Arca

Simple, fast, rust-based [sqlite archive](https://www.sqlite.org/sqlar.html) extraction tool.

Proof of concept, not a fully-featured archival. Do not use in production.


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

Install the Rust toolchain with [rustup](https://rustup.rs).

Then, build the project with cargo:

```sh
cargo b --release
```

and copy the executable to a binary folder of your choice, e.g. `~/bin`:

```
cp ./target/release/arca ~/bin
```
    