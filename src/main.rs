use clap::{Parser, Subcommand};
mod start;
mod common;

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
    /// Starts a new entry for a project and optionally with tags.
    Start {
        /// The project you are starting the clock for.
        project: String,
        // tags: array of strings?
    },
    // /// Stops the current 
    // Stop,
    // /// Opens the timesheet file so you can edit it.
    // Edit,

}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Start { project } => {
            // - if no project, run stop on current project, but if no current project, return error (which will be handled by stop)

            // - if no project, use previous project/tags
            //let start_entry = start(project.to_string());
            //println!("{}", start_entry);
            start::start(project);
        }
    }
}
