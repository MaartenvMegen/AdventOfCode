use std::collections::{HashMap, HashSet};
use std::iter::once;
use crate::day13::min_max;

pub fn run_day_14(input: &str, rounds : u64) -> usize {
    let (template, insert_lookup) = parse_input(input);
    let char_counts = perform_insertion_rounds(rounds, &template, insert_lookup);
    get_answer(char_counts)
}

fn get_answer(char_counts: HashMap<&str, u64>) -> usize {
    let min = char_counts.iter().map(|(k, v)| v).min().unwrap();
    let max = char_counts.iter().map(|(k, v)| v).max().unwrap();
    (max - min) as usize
}

fn parse_input(input: &str) -> (Vec<&str>, HashMap<(&str, &str), &str>) {
    let (template, insertions) = input.split_once("\n\n").unwrap();
    let insertions = insertions.trim().split("\n");
    let mut insert_lookup: HashMap<(&str, &str), &str> = HashMap::new();

    for insertion in insertions {
        let (combination, inserted) = insertion.trim().split_once(" -> ").unwrap();
        let pair = combination.trim().split_at(1);

        insert_lookup.insert(pair, inserted);
    }
    let template : Vec<&str> = template.split_terminator("").skip(1).collect();

    (template, insert_lookup)
}

fn perform_insertion_rounds<'a>(rounds: u64, template: &'a Vec<&str>, insert_lookup: HashMap<(&str, &str), &'a str>) -> HashMap<&'a str, u64> {
    // init using starting string
    let mut pair_counts: HashMap<(&str, &str), u64> = HashMap::new();
    let mut char_counts: HashMap<&str, u64> = HashMap::new();
    for char in template {
        *char_counts.entry(char).or_insert(0) += 1;
    }
    for pair in template.windows(2) {
        *pair_counts.entry((pair[0], pair[1])).or_insert(0) += 1;
    }

    // do iterations
    for _ in 0..rounds {
        let mut new_counts: HashMap<(&str, &str), u64> = HashMap::new();

        for (pair, count) in pair_counts {
            if count > 0 {
                let insert = insert_lookup.get(&pair).unwrap();
                let (pair_a, pair_b) = pair;
                *char_counts.entry(insert).or_insert(0) += count;
                *new_counts.entry((pair_a, *insert)).or_insert(0) += count;
                *new_counts.entry((*insert, pair_b)).or_insert(0) += count;
            }
        }
        pair_counts = new_counts
    }
    char_counts
}

#[cfg(test)]
mod tests {
    use crate::day14::{run_day_14};


    #[test]
    fn test_split() {
        let combination = "HH";
        println!("{:?}",combination.split_at(1));
        let template = "AVG";
        let template : Vec<&str> = template.split_terminator("").skip(1).collect();
        println!("{:?}", template)
    }

    #[test]
    fn test_10_rounds() {
        let input = include_str!(r"../resources/inputs/day14-example.txt");
        assert_eq!(1588, run_day_14(input, 10));
        let input2 = include_str!(r"../resources/inputs/day14-input.txt");
        assert_eq!(3906, run_day_14(input2, 10));
    }

    #[test]
    fn test_40_rounds() {
        let input1 = include_str!(r"../resources/inputs/day14-example.txt");
        assert_eq!(2188189693529, run_day_14(input1, 40));
        let input2 = include_str!(r"../resources/inputs/day14-input.txt");
        assert_eq!(4441317262452, run_day_14(input2, 40));
    }
}