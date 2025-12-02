use std::fs;

fn parse_range(s: &str) -> Result<(u64, u64), String> {
    let parts: Vec<&str> = s.split('-').collect();

    if parts.len() != 2 {
        return Err("Expected format: <num>-<num>".into());
    }

    let a = parts[0]
        .trim()
        .parse::<u64>()
        .map_err(|_| format!("Invalid number '{}'", parts[0]))?;

    let b = parts[1]
        .trim()
        .parse::<u64>()
        .map_err(|_| format!("Invalid number '{}'", parts[1]))?;

    Ok((a, b))
}

fn is_valid_pt2(input: u64) -> bool {
    // a string based approach seems most efficient
    let number = input.to_string();
    let len = number.len();

    // A repeating pattern must evenly divide the length
    for pat_len in 1..=len / 2 {
        if len % pat_len != 0 {
            continue;
        }

        let pat = &number[..pat_len];
        let repeated = pat.repeat(len / pat_len);

        if repeated == number {
            return false; // invalid
        }
    }

    true // valid ID
}

fn is_valid(input: u64) -> bool {
    let digits = input.to_string().len();
    // odd digits cant return a repeat
    if digits % 2 != 0 {
        return true;
    }

    // Half the digits
    let half = digits / 2;

    // 10^half
    let pow = 10u64.pow(half as u32);

    // Shift down by removing lower half
    let downshifted = input / pow;
    let lower_part = input - downshifted * pow;
    lower_part != downshifted
}
fn solve(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        // parse "95-115" -> (95,115)
        .map(|s| parse_range(s).expect("invalid range"))
        // expand each (lower, higher) into the inclusive range lower..=higher
        .flat_map(|(lower, higher)| lower..=higher)
        // keep only invalid numbers
        .filter(|&number| {
            let valid = is_valid_pt2(number);
            if !valid {
                println!("{number} is invalid");
            }
            !valid
        })
        // sum them up
        .sum()
}
fn main() {
    let input = fs::read_to_string("2025/rustaoc2025/resources/day2-input.txt").unwrap();
    let example = fs::read_to_string("2025/rustaoc2025/resources/day2-example.txt").unwrap();
    println!("{}", solve(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        assert!(is_valid(1234));
        assert!(!is_valid(4646))
    }

    #[test]
    fn test_solve_pt2() {
        assert!(is_valid_pt2(1234));
        assert!(!is_valid_pt2(4646));
        assert!(!is_valid_pt2(464646))
    }
}
