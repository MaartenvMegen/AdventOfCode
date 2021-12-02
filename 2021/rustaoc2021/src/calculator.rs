use std::fmt::Debug;
use std::time::Instant;

pub fn run_timed<T, X>(f: fn(T) -> X, argument: T, part: u64)
where
    X: Debug,
{
    let now = Instant::now();

    let answer = f(argument);

    println!(
        "part {}: {:?}, result found in {} ms",
        part,
        answer,
        now.elapsed().as_millis()
    );
}
