use std::fs;

pub fn get_input(filename: &str) -> String {
    fs::read_to_string("2025/rustaoc2025/resources/".to_owned() + filename).unwrap()
}
