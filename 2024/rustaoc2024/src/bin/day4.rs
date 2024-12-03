use rustaoc2024::create_parts;
use std::fs;

const EXAMPLE_STRING: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";

fn part_a(input: &str) -> usize {
    let _parts: Vec<&str> = create_parts(input);

    0
}

// fn part_b(input: &str) -> usize {
//     0
// }

fn main() {
    let _input = fs::read_to_string(r"2024/rustaoc2024/resources/day4-input.txt").unwrap();
    part_a(EXAMPLE_STRING);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part_a(EXAMPLE_STRING), 0);
    }

}
