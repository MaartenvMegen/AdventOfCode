use std::fmt::Debug;
use rustaoc2022::run_timed;

fn part1(input: &str) -> u64 {
    0
}

fn part2(input: &str) -> u64 {
    0
}

fn main() {
    let example = include_str!(r"../../resources/day1-example.txt");
    let input = include_str!(r"../../resources/day1-input.txt");

    rustaoc2022::run_matrix(part1, part2, example, input);
}

#[cfg(test)]
mod day2 {
    #[test]
    fn test_example() {
        let input = include_str!(r"../../resources/day1-example.txt");
    }
}
