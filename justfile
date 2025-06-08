
build: format lint test build-docs build-package

serve-book: 
  # Install via `cargo install mdbook`
  mdbook serve --open

lint:
  cargo clippy
  cargo check
  # Install via `cargo install typos`
  typos

format:
  cargo fmt --all -- --check
  cargo clippy --fix
  cargo fix

test:
  cargo test
  mdbook test

build-docs:
  mdbook build
  cargo doc

build-package:
  cargo build
