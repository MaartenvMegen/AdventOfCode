use crate::reader::get_lines;

#[derive(Debug)]
enum Error {
    Unmatched(char, char),
    Missing(Vec<char>),
}

fn get_error_score_for_line(line: &str) -> Option<u64> {
    if let Err(Error::Unmatched(expected, _found)) = parse_line(line) {
        match expected {
            '>' => Some(25137),
            '}' => Some(1197),
            ']' => Some(57),
            ')' => Some(3),
            _ => panic!("unexpected char"),
        }
    } else {
        None
    }
}

fn get_missing_score_for_line(line: &str) -> Option<u64> {
    if let Err(Error::Missing(mut chars)) = parse_line(line) {
        chars.reverse();
        return Some(chars.iter().fold(0, |score, character| {
            let points: u64 = match character {
                '>' => 4,
                '}' => 3,
                ']' => 2,
                ')' => 1,
                _ => panic!("unexpected char"),
            };
            score * 5 + points
        }));
    }
    None
}

fn parse_line(line: &str) -> Result<bool, Error> {
    let mut stack = Vec::new();
    for char in line.chars().collect::<Vec<char>>() {
        match char {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => {
                if let Some(expected) = stack.pop() {
                    if char != expected {
                        return Err(Error::Unmatched(char, expected));
                    }
                }
            }
            _ => println!("error unknown input"),
        }
    }
    if stack.is_empty() {
        Ok(true)
    } else {
        Err(Error::Missing(stack))
    }
}

pub fn part_1(filename: &str) -> u64 {
    get_lines(filename)
        .flatten()
        .filter_map(|line| get_error_score_for_line(&line))
        .sum()
}

pub fn part_2(filename: &str) -> u64 {
    let mut scores: Vec<u64> = get_lines(filename)
        .flatten()
        .map(|line| get_missing_score_for_line(&line))
        .flatten()
        .collect();

    scores.sort();
    *scores.get(scores.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day10::{
        get_error_score_for_line, get_missing_score_for_line, parse_line, part_1, part_2, Error,
    };

    #[test]
    fn test_parse_line() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        assert!(matches!(parse_line(line), Err(Error::Unmatched(_, _))));

        let line = "[({(<(())[]>[[{[]{<()<>>";
        assert!(matches!(parse_line(line), Err(Error::Missing(_))));

        let line = "<>";
        assert!(matches!(parse_line(line), Ok(true)));
    }

    #[test]
    fn test_unmatched_score() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        assert_eq!(Some(1197), get_error_score_for_line(line))
    }

    #[test]
    fn test_missing_score() {
        let line = "<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(Some(294), get_missing_score_for_line(line))
    }

    #[test]
    fn test_part1() {
        assert_eq!(26397, part_1("./resources/inputs/day10-example.txt"));
        assert_eq!(415953, part_1("./resources/inputs/day10-input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(288957, part_2("./resources/inputs/day10-example.txt"));
        assert_eq!(2292863731, part_2("./resources/inputs/day10-input.txt"));
    }
}
