use std::str::FromStr;

fn part1(input: &str) -> u64 {
    let parts: Vec<&str> = rustaoc2022::create_chunks(input);
    let calories_per_elf: Vec<u64> = get_calories_per_elf(parts);
    calories_per_elf.into_iter().max().unwrap()
}

fn part2(input: &str) -> u64 {
    let parts: Vec<&str> = rustaoc2022::create_chunks(input);
    let mut calories_per_elf: Vec<u64> = get_calories_per_elf(parts);
    three_heighest_elf_sum(&mut calories_per_elf)
}

fn three_heighest_elf_sum(calories_per_elf: &mut [u64]) -> u64 {
    calories_per_elf.sort();
    calories_per_elf.iter().rev().take(3).sum()
}

fn get_calories_per_elf(parts: Vec<&str>) -> Vec<u64> {
    parts
        .iter()
        .map(|chunk| {
            chunk
                .split('\n')
                .map(|value| u64::from_str(value).unwrap())
                .sum()
        })
        .collect()
}

fn main() {
    let example = include_str!(r"../../resources/day1-example.txt");
    let input = include_str!(r"../../resources/day1-input.txt");
    rustaoc2022::run_matrix(part1, part2, example, input);
}

#[cfg(test)]
mod day1 {
    use crate::{part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!(r"../../resources/day1-example.txt");
        assert_eq!(24000, part1(input));
        assert_eq!(45000, part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!(r"../../resources/day1-input.txt");
        assert_eq!(66306, part1(input));
        assert_eq!(195292, part2(input));
    }
}
