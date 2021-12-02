use rustaoc2021::calculator::run_timed;
use rustaoc2021::reader::parse_lines_to_vec;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = s.split(" ");
        let command = result.next().unwrap();
        let value = result.next().unwrap().parse().unwrap();
        match command {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(ParseError {}),
        }
    }
}

fn main() {
    let input: Vec<Command> = parse_lines_to_vec("./resources/inputs/day2-input.txt").unwrap();
    run_timed(part_1, &input, 1);
    run_timed(part_2, &input, 2);
}

fn part_1(input: &Vec<Command>) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in input {
        match command {
            Command::Forward(amount) => horizontal += amount,
            Command::Down(amount) => depth += amount,
            Command::Up(amount) => depth -= amount,
        }
    }
    horizontal * depth
}

fn part_2(input: &Vec<Command>) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in input {
        match command {
            Command::Forward(amount) => {
                horizontal += amount;
                depth += amount * aim
            }
            Command::Down(amount) => aim += amount,
            Command::Up(amount) => aim -= amount,
        }
    }
    horizontal * depth
}
