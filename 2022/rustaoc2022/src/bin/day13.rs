
const EXAMPLE: &str = include_str!(r"../../resources/day13-example.txt");
const INPUT: &str = include_str!(r"../../resources/day13-input.txt");


fn part1(_input: &str) -> u64 {
    0
}

fn part2(_input: &str) -> u64 {
    0
}



fn main() {
    rustaoc2022::run_matrix(part1, part2, EXAMPLE, INPUT);
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, EXAMPLE, INPUT};

    #[test]
    #[ignore]
    fn test_example() {
        assert_eq!(31, part1(EXAMPLE));
        assert_eq!(29, part2(EXAMPLE));
    }

    #[test]
    #[ignore]
    fn test_input() {
        assert_eq!(456, part1(INPUT));
        assert_eq!(454, part2(INPUT));
    }
}
