use rustaoc2024::get_input;
use std::collections::HashMap;

fn main() {
    let input = get_input("day11-input.txt");
    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}

fn part_a(input: &str) -> u64 {
    let stones = get_stones_from_input(input);
    let stone_counts = blink_for(stones, 25);
    stone_counts.values().sum()
}

fn part_b(input: &str) -> u64 {
    let stones = get_stones_from_input(input);
    let stone_counts = blink_for(stones, 75);
    stone_counts.values().sum()
}

fn get_stones_from_input(input: &str) -> Vec<u64> {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|number| number.to_string().parse::<u64>().unwrap())
        .collect();
    stones
}

fn blink_for(stones: Vec<u64>, times: u8) -> HashMap<u64, u64> {
    let mut stone_counts = HashMap::new();
    for stone in stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for _blink in 1..=times {
        // Count total number of stones
        stone_counts = simulate_blink(&stone_counts);
    }
    stone_counts
}

fn simulate_blink(stone_counts: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stone_counts = HashMap::new();

    for (&stone, &count) in stone_counts {
        if stone == 0 {
            // Rule 1: Replace 0 with 1
            *new_stone_counts.entry(1).or_insert(0) += count;
        } else if is_even_digits(stone) {
            // Rule 2: Split into two stones
            let digits = stone.to_string();
            let mid = digits.len() / 2;
            let left = digits[..mid].parse::<u64>().unwrap_or(0);
            let right = digits[mid..].parse::<u64>().unwrap_or(0);
            *new_stone_counts.entry(left).or_insert(0) += count;
            *new_stone_counts.entry(right).or_insert(0) += count;
        } else {
            // Rule 3: Multiply by 2024
            *new_stone_counts.entry(stone * 2024).or_insert(0) += count;
        }
    }

    new_stone_counts
}

fn is_even_digits(number: u64) -> bool {
    number.to_string().len() % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!(r"../.././resources/day11-example.txt");

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(EXAMPLE), 55312);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(EXAMPLE), 65601038650482);
    }
}
