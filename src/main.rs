use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use std::time::SystemTime;

/// A Rust-based implementation of the Watson CLI.
///
/// This is mainly a way for me to learn Rust, while also improving the
/// [Watson](https://github.com/jazzband/Watson) time tracking CLI that
/// is written in Python.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Start,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Start) => {
            println!("{}", timestamp());
        }
        None => {}
    }
}

fn timestamp() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339();

    now
}