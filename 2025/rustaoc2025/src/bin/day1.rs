use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Parsed {
    dir: Direction,
    value: i64,
}

impl Parsed {
    fn parse(input: &str) -> Result<Self, String> {
        let mut chars = input.chars();

        // 1. Extract the first character (direction)
        let dir = match chars.next() {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            Some(c) => return Err(format!("Invalid direction '{}'", c)),
            None => return Err("Empty string".to_string()),
        };

        // 2. Parse the rest as digits
        let digits: String = chars.collect();
        let value: i64 = digits
            .parse()
            .map_err(|_| format!("Invalid digits '{}'", digits))?;

        Ok(Parsed { dir, value })
    }
}

fn solve(input: &str) -> i64 {
    let parts: Vec<&str> = input.trim().split('\n').collect();

    let start_nr = 50;
    let mut current_nr = start_nr;
    let mut crossings = 0;

    for parts in parts {
        let parsed = Parsed::parse(parts).unwrap();
        // println!("{:?}", parsed);

        match parsed.dir {
            Direction::Left => {
                let impact: i64 = if current_nr > parsed.value {
                    0
                } else {
                    if current_nr == 0 {
                        -(current_nr - parsed.value) / 100
                    } else {
                        1 - (current_nr - parsed.value) / 100
                    }
                };
                println!("amount of zero crossings is {}", impact);
                crossings += impact
            }
            Direction::Right => {
                let impact: i64 = ((current_nr + parsed.value) / 100);
                println!("amount of zero crossings is {}", impact);
                crossings += impact
            }
        }

        current_nr = match parsed.dir {
            Direction::Right => (current_nr + parsed.value).rem_euclid(100),
            Direction::Left => (current_nr - parsed.value).rem_euclid(100),
        };

        println!(
            "Now moving by {:?} by {} point to {:?}",
            parsed.dir, parsed.value, current_nr
        );
    }
    crossings
}
fn main() {
    let input = fs::read_to_string("2025/rustaoc2025/resources/day1-input.txt").unwrap();
    let example = fs::read_to_string("2025/rustaoc2025/resources/day1-example.txt").unwrap();
    println!("{}", solve(input.as_str()));
}
