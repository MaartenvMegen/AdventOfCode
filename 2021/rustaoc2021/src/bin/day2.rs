use rustaoc2021::calculator::run_timed;
use rustaoc2021::reader::parse_lines_to_vec;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ParseError;

#[derive(Debug, PartialEq)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Vec<&str> = s.split(" ").collect();
        let command = result[0];
        let value = result[1].parse().unwrap();
        match command {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(ParseError),
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

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, Command, ParseError};
    use rustaoc2021::reader::parse_lines_to_vec;

    #[test]
    fn test_parser() {
        let command_str = "up 5";
        let command: Command = command_str.parse().unwrap();
        assert_eq!(Command::Up(5), command)
    }

    #[test]
    fn test_parser_error() {
        let command_str = "left 5";
        let command: Result<Command, ParseError> = command_str.parse();
        assert_eq!(Result::Err(ParseError), command)
    }

    #[test]
    fn test_example_part1() {
        let input: Vec<Command> =
            parse_lines_to_vec("./resources/inputs/day2-example_1.txt").unwrap();
        assert_eq!(150, part_1(&input));
    }

    #[test]
    fn test_example_part2() {
        let input: Vec<Command> =
            parse_lines_to_vec("./resources/inputs/day2-example_1.txt").unwrap();
        assert_eq!(900, part_2(&input));
    }

    #[test]
    fn test_input_part1() {
        let input: Vec<Command> = parse_lines_to_vec("./resources/inputs/day2-input.txt").unwrap();
        assert_eq!(2120749, part_1(&input));
    }

    #[test]
    fn test_input_part2() {
        let input: Vec<Command> = parse_lines_to_vec("./resources/inputs/day2-input.txt").unwrap();
        assert_eq!(2138382217, part_2(&input));
    }
}
