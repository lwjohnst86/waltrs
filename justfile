@_default:
  just --list

# Build the full package, including any checks and documentation.
run-all: format check test build

# Build the book and serve it.
serve-book: 
  # Install via `cargo install mdbook`
  mdbook serve --open

# Run all checks.
check: check-spelling check-fmt check-cargo check-clippy

# Run linters and checkers.
check-spelling:
  # Install via `cargo install typos`
  typos

# Checks with clippy.
check-clippy:
  cargo clippy -- \
    -W clippy::pedantic \
    -W clippy::nursery \
    -W clippy::unwrap_used

# Checks with cargo.
check-cargo:
  cargo check
  
# Checks with rustfmt.
check-fmt:
  cargo fmt --check

# Format the code and fix issues.
format:
  cargo fix
  cargo clippy --fix
  cargo fmt

# Run tests and check the documentation.
test: test-src test-docs

# Run the tests in the `src/` or `tests/` directories.
test-src:
  cargo test
  
# Run tests on or in the documentation.
test-docs:
  mdbook test

# Run all build recipes.
build: build-docs build-package

# Build the documentation.
build-docs:
  mdbook build
  cargo doc

# Build the package.
build-package:
  cargo build
