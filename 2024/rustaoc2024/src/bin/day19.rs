use std::collections::HashMap;
use rustaoc2024::get_input;

// Parse input into towel patterns and designs
fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let patterns = parts[0]
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let designs = parts[1]
        .lines()
        .map(|s| s.trim().to_string())
        .collect();
    (patterns, designs)
}

// Check if a design can be constructed using the towel patterns
fn can_construct(
    design: &str,
    patterns: &[String],
    memo: &mut HashMap<String, bool>,
) -> bool {
    // Base case: if the design is empty, it's valid
    if design.is_empty() {
        return true;
    }

    // Check memoization to avoid redundant calculations
    if let Some(&result) = memo.get(design) {
        return result;
    }

    // Try each pattern as a prefix
    for pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            if can_construct(remaining, patterns, memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }
    }

    // If no pattern works, the design is impossible
    memo.insert(design.to_string(), false);
    false
}

// Solve the problem
fn count_possible_designs(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    let mut memo = HashMap::new();

    designs
        .iter()
        .filter(|design| can_construct(design, &patterns, &mut memo))
        .count()
}

fn main() {
    // Read input from a file
    let input = get_input("day19-input.txt");

    // Compute the result
    let result = count_possible_designs(&input);

    // Print the result
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let input = "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";
        assert_eq!(count_possible_designs(input), 6);
    }
}