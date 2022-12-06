extern crate core;

use std::collections::HashMap;
use std::str::FromStr;

fn part1(input: &str) -> String {
    let (instructions, mut stacks) = get_stacks_and_instructions(input);
    parse_command_and_execute(perform_action_part1, instructions, &mut stacks);
    calculate_output(stacks)
}

fn part2(input: &str) -> String {
    let (instructions, mut stacks) = get_stacks_and_instructions(input);
    parse_command_and_execute(perform_action_part2, instructions, &mut stacks);
    calculate_output(stacks)
}

fn get_stacks_and_instructions(input: &str) -> (&str, HashMap<usize, Vec<String>>) {
    let mut parts = input.trim_end().split("\n\n");
    let starting_stack = parts.next().unwrap();
    let instructions = parts.next().unwrap();

    let stacks = build_stacks(starting_stack);
    (instructions, stacks)
}

fn parse_command_and_execute(command: fn(&mut HashMap<usize, Vec<String>>, u64, &usize, &usize), instructions: &str, mut stacks: &mut HashMap<usize, Vec<String>>) {
    instructions.split("\n").for_each(|line| {
        let (amount, origin, destination) = parse_instruction(line);
        command(&mut stacks, amount, &origin, &destination);
    });
}

fn calculate_output(mut stacks: HashMap<usize, Vec<String>>) -> String {
    let mut output = String::new();
    for index in 1..stacks.len() + 1 {
        output.push_str(stacks.get_mut(&index).unwrap().pop().unwrap().as_str());
    }
    output
}

fn parse_instruction(line: &str) -> (u64, usize, usize) {
    let instruction: Vec<&str> = line.split(" ").skip(1).step_by(2).collect();
    let amount = u64::from_str(instruction[0]).unwrap();
    let origin = usize::from_str_radix(instruction[1], 10).unwrap();
    let destination = usize::from_str_radix(instruction[2], 10).unwrap();
    (amount, origin, destination)
}

fn build_stacks(starting_stack: &str) -> HashMap<usize, Vec<String>> {
    let mut stack_spec: Vec<&str> = starting_stack.split("\n").collect();
    stack_spec.reverse();
    let mut stacks: HashMap<usize, Vec<String>> = HashMap::new();
    for (index, line) in stack_spec.iter().enumerate() {
        if index == 0 {
            line
                .chars()
                .skip(1)
                .step_by(4)
                .for_each(|char| {
                    stacks.insert(char::to_digit(char, 10).unwrap() as usize, Vec::new());
                });
        } else {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .for_each(|(index2, char)| {
                    if char != ' ' {
                        stacks
                            .get_mut(&(&index2 + 1))
                            .unwrap()
                            .push(String::from(char))
                    }
                });
        }
    }
    stacks
}

fn perform_action_part2(
    stacks: &mut HashMap<usize, Vec<String>>,
    amount: u64,
    origin: &usize,
    destination: &usize,
) {
    let value = stacks.get_mut(&origin).unwrap();
    let to_move_stack = value.split_off(value.len() - amount as usize);
    stacks.get_mut(&destination).unwrap().extend(to_move_stack);
}

fn perform_action_part1(
    stacks: &mut HashMap<usize, Vec<String>>,
    amount: u64,
    origin: &usize,
    destination: &usize,
) {
    for _ in 0..amount {
        let value = stacks.get_mut(&origin).unwrap();
        let value = value.pop().unwrap();
        stacks.get_mut(&destination).unwrap().push(value);
    }
}


fn main() {
    let example = include_str!(r"../../resources/day5-example.txt");
    let input = include_str!(r"../../resources/day5-input.txt");

    rustaoc2022::run_matrix(part1, part2, example, input);
}

#[cfg(test)]
mod day5 {
    use crate::{part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!(r"../../resources/day5-example.txt");
        assert_eq!("CMZ", part1(input));
        assert_eq!("MCD", part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!(r"../../resources/day5-input.txt");
        assert_eq!("LBLVVTVLP", part1(input));
        assert_eq!("TPFFBDRJD", part2(input));
    }
}
