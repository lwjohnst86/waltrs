use clap::{Args, Parser, Subcommand};
// mod common;
// mod start;

// Structs are data structures that can hold multiple custom values
// or data types.
//
// For the `clap` crate, it uses structs to contain the command line
// arguments and options that the user can provide when running the CLI.
// In this case, it would hold all the parameters and options that the user
// can pass to the CLI. It is a structj

/// A Rust-based implementation of the Watson CLI.
///
/// This is mainly a way for me to learn Rust, while also improving the
/// [Watson](https://github.com/jazzband/Watson) time tracking CLI that
/// is written in Python.
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    // This is the first argument that the user can provide when running the CLI.
    // It uses the `#[command(subcommand)]` attribute to indicate that this field
    // will contain subcommands. Or rather, it will contain multiple types of commands
    // in the first (required) argument position. The `Commands` enum defines
    // the different first position commands that can be used in the CLI, shown
    // below in `enum Commands`.
    #[command(subcommand)]
    command: Commands,
}

// `enum` is a data structure that can take on different variants, but only
// one at a time. In this case, `Commands` is an enum that represents the
// different commands that can be executed in the CLI application. It is an
// `enum` because you can only ever use one command at a time in the CLI.
// For example, if you use `Start`, you can't also use `Stop`.
//
// Use the `derive(Subcommand)` attribute to automatically connect the
// trait for the `Commands` enum.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Starts the timer for a project and optionally with tags.
    ///
    /// Creates a new entry in the timesheet file with an empty `stop` field. If
    /// a timer is already running for a different project, it will stop the
    /// timer on the other project and start a new one for the given project. If
    /// no project is given, it defaults to starting the last project (along
    /// with the same tags used).
    ///
    /// It will not add a new entry if the timesheet file can't be
    /// parsed/read, if there are duplicate `id` values, or if there are
    /// `start` entries later than the `stop` entries (`stop` is earlier than
    /// `start`).
    Start {
        /// The project you are starting the clock for. If none is given,
        /// it will use the last project you worked on.
        #[arg(required = true)]
        project: String,
        /// Optional tags to add to the timesheet entry. Defaults to an
        /// empty array if no tag is given.
        #[arg(short, long)]
        tags: Option<Vec<String>>,
    },
    /// Stops the current timer.
    ///
    /// Adds a timestamp to the `stop` value in the timesheet file with an entry
    /// that has an empty `stop` value. It will only stop and add the timestamp
    /// if there is a timer running (the most recent `start` timestamp without a
    /// `stop` timestamp). Can't stop if the current `start` entry is in the
    /// future (e.g. if the person incorrectly edited the file).
    ///
    /// It has the same constraints as `start` for when it won't work.
    Stop,
    /// Opens the timesheet file so you can edit it.
    ///
    /// Opens the timesheet file in the your editor (e.g. `vim`) for you to
    /// edit directly yourself. Use this to fix a mistake in an entry, remove an
    /// entry, fix a merge conflict, or cancel the currently running entry.
    /// Since this will "simply" open the file in the default editor, going to
    /// the necessary entry can be done by searching for the timestamp, project
    /// or tag, or going to the end of the file for the most recent entry.
    ///
    /// If you edit the file, it will only save if it passes the same
    /// constraints as the `start` command.
    Edit,
    /// Get some basic statistics about your projects and time usage.
    ///
    /// This command contains several subcommands that provide basic
    /// statistics for various aspects of the timesheet data.
    Stats(StatsArgs),
    Today,
}

// This contains the subcommands for the `stats` command, e.g. `waltrs stats project`
#[derive(Args, Debug)]
struct StatsArgs {
    /// Shows statistics about your projects and time usage.
    #[arg(name = "type", required = true, value_parser = ["projects", "tags"])]
    stat_type: String,
    /// The unit of time to show the stats for.
    #[arg(short, long, default_value = "week", value_parser = ["day", "week", "month", "year", "all"])]
    unit: String,
    /// The number of units to show the stats for, e.g. 10 weeks.
    #[arg(short, long, default_value_t = 1)]
    number_of_units: u8,
    /// Whether to include tags in the stats output (only for `projects` type).
    #[arg(short, long, default_value_t = false)]
    include_tags: bool,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args.command);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    // match &cli.command {
    //     Commands::Start { project } => {
    //         // - if no project, run stop on current project, but if no current project, return error (which will be handled by stop)

    //         // - if no project, use previous project/tags
    //         //let start_entry = start(project.to_string());
    //         //println!("{}", start_entry);
    //         start::start(project);
    //     }
    // }
}
