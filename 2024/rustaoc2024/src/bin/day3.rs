use std::fs;
use regex::Regex;

fn calculate_mul_sum(memory: &str, track_do_dont: bool) -> i32 {
    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don\'t\(\))").unwrap();
    let mut total_sum = 0;
    let mut mul_enabled = true;

    for cap in re.captures_iter(memory) {
        if cap.get(1).is_some() {
            if mul_enabled {
                let num1: i32 = cap[2].parse().unwrap();
                let num2: i32 = cap[3].parse().unwrap();
                total_sum += num1 * num2;
            }
        } else if cap.get(4).is_some() {
            mul_enabled = true;
        } else if track_do_dont {
            mul_enabled = false;
        }
    }
    total_sum
}

fn main() {
    let input = fs::read_to_string("2024/rustaoc2024/resources/day3-input.txt").unwrap();
    let part1 = calculate_mul_sum(&input, false);
    let part2 = calculate_mul_sum(&input, true);
    println!("Result part1: {}, Result part2: {}", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_sum_with_conditions() {
        let test_cases = [
            ("mul(2,3)mul(4,5)", 26, 26),
            ("don't()mul(2,3)do()mul(4,5)", 26, 20),
            ("mul(2,3)don't()mul(4,5)do()mul(6,7)", 68, 48),
            ("invalid_instruction mul(2,3) do() mul(4,5)", 26, 26),
            ("mul(2,3)mul(4,5)invalid_instruction", 26, 26),
            (
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
                161,
                48,
            ),
        ];

        for (input, expected_output_basic, expected_output_track) in test_cases {
            let result = calculate_mul_sum(input, false);
            assert_eq!(result, expected_output_basic);
            let result = calculate_mul_sum(input, true);
            assert_eq!(result, expected_output_track);
        }
    }
}
