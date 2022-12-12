use std::str::FromStr;

const EXAMPLE: &str = include_str!(r"../../resources/day10-example.txt");
const INPUT: &str = include_str!(r"../../resources/day10-input.txt");

fn part1(input: &str) -> i64 {
    let mut cycle = 0;
    let mut value = 1;
    let mut signal_strenghs: Vec<i64> = Vec::new();
    let mut output = String::new();

    for line in input.trim().split("\n") {
        let mut parts = line.split(" ");
        let (cycles, add_value) = match parts.next().unwrap() {
            "noop" => (1, 0),
            "addx" => (2, i64::from_str(parts.next().unwrap()).unwrap()),
            _ => panic!("unexpected instruction: {line}"),
        };
        for _ in 0..cycles {
            cycle += 1;
            update_screen(&mut cycle, &mut value, &mut output);
            update_single_strength(cycle, value, &mut signal_strenghs)
        }
        value += add_value;
    }
    signal_strenghs.iter().sum()
}

fn update_single_strength(cycle: i64, value: i64, signal_strenghs: &mut Vec<i64>) {
    if (cycle - 20) % 40 == 0 {
        signal_strenghs.push(cycle * value);
    }
}

fn update_screen(cycle: &i64, value: &i64, output: &mut String) {
    let position = *cycle - 1;
    if position % 40 >= value - 1 && position % 40 <= value + 1 {
        output.push('#');
    } else {
        output.push('.');
    }

    if cycle % 40 == 0 {
        println!("{output}");
        output.clear();
    }
}

fn part2(_input: &str) -> i64 {
    0
}

fn main() {
    rustaoc2022::run_matrix(part1, part2, EXAMPLE, INPUT);
}

#[cfg(test)]
mod test {
    use crate::{part1, EXAMPLE, INPUT};

    #[test]
    fn test_example() {
        assert_eq!(13140, part1(EXAMPLE));
    }

    #[test]
    fn test_input() {
        assert_eq!(12560, part1(INPUT));
    }
}
