use rustaoc2021::reader::parse_lines_to_vec;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Test {
    value: u64,
}

impl FromStr for Test {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value_from_str = s.parse::<u64>()?;
        Ok(Test {
            value: value_from_str,
        })
    }
}

fn main() -> std::io::Result<()> {
    run_part_1();
    run_part_2();

    Ok(())
}

fn run_part_2() {
    let input: Vec<u64> = parse_lines_to_vec("./resources/inputs/day1a-input.txt").unwrap();

    let mut count = 0;
    for index in 3..input.len() {
        // do something with input
        let sum_a = input[index-3] + input[index-2]+input[index-1];
        let sum_b = input[index-2] + input[index-1]+input[index];

        if sum_b>sum_a {
            count += 1;
        }
    }
    println!("part 2: {}", count);
}

fn run_part_1() {
    let input: Vec<u64> = parse_lines_to_vec("./resources/inputs/day1a-input.txt").unwrap();

    let one = &input[0..input.len()-1];
    let other = &input[1..];
    let result = one.iter().zip(other.iter()).filter( | (one,other)| other > one ).count();
    println!("part 1: {}", result);
}
