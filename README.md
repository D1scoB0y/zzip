# zzip

ZIP archiver that honors `.gitignore` and `.archignore` files.

Features:

* Archives files and directories
* Preserves directory structure
* Optional `.gitignore` support
* Supports custom `.archignore` files
* Multiple input paths
* No prebuilt binaries

## Build

Requirements:

* Rust toolchain (`cargo`)

Clone the repository and build:

```bash
cargo build --release
```

The binary will be available at:

```text
target/release/zzip
```

## Usage

Archive a directory:

```bash
zzip my-project
```

Archive multiple files and directories:

```bash
zzip src Cargo.toml README.md
```

Respect `.gitignore` rules:

```bash
zzip --gitignore my-project
```

Specify output archive name:

```bash
zzip my-project -o project.zip
```

## License

MIT
