use rustaoc2024::{create_chunks, run_matrix};
use std::collections::{HashSet};
use std::fs;

fn part_a(input: &str) -> usize {
    let chunks = create_chunks(input);
    let rules = parse_rules(chunks[0]);
    let updates = parse_updates(chunks[1]);

    updates
        .iter()
        .filter(|update| is_valid_order(&rules, update))
        .map(|update| find_middle_number(update).unwrap())
        .sum::<u32>() as usize
}

fn part_b(input: &str) -> usize {
    let chunks = create_chunks(input);
    let rules = parse_rules(chunks[0]);
    let updates = parse_updates(chunks[1]);

    updates
        .iter()
        .filter(|update| !is_valid_order(&rules, update))
        .map(|update| find_middle_number(&sort_update(&rules, update)).unwrap())
        .sum::<u32>() as usize
}

fn parse_rules(input: &str) -> HashSet<(u32, u32)> {
    let mut rules = HashSet::new();
    for line in input.trim().split("\n") {
        let mut parts = line.split('|');
        let x = parts.next().unwrap().parse::<u32>().unwrap();
        let y = parts.next().unwrap().parse::<u32>().unwrap();
        rules.insert((x, y));
    }
    rules
}

fn parse_updates(input: &str) -> Vec<Vec<u32>> {
    let mut updates = Vec::new();
    for line in input.trim().split("\n") {
        let update: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        updates.push(update);
    }
    updates
}

fn is_valid_order(rules: &HashSet<(u32, u32)>, page_nrs: &[u32]) -> bool {
    for i in 0..page_nrs.len() - 1 {
        for j in i + 1..page_nrs.len() {
            if rules.contains(&(page_nrs[j], page_nrs[i])) {
                return false;
            }
        }
    }
    true
}

fn find_middle_number(numbers: &[u32]) -> Option<u32> {
    let len = numbers.len();
    if len == 0 {
        return None; // Handle empty list case
    }

    let middle_index = len / 2;

    if len % 2 == 1 {
        // Odd number of elements
        Some(numbers[middle_index])
    } else {
        // Even number of elements
        let middle_sum = numbers[middle_index - 1] + numbers[middle_index];
        Some(middle_sum / 2)
    }
}

fn sort_update(rules: &HashSet<(u32, u32)>, update: &[u32]) -> Vec<u32> {
    let mut sorted_update = Vec::new();
    let mut remaining_update = update.to_vec();

    while !remaining_update.is_empty() {
        for (i, page) in remaining_update.iter().enumerate() {
            if remaining_update
                .iter()
                .all(|other_page| !rules.contains(&(*other_page, *page)))
            {
                sorted_update.push(*page);
                remaining_update.remove(i);
                break;
            }
        }
    }
    sorted_update
}

fn main() {
    let example = fs::read_to_string(r"2024/rustaoc2024/resources/day5-example.txt").unwrap();
    let input = fs::read_to_string(r"2024/rustaoc2024/resources/day5-input.txt").unwrap();

    run_matrix(part_a, part_b, &example, &input);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_order() {
        let rules = [
            (47, 53), (97, 13), (97, 61), (97, 47), (75, 29), (61, 13), (75, 53),
            (29, 13), (97, 29), (53, 29), (61, 53), (97, 53), (61, 29), (47, 13),
            (75, 47), (97, 75), (47, 61), (75, 61), (47, 29), (75, 13), (53, 13),
        ]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();

        let valid_update = vec![75, 47, 61, 53, 29];
        let invalid_update = vec![75, 97, 47, 61, 53];

        assert!(is_valid_order(&rules, &valid_update));
        assert!(!is_valid_order(&rules, &invalid_update));
    }

    #[test]
    fn test_sort_update() {
        let rules = [
            (47, 53), (97, 13), (97, 61), (97, 47), (75, 29), (61, 13), (75, 53),
            (29, 13), (97, 29), (53, 29), (61, 53), (97, 53), (61, 29), (47, 13),
            (75, 47), (97, 75), (47, 61), (75, 61), (47, 29), (75, 13), (53, 13),
        ]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let update = vec![75, 97, 47, 61, 53];

        let sorted_update = sort_update(&rules, &update);
        assert_eq!(sorted_update, vec![97, 75, 47, 61, 53]);
    }

    #[test]
    fn test_find_middle_number_odd() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(find_middle_number(&numbers), Some(3));
    }

    #[test]
    fn test_find_middle_number_even() {
        let numbers = vec![1, 2, 3, 4];
        assert_eq!(find_middle_number(&numbers), Some(2));
    }

    #[test]
    fn test_find_middle_number_empty() {
        let numbers: Vec<u32> = vec![];
        assert_eq!(find_middle_number(&numbers), None);
    }
}