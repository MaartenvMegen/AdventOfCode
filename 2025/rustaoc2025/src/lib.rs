mod bfs;
pub mod grid;

use std::fmt::Debug;
use std::fs;
use std::time::Instant;

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

pub fn run_timed<T, X>(f: fn(T) -> X, argument: T, part: u64)
where
    X: Debug,
{
    let now = Instant::now();

    let answer = f(argument);

    println!(
        "part {}: {:?}, result found in {} ms",
        part,
        answer,
        now.elapsed().as_millis()
    );
}

pub fn get_input(filename: &str) -> String {
    fs::read_to_string("2025/rustaoc2025/resources/".to_owned() + filename).unwrap()
}
