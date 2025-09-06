// use crate::common;
// use polars::prelude::*;
// use uuid::Uuid;

// //fn start(project: &String, tags: [&String]) -> DataFrame {
// /// Starts a new entry for a project with a timestamp for "now".
// ///
// /// # Arguments
// ///
// /// - `project`: The name of the project to start the entry for.
// ///
// /// # Returns
// ///
// /// - Returns a DataFrame with the new entry, including a unique ID, start time,
// ///  end time (empty for now), and the project name.
// ///
// /// # Errors
// ///
// /// - Returns an error if the DataFrame cannot be created.
// ///
// /// # Example
// ///
// /// ```rust
// /// use waltrs::start::start;
// /// let start_entry = start("waltrs".to_string());
// /// assert_eq!(start_entry.shape(), (1, 4));
// /// ```
// pub fn start(project: String) -> DataFrame {
//     let id: Uuid = Uuid::new_v4();
//     df!(
//         "id" => [id.to_string()],
//         "start" => [common::timestamp()],
//         "end" => [String::new()],
//         "project" => [project]
//     )
//     .expect("Can't create the DataFrame. There many be something wrong with the `project` string.")
// }

// // Unit test for the start function
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_start() {
//         let project: String = "test_project".to_string();
//         let df: DataFrame = start(project);
//         assert_eq!(df.shape(), (1, 4));
//         let expected_prj: DataFrame = df.select(["project"]).unwrap().head(Some(1));
//         let actual_prj: DataFrame = df!("project" => ["test_project"]).unwrap();
//         assert_eq!(expected_prj, actual_prj);
//     }
// }
