@_default:
  just --list

# Build the full package, including any checks and documentation.
build: format lint test build-docs build-package

# Build the book and serve it.
serve-book: 
  # Install via `cargo install mdbook`
  mdbook serve --open

# Run linters and checkers.
lint:
  # Install via `cargo install typos`
  typos
  cargo clippy
  cargo check

# Format the code and fix issues.
format:
  cargo fmt --all -- --check
  cargo clippy --fix
  cargo fix

# Run tests and check the documentation.
test:
  cargo test
  mdbook test

# Build the documentation.
build-docs:
  mdbook build
  cargo rustdoc
  cargo doc

# Build the package.
build-package:
  cargo build
