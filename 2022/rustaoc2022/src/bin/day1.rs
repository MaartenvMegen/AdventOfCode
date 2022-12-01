use std::str::FromStr;

fn day1_part1(input: &str) -> u64 {
    let parts: Vec<&str> = rustaoc2022::create_chunks(input);
    let calories_per_elf: Vec<u64> = get_calories_per_elf(parts);
    calories_per_elf.into_iter().max().unwrap()
}

fn day1_part2(input : &str) -> u64 {
    let parts: Vec<&str> = rustaoc2022::create_chunks(input);
    let mut calories_per_elf: Vec<u64> = get_calories_per_elf(parts);
    three_heighest_elf_sum(&mut calories_per_elf)
}

fn three_heighest_elf_sum(calories_per_elf: &mut Vec<u64>) -> u64 {
    calories_per_elf.sort();
    calories_per_elf.iter().rev().take(3).sum()
}

fn get_total_calories_per_elf(chunk: &str) -> u64 {
    chunk.split("\n").map(|value | u64::from_str(value).unwrap()).sum()
}

fn get_calories_per_elf(parts: Vec<&str>) -> Vec<u64> {
    parts.iter().map(|spec| get_total_calories_per_elf(spec)).collect()
}

fn main() {
    let example = include_str!(r"../../resources/day1-example.txt");
    let input = include_str!(r"../../resources/day1-input.txt");
    println!("Example part a: {}",day1_part1(example));
    println!("Input part a: {}", day1_part1(input));

    println!("Example part b: {}",day1_part2(example));
    println!("Input part b: {}", day1_part2(input));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use rust::create_chunks;
    use crate::bin::day1::{create_chunks, day1_part1, get_calories_per_elf, get_total_calories_per_elf};
    use crate::{day1_part1, get_calories_per_elf};

    #[test]
    fn test_digits() {
        println!("{}", u64::from_str("1000").unwrap())
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(24000, day1_part1("../../resources/day1-example.txt"));
    }

    #[test]
    fn test_example_part2() {
        let input = include_str!(r"../../resources/day1-example.txt");
        let parts: Vec<&str> = create_chunks(input);
        let mut calories_per_elf: Vec<u64> = get_calories_per_elf(parts);
        calories_per_elf.sort();
        assert_eq!(45000 as u64, calories_per_elf.iter().rev().take(3).sum());
    }

    #[test]
    fn test_input_part1() {
        assert_eq!(66306, day1_part1("../../resources/day1-example.txt"));
    }

    #[test]
    fn test_input_part2() {
        let input = include_str!(r"../../resources/day1-input.txt");
        let parts: Vec<&str> = create_chunks(input);
        let mut values: Vec<u64> = get_calories_per_elf(parts);
        values.sort();
        assert_eq!(195292 as u64, values.iter().rev().take(3).sum());
    }


}

