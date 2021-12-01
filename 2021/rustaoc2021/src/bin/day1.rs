use rustaoc2021::reader::parse_lines_to_vec;
use std::time::Instant;

fn main() {
    let input: Vec<u64> = parse_lines_to_vec("./resources/inputs/day1a-input.txt").unwrap();
    run_part_1(&input);
    run_part_2(&input);
}

fn run_part_1(input : &Vec<u64>) {
    let now = Instant::now();
    let answer = get_increases(&input, 1);
    println!(
        "part 1: {}, result found in {} ms",
        answer,
        now.elapsed().as_millis()
    );
}

fn run_part_2(input : &Vec<u64>) {
    let now = Instant::now();
    let answer = get_increases(&input, 3);
    println!(
        "part 2: {}, result found in {} ms",
        answer,
        now.elapsed().as_millis()
    );
}

fn get_increases(input: &Vec<u64>, window_size: usize) -> usize {
    input
        .windows(window_size + 1)
        .filter(|window| window[window_size] > window[0])
        .count()
}

