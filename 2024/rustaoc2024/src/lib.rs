mod pathfinder;

use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

pub fn create_chunks(input: &str) -> Vec<&str> {
    input.trim().split("\n\n").collect()
}

pub fn create_parts(input: &str) -> Vec<&str> {
    input.trim().split('\n').collect()
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

pub fn get_input(filename: &str) -> String {
    fs::read_to_string("2024/rustaoc2024/resources/".to_owned() + filename).unwrap()
}

pub fn get_map_of_things<T: FromStr>(input: &str) -> Result<Vec<Vec<T>>, <T as FromStr>::Err>
where
    <T as FromStr>::Err: Debug,
{
    let mut grid = Vec::new();
    for line in input.trim().split('\n') {
        let row: Vec<T> = line
            .chars()
            .map(|c| c.to_string().parse::<T>().unwrap())
            .collect();
        grid.push(row);
    }
    Ok(grid)
}

pub fn print_grid<T>(grid: &[Vec<T>], to_char: fn(&T) -> char) {
    println!("grid size:  {},{}", grid.len(), grid[0].len());
    print!("┏");
    for _ in 0..grid[0].len() {
        print!("┳");
    }
    println!("┓");
    for row in grid {
        print!("┣");
        for cell in row {
            print!("{}", to_char(cell));
        }
        print!("┫");
        println!();
    }
    print!("┗");
    for _ in 0..grid[0].len() {
        print!("┻");
    }
    println!("┛");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let int_grid = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec()].to_vec();
        print_grid(&int_grid, |&n| char::from_digit(n as u32, 10).unwrap());
    }
}
