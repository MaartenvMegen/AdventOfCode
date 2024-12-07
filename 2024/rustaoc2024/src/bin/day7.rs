use std::fs;

const OPERATIONS_B: &[fn(u64, u64) -> u64] = &[
    |a, b| a + b,
    |a, b| a * b,
    |a, b| a * 10_u64.pow(b.to_string().len() as u32) + b,
];

const OPERATIONS_A: &[fn(u64, u64) -> u64] = &[
    |a, b| a + b,
    |a, b| a * b,
];


fn can_solve(numbers: &[u64], target: u64, instructions: &[fn(u64, u64) -> u64], current_score: u64, index: usize) -> bool {
    if index == numbers.len() {
        return current_score == target;
    }

    let num = numbers[index];

    for fn_op in instructions {
        let new_score = fn_op(current_score, num);
        if new_score <= target && can_solve(numbers, target, instructions, new_score, index + 1) {
            return true;
        }
    }

    false
}

fn part_a(input: &str) -> u64 {
    get_sum_valid_input(input, OPERATIONS_A)
}

fn part_b(input: &str) -> u64 {
    get_sum_valid_input(input, OPERATIONS_B)
}

fn get_sum_valid_input(input: &str, operations : &[fn(u64, u64) -> u64]) -> u64 {
    let equations = input_to_instructions(input);
    let mut sum = 0;

    for (target, equation) in equations {
        if can_solve(&equation, target, operations, 0, 0) {
            sum += target;
        }
    }
    sum
}

fn main() {
    let example = fs::read_to_string(r"2024/rustaoc2024/resources/day7-example.txt").unwrap();
    let _input = fs::read_to_string(r"2024/rustaoc2024/resources/day7-input.txt").unwrap();

    println!("Part a: {}", part_a(&example));
    println!("Part b: {}", part_b(&example));
}

fn input_to_instructions(input: &str) -> Vec<(u64, Vec<u64>)> {
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut equations = vec![];
    for line in lines {
        let mut parts = line.split(": ");
        let target: u64 = parts.next().unwrap().parse().unwrap();
        let numbers: Vec<u64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        equations.push((target, numbers));
    }
    equations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_solve() {
        let (target, numbers) = (292, vec![11, 6 ,16, 20]);
        assert!(can_solve(&numbers, target, OPERATIONS_A, 0, 0));

    }

    #[test]
    fn test_can_solve_192() {
        let (target, numbers) = (192, vec![17, 8 ,14]);
        assert!(can_solve(&numbers, target, OPERATIONS_B, 0, 0));

    }

    #[test]
    fn test_can_sole_parta() {
        let example = fs::read_to_string(r"./resources/day7-example.txt").unwrap();
        assert_eq!(part_a(&example), 3749);
    }

    #[test]
    fn test_can_sole_partb() {
        let example = fs::read_to_string(r"./resources/day7-example.txt").unwrap();
        assert_eq!(part_b(&example), 11387);
    }
}