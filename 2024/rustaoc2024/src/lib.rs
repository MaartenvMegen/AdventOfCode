
use std::fmt::Debug;
use std::time::Instant;

pub fn create_chunks(input: &str) -> Vec<&str> {
    input.trim().split("\n\n").collect()
}

pub fn run_timed<T, X>(f: fn(T) -> X, argument: T, description: &str)
where
    X: Debug,
{
    let now = Instant::now();

    let answer = f(argument);

    println!(
        "{}: {:?}, result found in {} ms",
        description,
        answer,
        now.elapsed().as_millis()
    );
}


pub fn run_matrix<T: ?Sized, X>(part1: fn(&T) -> X, part2: fn(&T) -> X, example: &T, input: &T)
where
    X: Debug,
{
    run_timed(part1, example, "part 1 example");
    run_timed(part1, input, "part 1 input");
    run_timed(part2, example, "part 2 example");
    run_timed(part2, input, "part 2 input");
}