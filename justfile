
build: format lint test build-book build-package

serve-book: 
  # Install via `cargo install mdbook`
  mdbook serve --open

lint:
  cargo clippy
  # Install via `cargo install typos`
  typos

format:
  cargo fmt --all -- --check
  cargo clippy --fix
  cargo fix

test:
  cargo test
  mdbook test

build-book:
  mdbook build

build-package:
  cargo build
