# rtail

A Rust implementation of the tail command that displays the last part of a file.

## Features

- Display the last N lines of a file
- Efficient buffer-based reading for large files
- Simple command-line interface

## Installation

```bash
cargo install --path .
```

## Usage

```bash
rtail [-n <number>] <file_name>
```

Where:

- `<file_name>`: Path to the file you want to read
- `-n, --number`: Number of lines to display from the end (default: 10)

## Example

```bash
rtail -n 5 example.txt
```

## Building from source

```bash

# Build
cargo build --release

# Run
cargo run -- -n 5 example.txt
```

## License

MIT
