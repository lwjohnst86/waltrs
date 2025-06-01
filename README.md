# watrs: A time tracker built in Rust

`watrs` is a Rust-based implementation of the
[Watson](https://github.com/jazzband/Watson) CLI tool, which is written
in Python. Unfortunately, Watson hasn't been maintained for many years
and there are many open issues and pull requests that have not been
addressed. This project aims to provide a modern alternative with
similar functionality, but built in Rust.

Why Rust? Because I want to learn it and building something that already
exists is an easy way to get started. Plus, I use Watson daily for work
and over the years of using it, I've found many small issues that I
would like to fix or things to improve on. Since the original Watson is
not actively maintained, it doesn't make sense to contribute to it. Want
to learn more about how I've made this different from Watson? See the
[design document](src/design.md).

## Installing

For now, the only way to install it is a development version on GitHub
with `cargo`:

``` bash
cargo install --git https://github.com/lwjohnst86/watrs
```
