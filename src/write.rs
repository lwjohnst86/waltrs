use std::fs::File;
use std::path::PathBuf;
use polars::prelude::*;

/// Writes the timesheet DataFrame to the timesheet JSON file.
/// 
/// # Arguments
/// 
/// - `df`: The DataFrame containing the timesheet data.
/// 
/// # Returns
/// 
/// - Returns the DataFrame that was written to the file, so has the
/// side effect of writing the DataFrame to the file.
/// 
/// # Panics
/// 
/// - If the file cannot be created or written to, or if the DataFrame
/// cannot be written to the file.
///
pub fn write_timesheet(mut df: DataFrame, path: PathBuf) -> DataFrame {
    // This creates a connection to a file to have write access to it.
    // If the file does not exist, it will be created. But, some operating
    // systems may not create the folders to the file, so there may be an error
    // for that.
    let output_file: File =
        File::create(path)
        .expect("Unable to create clock file, you may need to create the directory to the file first.");

    // Need to initialise a new writer struct to make the connection
    // to the file.
    JsonWriter::new(output_file)
        // Use a specific JSON format, in this case, add newlines between records.
        .with_json_format(JsonFormat::JsonLines)
        // Use finish to actually write the DataFrame to the file.
        // Uses `&mut` because `finish()` needs a mutable DataFrame object
        // as it needs to be able to modify it when writing. `&` because I
        // just need to give a reference to it, but not give ownership.
        .finish(&mut df)
        // TODO: update message to be more informative
        .expect("Unable to write the timesheet data to the file.");

    df
}

