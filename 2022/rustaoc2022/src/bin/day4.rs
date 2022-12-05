extern crate core;

use std::collections::HashSet;
use std::str::FromStr;

fn part1(input: &str) -> u64 {
    input
        .trim()
        .split("\n")
        .map(|line|
            get_sectors_from_line(line)
        )
        .filter(|(sector1, sector2)|
            sector1.is_subset(&sector2) || sector2.is_subset(&sector1)
        )
        .count() as u64
}

fn get_sectors_from_line(line: &str) -> (HashSet<u64>, HashSet<u64>) {
    let mut elves = line
        .split(",")
        .map(|section| {
            let range: Vec<u64> = section
                .split("-")
                .map(|sector| u64::from_str(sector).unwrap())
                .collect();
            HashSet::from_iter(range[0]..range[1] + 1)
        });
    (elves.next().unwrap(), elves.next().unwrap())
}

fn part2(input: &str) -> u64 {
    input
        .trim()
        .split("\n")
        .map(|line| {
            get_sectors_from_line(line)
        })
        .filter(|(sector1, sector2)| !sector2.is_disjoint(&sector1))
        .count() as u64
}

fn main() {
    let example = include_str!(r"../../resources/day4-example.txt");
    let input = include_str!(r"../../resources/day4-input.txt");

    rustaoc2022::run_matrix(part1, part2, example, input);
}

#[cfg(test)]
mod day3 {
    use crate::{part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!(r"../../resources/day4-example.txt");
        assert_eq!(2, part1(input));
        assert_eq!(4, part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!(r"../../resources/day4-input.txt");
        assert_eq!(459, part1(input));
        assert_eq!(779, part2(input));
    }
}
