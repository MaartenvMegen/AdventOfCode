extern crate core;

fn map_symbol_to_value(symbol: &str) -> u64 {
    match symbol {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!("unexpected symbol")
    }
}

fn calculate_score(other : u64, own : u64) -> u64 {
    let win_score = (other as i64 - own as i64 ) % 3;
    let win_score = match win_score {
        1 | -2 => 0,
        2 | -1 => 6,
        0 => 3,
        _ => panic!("unexpected score: {}", win_score)
    };
    win_score + own + 1
}

fn map_goal_to_num(goal : &str , other_hand : u64 ) -> u64 {
    // stupid +3 offset because negative values have negative remainder
    match goal {
        "X" => (other_hand + 2 ) % 3,
        "Y" => other_hand,
        "Z" => (other_hand + 4 ) % 3,
        _ => panic!("unexpected goal: {}", goal)
    }
}

fn part1(input: &str) -> u64 {
    input.trim().split("\n").map( |line | {
        let pairs : Vec<u64> = line.split(" ").map(map_symbol_to_value).take(2).collect();
        calculate_score(pairs[0], pairs[1])
    }).sum()
}

fn part2(input: &str) -> u64 {
    input.trim().split("\n").map( |line | {
        let pairs : Vec<&str> = line.split(" ").take(2).collect();
        let other = map_symbol_to_value(pairs[0]);
        calculate_score(other, map_goal_to_num(pairs[1], other))
    }).sum()
}

fn main() {
    let example = include_str!(r"../../resources/day2-example.txt");
    let input = include_str!(r"../../resources/day2-input.txt");

    rustaoc2022::run_matrix(part1, part2, example, input);
}

#[cfg(test)]
mod day2 {
    use crate::{part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!(r"../../resources/day2-example.txt");
        assert_eq!(15, part1(input));
        assert_eq!(12, part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!(r"../../resources/day2-input.txt");
        assert_eq!(12276, part1(input));
        assert_eq!(9975, part2(input));
    }
}
