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


pub fn min_max(input : &Vec<i64>) -> (i64, i64) {
    let min_max = (i64::MAX, i64::MIN);
    input.iter().fold( min_max, |(mut min, mut max), nr| {
        if min > *nr {
            min = *nr
        }
        if  max < *nr {
            max = *nr
        }
        (min,max)
    })
}



#[cfg(test)]
mod tests {
    use crate::calculator::min_max;

    #[test]
    fn test_min_max() {
        let input: Vec<i64> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let (min, max) = min_max(&input);
        assert_eq!(0, min);
        assert_eq!(16, max);
    }
}