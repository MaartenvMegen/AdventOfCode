use rustaoc2024::get_input;
use std::collections::HashMap;

// Parse input into towel patterns and designs
fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let patterns = parts[0].split(',').map(|s| s.trim().to_string()).collect();
    let designs = parts[1].lines().map(|s| s.trim().to_string()).collect();
    (patterns, designs)
}

// Count all ways to construct a design using the towel patterns
fn count_combinations(
    design: &str,
    patterns: &[String],
    memo: &mut HashMap<String, usize>,
) -> usize {
    // Base case: if the design is empty, there's one valid way (use no towels)
    if design.is_empty() {
        return 1;
    }

    // Check memoization to avoid redundant calculations
    if let Some(&result) = memo.get(design) {
        return result;
    }

    // Try each pattern as a prefix
    let mut total_ways = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            total_ways += count_combinations(remaining, patterns, memo);
        }
    }

    // Store the result in the memoization map
    memo.insert(design.to_string(), total_ways);
    total_ways
}

// Solve the problem: Count all valid designs and their combinations
fn count_total_combinations(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    let mut memo = HashMap::new();
    // Sum the number of ways for all designs
    let output = designs
        .iter()
        .map(|design| count_combinations(design, &patterns, &mut memo))
        .sum();
    println!("{:?}", memo);
    output
}

fn main() {
    // Read input from a file
    let input = get_input("day19-input.txt");

    // Compute the result
    let result = count_total_combinations(&input);

    // Print the result
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let input = "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb\n";
        assert_eq!(count_total_combinations(input), 16);
        // Explanation of result:
        // - brwrr: 1 way
        // - bggr: 1 way
        // - gbbr: 1 way
        // - rrbgbr: 6 ways
        // - ubwu: 0 ways (impossible)
        // - bwurrg: 6 ways
        // - brgr: 1 way
        // - bbrgwb: 0 ways (impossible)
        // Total = 1 + 1 + 1 + 6 + 0 + 6 + 1 + 0 = 16
    }
}
