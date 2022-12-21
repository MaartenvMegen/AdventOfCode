use colored::*;
use std::collections::{HashMap, HashSet};
use std::thread::sleep;
use std::time::Duration;

const EXAMPLE: &str = include_str!(r"../../resources/day14-example.txt");
const INPUT: &str = include_str!(r"../../resources/day14-input.txt");

type Location = (i64, i64);

fn to_point(point_str: &str) -> Location {
    let (x, y) = point_str.split_once(',').unwrap();
    (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
}

fn diffs(point_a: Location, point_b: Location) -> (i64, i64) {
    let (x_a, y_a) = point_a;
    let (x_b, y_b) = point_b;
    (x_a - x_b, y_a - y_b)
}

const ROCK: char = '#';

fn drop_sand(
    grid: &mut HashMap<Location, char>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
) -> usize {
    // assume its not straight down
    let sand_origin: Location = (500, 0);
    // drop until location below,
    // check left
    // check right for sliding down. -> no options left place sand
    // continue while next available options are inside grid

    let mut current_loc = sand_origin;
    'sandloop: loop {
        current_loc = sand_origin;
        'droploop: loop {
            let offsets = vec![(0, 1), (-1, 1), (1, 1)];
            let mut found = false;
            for (x_offset, y_offset) in offsets {
                let new_loc = (current_loc.0 + x_offset, current_loc.1 + y_offset);
                //println!("now checking location: {:?}: xmin xminx {},{}", new_loc, x_min, x_max);

                if new_loc.0 < x_min || new_loc.0 > x_max || new_loc.1 > y_max {
                    //println!("index out of bounds, infite sand detected");
                    break 'sandloop;
                }

                match grid.get(&new_loc) {
                    None => {
                        // move further to allowed location, break out of for loop
                        current_loc = new_loc;
                        found = true;
                        break;
                    }
                    Some(_) => {
                        // something is blocking movement
                        // consider other offset
                        continue;
                    }
                }
            }
            // if no viable location found we place sand at current location
            if !found {
                //println!("inserting sand at location {:?}", current_loc);
                grid.insert(current_loc, 'o');
                break 'droploop;
            }
        }
    }

    let value: usize = grid
        .iter()
        .map(|(key, value)| value)
        .filter(|char| **char == 'o')
        .count();
    value
}

fn drop_sand_pt2(
    grid: &mut HashMap<Location, char>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
) -> usize {
    // assume its not straight down
    let sand_origin: Location = (500, 0);
    // drop until location below,
    // check left
    // check right for sliding down. -> no options left place sand
    // continue while next available options are inside grid

    let mut current_loc = sand_origin;
    let mut iterations = 0;
    let mut grid = grid;
    'sandloop: loop {
        current_loc = sand_origin;
        'droploop: loop {
            let offsets = vec![(0, 1), (-1, 1), (1, 1)];
            let mut found = false;

            iterations += 1;
            sleep(Duration::from_millis(100));
            if iterations % 1 == 0 {
                display_grid(grid, &(x_min - 10), &(x_max + 10), &y_max, current_loc);
            }

            for (x_offset, y_offset) in offsets {
                let new_loc = (current_loc.0 + x_offset, current_loc.1 + y_offset);
                //println!("now checking location: {:?}: xmin xminx {},{}", new_loc, x_min, x_max);

                if new_loc.1 == y_max + 2 {
                    // we hit the floor, find another spot
                    continue;
                }

                match grid.get(&new_loc) {
                    None => {
                        // move further to allowed location, break out of for loop
                        current_loc = new_loc;
                        found = true;
                        break;
                    }
                    Some(_) => {
                        // something is blocking movement
                        // consider other offset
                        continue;
                    }
                }
            }

            // if no viable location found we place sand at current location
            if !found {
                //println!("inserting sand at location {:?}", current_loc);
                grid.insert(current_loc, 'o');

                if current_loc == sand_origin {
                    //display_grid(grid, &x_min, &x_max, &y_max);
                    break 'sandloop;
                }
                break 'droploop;
            }
        }
    }

    let value: usize = grid
        .iter()
        .map(|(key, value)| value)
        .filter(|char| **char == 'o')
        .count();
    value
}

