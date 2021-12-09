use std::collections::{HashMap};
use std::str::{FromStr};

#[derive(Debug, PartialEq)]
pub struct ParseError;

#[derive(Debug, PartialEq)]
pub struct SevenSegmentInfo {
    input: Vec<String>,
    output: Vec<String>,
}

impl FromStr for SevenSegmentInfo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let in_out: Vec<&str> = s.split(" | ").collect();

        let input = in_out[0].split(" ").map(|x| sort_string(&x.to_string())).collect();
        let output = in_out[1].split(" ").map(|x| sort_string(&x.to_string())).collect();

        Ok(Self { input, output })
    }
}

pub fn part_1(input: &Vec<SevenSegmentInfo>) -> usize {
    let easy_digits: Vec<usize> = vec![2, 3, 4, 7]; // 1, 7, 4, 8
    let mut counter = 0;

    for spec in input {
        for nr in &spec.output {
            if easy_digits.contains(&nr.len()) {
                counter += 1
            }
        }
    }
    counter
}

pub fn decode(segment_info: &SevenSegmentInfo) -> usize {
    // identify the 1,4,7,8 digits
    let easy_digits: Vec<usize> = vec![2, 3, 4, 7]; // 1, 7, 4, 8
    let mut length_to_digit: HashMap<usize, usize> = HashMap::new();
    length_to_digit.insert(2, 1);
    length_to_digit.insert(3, 7);
    length_to_digit.insert(4, 4);
    length_to_digit.insert(7, 8);

    // convert output string to number
    let mut digit_map: HashMap<usize, String> = HashMap::new();

    segment_info
        .input
        .iter()
        .filter(|wires| easy_digits.contains(&wires.len()))
        .for_each(|wires| {
            digit_map.insert(*length_to_digit.get(&wires.len()).unwrap(), wires.clone());
        });

    segment_info.input.iter().filter( | wires| !easy_digits.contains(&wires.len())).for_each( | wires| {
        if wires.len() == 5 {
            if one_subset_other(digit_map.get(&7).unwrap(), wires) {
                digit_map.insert(3, wires.clone());
            } else if count_matching(digit_map.get(&4).unwrap(), wires) == 2 {
                // 2 matching chars with 4 -> 2
                digit_map.insert(2, wires.clone());
            } else {
                digit_map.insert(5, wires.clone());
            }
        }
        else if wires.len() == 6 {
            if count_matching(digit_map.get(&4).unwrap(), wires) == 3 && count_matching(digit_map.get(&1).unwrap(), wires) == 1{
                //println!("found a 6 for {} matched to {}",wires, digit_map.get(&4).unwrap());
                digit_map.insert(6, wires.clone());
            } else if count_matching(digit_map.get(&4).unwrap(), wires) == 4 &&  count_matching(digit_map.get(&7).unwrap(), wires) == 3{
                //println!("found a 9 for {}",wires);
                digit_map.insert(9,wires.clone());
            } else {
                digit_map.insert(0, wires.clone());
            }
        }


    });
    //
    let decode_map : HashMap<String, usize> = digit_map.iter().map( | (key, value)| (sort_string(value),*key)).collect();
    //println!("{:?}", decode_map);
    //println!("{:?}", segment_info.output);
    //println!("{:?} - {:?}", segment_info.input, segment_info.output);

    let result : Vec<usize> = segment_info.output.iter().map( | wires | *decode_map.get(wires).unwrap()).collect();
    let mut output= result[3];
    output += result[2]*10;
    output += result[1]*100;
    output += result[0]*1000;


    //println!("{:?}", output);

    output
}

fn count_matching(one: &str, another: &str) -> usize {
    another.chars().map(|x| x.to_string()).filter(|x| one.contains(x)).count()
}

fn one_subset_other(one: &str, other: &str) -> bool {
    one.chars().map(|x| x.to_string()).filter(|x| other.contains(x)).count() == one.len()
}

fn sort_string(value: &str) -> String {
    let mut output = value.chars().map(|x| x.to_string()).collect::<Vec<String>>();
    output.sort();
    output.concat()
}

#[cfg(test)]
mod tests {
    use crate::day8::{decode, part_1, SevenSegmentInfo, count_matching, sort_string};
    use crate::reader::parse_lines_to_vec;


    #[test]
    fn test_contains() {
        let value = "abfgh";
        let another = "ba";
        assert!(count_matching(value, another) == 2)
    }

    #[test]
    fn test_sort() {
        let value = "fgabh";
        let iets  : String = sort_string(value);
        assert_eq!("abfgh", iets)
    }



    #[test]
    fn test_parser() {
        let command_str =
            "caebgd dagc eabgd aebfgc fbdacge edg dg dbgcef eabfd cgeba | gbcdae dagc acgd gd";
        let info: SevenSegmentInfo = command_str.parse().unwrap();
        assert_eq!("abcdeg", info.input[0])
    }

    #[test]
    fn test_decode() {
        let command_str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let info: SevenSegmentInfo = command_str.parse().unwrap();
        let code: usize = decode(&info);
        assert_eq!(5353, code);
        let command_str = "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef";
        let info: SevenSegmentInfo = command_str.parse().unwrap();
        let code: usize = decode(&info);
        assert_eq!(1625, code);

    }

    #[test]
    fn test_part1() {
        let input: Vec<SevenSegmentInfo> =
            parse_lines_to_vec("./resources/inputs/day8-example.txt").unwrap();
        assert_eq!(26, part_1(&input));
        let input: Vec<SevenSegmentInfo> =
            parse_lines_to_vec("./resources/inputs/day8-input.txt").unwrap();
        assert_eq!(519, part_1(&input));
    }


    #[test]
    fn test_input_part2() {
        let input: Vec<SevenSegmentInfo> = parse_lines_to_vec("./resources/inputs/day8-example.txt").unwrap();
        let answer : usize = input.iter().map(|segment| decode(segment)).sum();
        assert_eq!(61229, answer);

        let input: Vec<SevenSegmentInfo> = parse_lines_to_vec("./resources/inputs/day8-input.txt").unwrap();
        let answer : usize = input.iter().map(|segment| decode(segment)).sum();
        assert_eq!(1027483, answer);
    }
}
