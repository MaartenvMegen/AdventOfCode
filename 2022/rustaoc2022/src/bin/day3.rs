extern crate core;

use std::collections::HashSet;

fn part1(input: &str) -> u64 {
    input.trim().split("\n").map( |line | {
        let (first, second) = line.split_at(line.len()/2);
        let first : HashSet<char> = HashSet::from_iter(first.chars());
        let second : HashSet<char> = HashSet::from_iter(second.chars());
        let char = first.intersection(&second).next().unwrap();
        match char.is_ascii_lowercase() {
            true => *char as u64 - 96,
            false => *char as u64 - 64 + 26,
        }
    }).sum()
}

fn part2(input: &str) -> u64 {
    let backpacks : Vec<&str>= input.trim().lines().collect();
    backpacks.windows(3).step_by(3).map( |pair | {
        let first : HashSet<char> = HashSet::from_iter(pair[0].chars());
        let second : HashSet<char> = HashSet::from_iter(pair[1].chars());
        let overlap1 : HashSet<char> = first.intersection(&second).map( |char| *char ).collect();
        let third : HashSet<char> = HashSet::from_iter(pair[2].chars());
        let char  = third.intersection(&overlap1).next().unwrap();
        match char.is_ascii_lowercase() {
            true => *char as u64 - 96,
            false => *char as u64 - 64 + 26,
        }
    }).sum()
}

fn main() {
    let example = include_str!(r"../../resources/day3-example.txt");
    let input = include_str!(r"../../resources/day3-input.txt");

    rustaoc2022::run_matrix(part1, part2, example, input);
}

#[cfg(test)]
mod day3 {
    use crate::{part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!(r"../../resources/day3-example.txt");
        assert_eq!(157, part1(input));
        assert_eq!(70, part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!(r"../../resources/day3-input.txt");
        assert_eq!(7763, part1(input));
        assert_eq!(2569, part2(input));
    }
}
