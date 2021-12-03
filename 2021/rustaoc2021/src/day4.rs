
pub fn part_1(input: &Vec<String>) -> usize {

    1 + input.len()
}

pub fn part_2(input: &Vec<String>) -> usize {
    1 + input.len()
}


#[cfg(test)]
mod tests {
    use crate::day4::{part_1, part_2};
    use crate::reader::parse_lines_to_vec;

    #[test]
    fn test_example_part1() {
        let input: Vec<String> = parse_lines_to_vec("./resources/inputs/day4-example.txt").unwrap();

        assert_eq!(0, part_1(&input));
    }

    #[test]
    fn test_example_part2() {
        let input: Vec<String> = parse_lines_to_vec("./resources/inputs/day4-example.txt").unwrap();
        assert_eq!(0, part_2(&input));
    }

    #[test]
    fn test_input_part1() {
        let input: Vec<String> = parse_lines_to_vec("./resources/inputs/day4-input.txt").unwrap();
        assert_eq!(0, part_1(&input));
    }

    #[test]
    fn test_input_part2() {
        let input: Vec<String> = parse_lines_to_vec("./resources/inputs/day4-input.txt").unwrap();
        assert_eq!(0, part_2(&input));
    }
}
