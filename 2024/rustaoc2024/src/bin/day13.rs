use rustaoc2024::get_input;

fn main() {
    let input = get_input("day13-input.txt");
    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}

fn part_a(input: &str) -> u64 {
    0
}

fn part_b(input: &str) -> u64 {
   0
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../.././resources/day13-example.txt");

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(EXAMPLE), 0);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(EXAMPLE), 0);
    }
}

