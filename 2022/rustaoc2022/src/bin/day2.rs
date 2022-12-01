fn main() {
    let example = include_str!(r"../../resources/day1-example.txt");
    let input = include_str!(r"../../resources/day1-input.txt");

    println!("Example part a: {}",day1_part1(example));
    println!("Input part a: {}", day1_part1(input));

    println!("Example part b: {}",day1_part2(example));
    println!("Input part b: {}", day1_part2(input));
}


