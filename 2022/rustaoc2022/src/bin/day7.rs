extern crate core;

use std::collections::HashMap;
use std::str::FromStr;

fn part1(input: &str) -> u64 {
    let graph = get_dir_sizes(input);

    graph
        .iter()
        .filter(|(_dir, size)| **size < 100000u64)
        .map(|(_dir, size)| size)
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    let graph = get_dir_sizes(input);

    // get total file size, than figure out how much needs to be removed, get smallest from that
    let desired = graph.get(&vec!["/".to_string()]).unwrap() - (70_000_000 - 30_000_000);

    *graph
        .iter()
        .map(|(_dir, size)| size)
        .filter(|size| **size > desired)
        .min()
        .unwrap()
}

fn get_dir_sizes(input: &str) -> HashMap<Vec<String>, u64> {
    let mut graph: HashMap<Vec<String>, u64> = HashMap::new();
    let mut directory_stack = Vec::new();

    input.trim_end().split("$").skip(1).for_each(|line| {
        match line.trim() {
            line if &line[0..2] == "cd" && &line[3..] == ".." => {
                directory_stack.pop();
            }
            line if &line[0..2] == "cd" => {
                directory_stack.push(String::from(&line[3..]));
            }
            line if &line[0..2] == "ls" => {
                parse_ls_output(&mut graph, &mut directory_stack, line);
            }
            _ => (),
        }
    });
    graph
}

fn parse_ls_output(
    graph: &mut HashMap<Vec<String>, u64>,
    directory_stack: &mut Vec<String>,
    line: &str,
) {
    line.split("\n")
        .skip(1)
        .filter(|line| &line[0..3] != "dir")
        .for_each(|ls_output| {
            let file_size = ls_output.split_once(" ").unwrap().0;

            for index in 0..directory_stack.len() + 1 {
                *graph
                    .entry(Vec::from(&directory_stack[0..index]))
                    .or_insert(0) += u64::from_str(file_size).unwrap();
            }
        });
}

fn main() {
    let example = include_str!(r"../../resources/day7-example.txt");
    let input = include_str!(r"../../resources/day7-input.txt");
    rustaoc2022::run_matrix(part1, part2, example, input);
}

#[cfg(test)]
mod day7 {
    use crate::{part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!(r"../../resources/day7-example.txt");
        assert_eq!(95437, part1(input));
        assert_eq!(24933642, part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!(r"../../resources/day7-input.txt");
        assert_eq!(1792222, part1(input));
        assert_eq!(1112963, part2(input));
    }
}
