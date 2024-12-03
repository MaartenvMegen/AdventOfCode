use std::collections::HashMap;
use std::fs;

fn part1(input: &str) -> u64 {
    let parts: Vec<&str> = input.trim().split('\n').collect();
    let (mut col1, mut col2) = get_columns(parts);

    // Sort both columns independently
    col1.sort();
    col2.sort();

    // Calculate differences and print results
    let mut dif_total = 0;
    for (a, b) in col1.iter().zip(col2.iter()) {
        let diff = a.abs_diff(*b);
        //println!("Difference: {}", diff);
        dif_total += diff;
    }

    dif_total as u64
}

fn part2(input: &str) -> u64 {
    let parts: Vec<&str> = input.trim().split('\n').collect();

    let (mut col1, mut col2) = get_columns(parts);

    // Sort both columns independently
    col1.sort();
    col2.sort();

    // Create a HashMap to store the counts
    let mut counts: HashMap<i32, usize> = HashMap::new();

    // Iterate over col1 and count occurrences in col2
    for num in &col1 {
        let count = col2.iter().filter(|&x| x == num).count();
        counts.insert(*num, count);
    }

    col1.iter().fold(0, |acc, x| acc + (x * counts[x] as i32)) as u64
}

fn get_columns(parts: Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in parts {
        let mut iter = line.split_whitespace();
        col1.push(iter.next().unwrap().parse().unwrap());
        col2.push(iter.next().unwrap().parse().unwrap());
    }
    (col1, col2)
}

fn main() {
    let input = fs::read_to_string("2024/rustaoc2024/resources/day1-input.txt").unwrap();
    let example = fs::read_to_string("2024/rustaoc2024/resources/day1-example.txt").unwrap();
    rustaoc2024::run_matrix(part1, part2, &example, &input);
}
