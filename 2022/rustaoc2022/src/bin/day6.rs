extern crate core;

use std::collections::{HashSet};

fn part1(input: &str) -> usize {
    get_packet_start(input, 4)
}

fn part2(input: &str) -> usize {
    get_packet_start(input, 14)
}

fn get_packet_start(input: &str, packet_size: usize) -> usize {
    let input: Vec<char> = input.chars().collect();
    input
        .windows(packet_size)
        .enumerate()
        .filter(|(_, pairs)| {
            pairs.iter().collect::<HashSet<&char>>().len() == packet_size
        })
        .map(|(index, _)| index)
        .next()
        .unwrap()  + packet_size
}

fn main() {
    let example = include_str!(r"../../resources/day6-example.txt");
    let input = include_str!(r"../../resources/day6-input.txt");

    rustaoc2022::run_matrix(part1, part2, example, input);
}

#[cfg(test)]
mod day6 {
    use crate::{part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!(r"../../resources/day6-example.txt");
        assert_eq!(7, part1(input));
        assert_eq!(19, part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!(r"../../resources/day6-input.txt");
        assert_eq!(1034, part1(input));
        assert_eq!(2472, part2(input));
    }
}
