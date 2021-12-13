use crate::grid::{Grid, Point};
use crate::reader::get_lines;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::iter::FromIterator;
use std::str::FromStr;

fn parse_to_grid(input: Lines<BufReader<File>>) -> Grid {
    let mut grid = Grid::new();
    for (y, line) in input.enumerate() {
        let line = line.unwrap();
        //println!("{:?}", line.clone().trim().chars().collect::<Vec<char>>());
        for (x, nr) in line
            .trim()
            .chars()
            .map(|s| u64::from_str(&*s.to_string()).unwrap())
            .enumerate()
        {
            grid.add_to_grid(Point::new(x as isize, y as isize), nr);
        }
    }
    grid
}

pub fn part_1(filename: &str) -> usize {
    let input = get_lines(filename);
    let mut grid = parse_to_grid(input);

    let mut total = 0;
    for _ in 0..100 {
        let sparked = perform_round(&mut grid);
        total += sparked.len()
    }

    println!("found {} sparks", total);
    total
}

pub fn part_2(filename: &str) -> usize {
    let input = get_lines(filename);
    let mut grid = parse_to_grid(input);

    let mut index = 0;
    loop {
        let sparked = perform_round(&mut grid);
        index += 1;
        if sparked.len() == grid.get_map().keys().len() {
            break;
        }
    }

    println!("after {} iterations", index);
    index
}

fn perform_round(mut grid: &mut Grid) -> HashSet<Point> {
    // round starts with increment
    increment(&mut grid);
    let mut sparked: HashSet<Point> = HashSet::new();

    loop {
        let new_sparks: HashSet<Point> = HashSet::from_iter(
            grid.get_map()
                .iter()
                .filter(|(_k, v)| v > &&(9 as u64))
                .map(|(key, _value)| key)
                .filter(|point| !sparked.contains(*point))
                .cloned()
                .collect::<Vec<Point>>(),
        );

        if new_sparks.is_empty() {
            break;
        }

        // each loc can occur multiple times
        let incrementable_locs = new_sparks
            .iter()
            .map(|loc| grid.get_neighbours_diag(loc))
            .flatten()
            .collect::<Vec<Point>>();

        // increment perhaps even multiple times
        for loc in incrementable_locs {
            grid.increment_loc(&loc, 1);
        }

        sparked.extend(new_sparks);
    }

    // add current set to sparks
    for loc in &sparked {
        grid.update_loc(loc.clone(), 0);
    }
    sparked
}

fn increment(grid: &mut Grid) {
    let locs = grid.get_locations();
    for loc in locs {
        grid.increment_loc(&loc, 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::{part_1, part_2};

    #[test]
    fn test_part1() {
        assert_eq!(1656, part_1("./resources/inputs/day11-example.txt"));
        assert_eq!(1741, part_1("./resources/inputs/day11-input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(195, part_2("./resources/inputs/day11-example.txt"));
        assert_eq!(440, part_2("./resources/inputs/day11-input.txt"));
    }
}
