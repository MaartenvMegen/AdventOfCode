use std::collections::HashMap;

pub fn part_1(input: &Vec<String>) -> usize {
    let nr_map = get_counts_per_bit_position(input);

    let number_length = input[0].len();
    let total_numbers = input.len();

    let mut gamma_rate = String::new();
    for index in 0..number_length {
        let nr_positive_bits = nr_map.get(&index).unwrap();
        if nr_positive_bits * 2 > total_numbers {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
    }
    let gamma_rate = usize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let max_value = (1 << number_length) - 1;
    let epsilon_rate = max_value - gamma_rate;

    gamma_rate * epsilon_rate
}

pub fn part_2(input: &Vec<String>) -> usize {
    let number_length = input[0].len();

    let life_support_rating = get_winning_number(input, number_length, true);
    let co2_rating = get_winning_number(input, number_length, false);

    (life_support_rating * co2_rating) as usize
}

fn get_winning_number(input: &Vec<String>, number_length: usize, most_common: bool) -> isize {
    let mut input = input.clone();
    for index in 0..number_length {
        input = filter_input(&input, index, most_common);
        if &input.len() < &2 {
            break;
        }
    }
    let winning_nr = input.get(0).unwrap();
    let winning_nr = isize::from_str_radix(winning_nr, 2).unwrap();
    winning_nr
}

fn filter_input(input: &Vec<String>, bit: usize, most_common: bool) -> Vec<String> {
    let nr_map = get_counts_per_bit_position(input);
    let bits = &input.len();

    if most_common && (nr_map.get(&bit).unwrap() * 2) >= *bits {
        filter_on_value(input, bit, '1')
    } else if most_common {
        filter_on_value(input, bit, '0')
    } else if (nr_map.get(&bit).unwrap() * 2) < *bits {
        filter_on_value(input, bit, '1')
    } else {
        filter_on_value(input, bit, '0')
    }
}

fn get_counts_per_bit_position(input: &Vec<String>) -> HashMap<usize, usize> {
    let mut nr_map: HashMap<usize, usize> = HashMap::new();
    for line in input {
        for (index, char) in line.chars().enumerate() {
            if char == '1' {
                let counter = nr_map.entry(index).or_insert(0);
                *counter += 1;
            } else {
                nr_map.entry(index).or_insert(0);
            }
        }
    }
    nr_map
}

fn filter_on_value(input: &Vec<String>, bit: usize, value : char) -> Vec<String> {
    input
        .iter()
        .filter(|line| line.chars().nth(bit).unwrap() == value)
        .map(|str| str.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day3::{part_1, part_2};
    use crate::reader::parse_lines_to_vec;

    #[test]
    fn test_example_part1() {
        let input: Vec<String> = parse_lines_to_vec("./resources/inputs/day3-example.txt").unwrap();

        assert_eq!(198, part_1(&input));
    }

    #[test]
    fn test_example_part2() {
        let input: Vec<String> = parse_lines_to_vec("./resources/inputs/day3-example.txt").unwrap();
        assert_eq!(230, part_2(&input));
    }

    #[test]
    fn test_input_part1() {
        let input: Vec<String> = parse_lines_to_vec("./resources/inputs/day3-input.txt").unwrap();
        assert_eq!(1307354, part_1(&input));
    }

    #[test]
    fn test_input_part2() {
        let input: Vec<String> = parse_lines_to_vec("./resources/inputs/day3-input.txt").unwrap();
        assert_eq!(482500, part_2(&input));
    }
}
