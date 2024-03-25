# Description

Using the fantastic data supplied by `https://github.com/Hipo/university-domains-list`, this application allows quick and easy searching of universities by name, domain, or country.

# Installation

Download applicable release if available. Use this method to **upgrade** your
current version as well. Each is an executable compressed in a ZIP file.

- Windows
- Mac (darwin)

## Source

To install from source, use the following instructions.

Requirements

- Rust toolchain (rustup, rustc, cargo)

1. Clone source code (this repository).
1. Change your current directory to the newly created folder.
1. Run `cargo build --release`.
1. Move `./target/release/school-search.exe` to where you would like it kept.

# Contribution

Uses `Task` as the build system. Use `task`, after installation (`go install github.com/go-task/task/v3/cmd/task@latest`), to see the help, you may need to
install `go` to get this to work.

- `task` - See help
