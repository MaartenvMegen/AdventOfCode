use rustaoc2025::{get_input, parse_range};

fn is_valid_pt2(input: u64) -> bool {
    // a string based approach seems most efficient
    let number = input.to_string();
    let len = number.len();

    // A repeating pattern must evenly divide the length
    for pat_len in 1..=len / 2 {
        if !len.is_multiple_of(pat_len) {
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
    if !digits.is_multiple_of(2) {
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
fn solve(input: &str, is_valid: fn(u64) -> bool) -> u64 {
    input
        .trim()
        .split(',')
        // parse "95-115" -> (95,115)
        .map(|s| parse_range(s).expect("invalid range"))
        // expand each (lower, higher) into the inclusive range lower..=higher
        .flat_map(|(lower, higher)| lower..=higher)
        // keep only invalid numbers
        .filter(|&number| {
            let valid = is_valid(number);
            if !valid {
                println!("{number} is invalid");
            }
            !valid
        })
        // sum them up
        .sum()
}
fn main() {
    let input = get_input("day2-example.txt");
    println!("{}", solve(input.as_str(), is_valid));
    println!("{}", solve(input.as_str(), is_valid_pt2));
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
