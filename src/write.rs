use chrono::{DateTime, Utc};
use polars::prelude::*;

fn write_timesheet(df: DataFrame) -> DataFrame {
    let mut output_file = std::fs::File::create(clock_path()).expect("Unable to create clock file");

    JsonWriter::new(&mut output_file)
        .with_json_format(JsonFormat::JsonLines)
        .finish(&mut df)
        .expect("Unable to write DataFrame to file");

    df
}
