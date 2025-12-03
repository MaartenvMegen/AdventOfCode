use rustaoc2025::get_input;

fn joltage_detector(bank: &str, batteries: usize) -> u64 {
    let numbers: Vec<char> = bank.chars().collect();

    let mut result: u64 = 0;
    let mut start = 0;

    for digit in 0..batteries {
        let mut best_value = 0;
        let mut best_index = None;

        for (i, c) in numbers
            .iter()
            .enumerate()
            .skip(start)
            .take(numbers.len() - (batteries - digit) - start + 1)
        {
            let v = c.to_digit(10).unwrap();
            if v > best_value {
                best_value = v;
                best_index = Some(i);
            }
        }

        if let Some(idx) = best_index {
            result = result * 10 + best_value as u64;
            start = idx + 1;
        } else {
            panic!("help i am out of batteries!"); // no more digits available
        }
    }
    result
}

fn solve(input: &str) -> u64 {
    input
        .trim()
        .split('\n')
        .map(|s| joltage_detector(s, 12))
        .sum()
}

fn main() {
    let input = get_input("day3-input.txt");
    println!("{}", solve(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        assert_eq!(joltage_detector("1234", 2), 34);
    }
}