fn part1(_input: &str) -> usize {
    // 503,4 -> 502,4 -> 502,9 -> 494,9
    let (mut grid, xlocs, ylocs) = parse_to_grid(_input);
    // now lets drop some sand until nothing blocks sand from falling forever
    // this can be checked by noticing that x values are beyond the grid values
    let x_min = xlocs.iter().min().unwrap();
    let x_max = xlocs.iter().max().unwrap();
    let y_min = ylocs.iter().min().unwrap();
    let y_max = ylocs.iter().max().unwrap();

    let value = drop_sand(&mut grid, *x_min, *x_max, *y_min, *y_max);
    //println!("amount of rocks is {}", grid.len());

    //display_grid(grid, x_min, x_max, y_max);

    println!("total sand {}", value);
    value
}

fn display_grid(
    grid: &HashMap<Location, char>,
    x_min: &i64,
    x_max: &i64,
    y_max: &i64,
    current_loc: Location,
) {
    // set cursor to start

    print!("\x1B[1;1H");
    print!("{}", "|".blue());
    for _ in *x_min..*x_max + 1 {
        print!("{}", "-".blue())
    }
    print!("{}", "|\n".blue());
    for y in 0..=*y_max {
        print!("{}", "|".blue());
        for x in *x_min..=*x_max {
            let loc: Location = (x, y);
            if loc == current_loc {
                print!("{}", "o".yellow());
                continue;
            }
            match grid.get(&loc) {
                None => {
                    print!(" ")
                }
                Some(char) => match char {
                    '#' => print!("{}", char.to_string().white().bold()),
                    'o' => print!("{}", char.to_string().yellow()),
                    _ => print!("{}", char.to_string().yellow()),
                },
            }
        }
        print!("{}", "|\n".blue())
    }
    print!("{}", "|".blue());
    for _ in *x_min..*x_max + 1 {
        print!("{}", "-".blue())
    }
    print!("{}", "|\n".blue());
}

fn parse_to_grid(_input: &str) -> (HashMap<Location, char>, HashSet<i64>, HashSet<i64>) {
    let mut grid: HashMap<Location, char> = HashMap::new();
    let mut x_locs: HashSet<i64> = HashSet::new();
    let mut y_locs: HashSet<i64> = HashSet::new();

    for line in _input.lines() {
        let end_points: Vec<Location> = line.split(" -> ").map(|node| to_point(node)).collect();
        for nodes in end_points.windows(2) {
            //println!("drawing rocks from {:?} to {:?}", nodes[0], nodes[1]);
            let mut start_x = nodes[0].0;
            let mut end_x = nodes[1].0;
            let mut start_y = nodes[0].1;
            let mut end_y = nodes[1].1;

            if nodes[0].0 > nodes[1].0 {
                // decrementing lets swap x
                start_x = nodes[1].0;
                end_x = nodes[0].0;
            }
            if nodes[0].1 > nodes[1].1 {
                // decrementing lets swap y
                start_y = nodes[1].1;
                end_y = nodes[0].1;
            }

            for x_index in start_x..=end_x {
                for y_index in start_y..=end_y {
                    //println!("inserting rocks at {},{}", x_index, y_index);
                    x_locs.insert(x_index);
                    y_locs.insert(y_index);
                    grid.insert((x_index, y_index), ROCK);
                }
            }
        }
    }
    (grid, x_locs, y_locs)
}

fn part2(_input: &str) -> usize {
    // 503,4 -> 502,4 -> 502,9 -> 494,9
    let (mut grid, xlocs, ylocs) = parse_to_grid(_input);
    // now lets drop some sand until nothing blocks sand from falling forever
    // this can be checked by noticing that x values are beyond the grid values
    let x_min = xlocs.iter().min().unwrap();
    let x_max = xlocs.iter().max().unwrap();
    let y_min = ylocs.iter().min().unwrap();
    let y_max = ylocs.iter().max().unwrap();

    let value = drop_sand_pt2(&mut grid, *x_min, *x_max, *y_min, *y_max);
    //println!("amount of rocks is {}", grid.len());

    //display_grid(&mut grid, x_min, x_max, y_max);

    println!("total sand {}", value);
    value
}

fn main() {
    // clear terminal
    print!("\x1B[2J\x1B[1;1H");
    part2(INPUT);
    //rustaoc2022::run_matrix(part1, part2, EXAMPLE, INPUT);
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, EXAMPLE, INPUT};

    #[test]
    #[ignore]
    fn test_example() {
        assert_eq!(24, part1(EXAMPLE));
        assert_eq!(93, part2(EXAMPLE));
    }

    #[test]
    #[ignore]
    fn test_input() {
        assert_eq!(1001, part1(INPUT));
        assert_eq!(27976, part2(INPUT));
    }
}
