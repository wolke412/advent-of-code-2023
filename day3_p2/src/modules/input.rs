use std::fs;

pub fn file_in(file_name: &str) -> String {

    return fs::read_to_string(file_name)
        .expect("Error reading file.")
        .trim()
        .to_string();
}
