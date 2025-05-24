# RTail Project Guidelines

## Build Commands
- Build: `cargo build`
- Run: `cargo run -- <file_name> [-n <number>]`
- Release build: `cargo build --release`
- Check: `cargo check`

## Lint and Test Commands
- Format code: `cargo fmt`
- Lint: `cargo clippy`
- Run tests: `cargo test`
- Run a single test: `cargo test <test_name>`

## Code Style Guidelines
- **Formatting**: Use rustfmt with default settings
- **Imports**: Group standard library, then external crates, then local modules
- **Error Handling**: Use anyhow for application errors; return Result types
- **Naming**: Follow Rust conventions (snake_case for variables/functions, CamelCase for types)
- **Types**: Prefer explicit types for function signatures; use impl trait for complex returns
- **Documentation**: Document public APIs with doc comments
- **Constants**: Use SCREAMING_SNAKE_CASE for constants
- **Buffer Sizes**: Use appropriate buffer sizes for I/O operations (prefer powers of 2)