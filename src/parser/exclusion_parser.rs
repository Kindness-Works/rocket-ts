use std::{fs, io::Error};

/// Parses the exclusion file and returns a vector of types to exclude.
pub fn parse_exclusion_file(exclusion_file_path: &String) -> Result<Vec<String>, Error> {
    fs::read_to_string(exclusion_file_path).map(|contents| {
        contents
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|l| !l.starts_with('#'))
            .collect()
    })
}

/// Checks if a given type should be excluded based on the exclusion list.
pub fn should_exclude_type(type_name: String, exclusion_list: &[String]) -> bool {
    exclusion_list.contains(&type_name.to_string())
}
