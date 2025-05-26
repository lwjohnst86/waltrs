use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use std::time::SystemTime;
use polars::prelude::*;
use uuid::Uuid;
use dirs::home_dir;

fn clock_path() -> std::path::PathBuf {
    // This function returns the path to the clock file.
    // It uses the home directory and appends "Desktop/clock.json".
    // You can change this to any other path as needed.
    // For example, you might want to store it in a different directory.
  
    home_dir().unwrap().join("Desktop/clock.json")
}

/// A Rust-based implementation of the Watson CLI.
///
/// This is mainly a way for me to learn Rust, while also improving the
/// [Watson](https://github.com/jazzband/Watson) time tracking CLI that
/// is written in Python.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts a new clock entry
    Start {
      /// The project you are starting the clock for.
      project: String,
    }
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Start { project } => {
            let start_entry = start(project.to_string());
            println!("{}", start_entry);
        }
    }
}

fn start(project: String) -> DataFrame {
    let start_time = timestamp();
    let id = Uuid::new_v4();
    let mut df = df!(
        "start" => [start_time],
        "end" => [String::new()],
        "project" => [project],
        "id" => [id.to_string()]
    ).unwrap();

    let mut output_file = std::fs::File::create(clock_path()).expect("Unable to create clock file");

    JsonWriter::new(&mut output_file)
        .with_json_format(JsonFormat::JsonLines)
        .finish(&mut df)
        .expect("Unable to write DataFrame to file");

      df
}

fn timestamp() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    now
}

