use rustaoc2025::get_input;
use std::fmt::Debug;

fn rotate_clockwise<T>(matrix: &[Vec<T>], padding: T) -> Vec<Vec<T>>
where T: Copy + Debug
{
    let rows = matrix.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = matrix.iter().map(|row| row.len()).max().unwrap();

    let mut rotated = vec![vec![matrix[0][0]; rows]; cols];

    for r in 0..rows {
        println!("now parsing row {}", r);
        for c in 0..cols {
            println!("now parsing column {}, max length {}", c, matrix[r].len());

            // (r, c) in original goes to (c, rows - 1 - r) in rotated
            if matrix[r].len() <= c {
                println!("padding");
                rotated[c][rows - 1 - r] = padding;
            } else {
                rotated[c][rows - 1 - r] = matrix[r][c];
            }
        }
    }
    rotated
}

fn solve2(input: &str) -> usize {
    // rotate all characters
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut parts: Vec<Vec<char>> = Vec::new();
    for line in lines {
        parts.push(line.chars().collect())
    }
    let rotated = rotate_clockwise(&parts, ' ');

    let mut numbers: Vec<u64> = Vec::new();
    let mut total: u64 = 0;
    //collect numbers for combining characters
    for line in rotated.iter().rev() {
        println!("{:?}", line);
        let line_value = line.iter().skip(1).rev().collect::<String>().trim().parse();
        if let Ok(line_value) = line_value {
            println!("{:?}", line_value);
            numbers.push(line_value);
        } else {
            numbers.clear()
        }
        match line[0] {
            '+' => total += numbers.iter().sum::<u64>(),
            '*' => total += numbers.iter().product::<u64>(),
            _ => {
                // do nothing
            }
        }
    }
    total as usize
}

fn solve(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut parts: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        parts.push(line.split_whitespace().collect())
    }

    let mut total_result = 0;

    for index in 0..parts[0].len() {
        let mut numbers: Vec<u64> = Vec::new();
        for part in parts.iter().take(parts.len() - 1) {
            numbers.push(part[index].parse().unwrap());
        }
        let result: u64 = match parts[parts.len() - 1][index] {
            "+" => numbers.iter().sum(),
            "*" => numbers.iter().product(),
            _ => panic!("Unrecognized operation"),
        };
        total_result += result;
    }
    total_result as usize
}

fn main() {
    let input = get_input("day6-input.txt");

    println!("{}", solve(&input));
    println!("{}", solve2(&input));
}
