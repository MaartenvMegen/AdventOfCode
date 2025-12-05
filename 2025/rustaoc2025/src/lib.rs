pub mod grid;

use std::fs;

pub fn parse_range(s: &str) -> Result<(u64, u64), String> {
    let parts: Vec<&str> = s.split('-').collect();

    if parts.len() != 2 {
        return Err("Expected format: <num>-<num>".into());
    }

    let a = parts[0]
        .trim()
        .parse::<u64>()
        .map_err(|_| format!("Invalid number '{}'", parts[0]))?;

    let b = parts[1]
        .trim()
        .parse::<u64>()
        .map_err(|_| format!("Invalid number '{}'", parts[1]))?;

    Ok((a, b))
}

pub fn get_input(filename: &str) -> String {
    fs::read_to_string("2025/rustaoc2025/resources/".to_owned() + filename).unwrap()
}
