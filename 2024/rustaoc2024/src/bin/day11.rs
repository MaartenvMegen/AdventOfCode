use rustaoc2024::get_input;

fn main() {
    let input = get_input("day11-input.txt");
    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}


fn part_a(input: &str) -> usize {
    let mut stones : Vec<u64> = input.trim().split_whitespace().map(|number| number.to_string().parse::<u64>().unwrap()).collect();

    //let mut stones = vec![125 ,17];

    for blink in 1..=25 {
        println!("Blink {}: {:?}", blink, stones);
        stones = simulate_blink(&stones);
    }
    stones.len()
}

fn simulate_blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();

    for &stone in stones {
        if stone == 0 {
            // Rule 1: Replace 0 with 1
            new_stones.push(1);
        } else if is_even_digits(stone) {
            // Rule 2: Split into two stones
            let digits = stone.to_string();
            let mid = digits.len() / 2;
            let left = digits[..mid].parse::<u64>().unwrap_or(0);
            let right = digits[mid..].parse::<u64>().unwrap_or(0);
            new_stones.push(left);
            new_stones.push(right);
        } else {
            // Rule 3: Multiply by 2024
            new_stones.push(stone * 2024);
        }
    }

    new_stones
}

fn is_even_digits(number: u64) -> bool {
    number.to_string().len() % 2 == 0
}

fn part_b(input: &str) -> usize {
    // even nmbers double
    // odd numbers get evened
    //
    0
}


#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!(r"../.././resources/day11-example.txt");

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&EXAMPLE), 36);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&EXAMPLE), 81);
    }
}

