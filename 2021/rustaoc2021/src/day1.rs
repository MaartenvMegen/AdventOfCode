
pub fn part_1(input : &Vec<u64>) -> usize {
    get_increases(&input, 1)
}

pub fn part_2(input : &Vec<u64>) -> usize {
    get_increases(&input, 3)
}

fn get_increases(input: &Vec<u64>, window_size: usize) -> usize {
    input
        .windows(window_size + 1)
        .filter(|window| window[window_size] > window[0])
        .count()
}



#[cfg(test)]
mod tests {
    use crate::reader::parse_lines_to_vec;
    use crate::day1::{part_1, part_2};


    #[test]
    fn test_example_part1() {
        let input: Vec<u64> =
            parse_lines_to_vec("./resources/inputs/day1-example.txt").unwrap();
        assert_eq!(7, part_1(&input));
    }

    #[test]
    fn test_example_part2() {
        let input: Vec<u64> =
            parse_lines_to_vec("./resources/inputs/day1-example.txt").unwrap();
        assert_eq!(5, part_2(&input));
    }

    #[test]
    fn test_input_part1() {
        let input: Vec<u64> = parse_lines_to_vec("./resources/inputs/day1-input.txt").unwrap();
        assert_eq!(1532, part_1(&input));
    }

    #[test]
    fn test_input_part2() {
        let input: Vec<u64> = parse_lines_to_vec("./resources/inputs/day1-input.txt").unwrap();
        assert_eq!(1571, part_2(&input));
    }
}

