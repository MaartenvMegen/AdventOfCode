use std::collections::{HashMap, HashSet};
use std::fs;

fn part_b(input: &str) -> usize {
    let grid = get_grid(input);
    let freq_map = get_freq_map(&grid);
    let antinodes = count_antinodes_b(&grid, &freq_map);
    antinodes.len()
}

fn part_a(input: &str) -> usize {
    let grid = get_grid(input);
    let freq_map = get_freq_map(&grid);
    let antinodes = count_antinodes_a(&grid, &freq_map);
    antinodes.len()
}

fn get_freq_map(grid: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut freq_map = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != '.' {
                freq_map.entry(ch).or_insert_with(Vec::new).push((x, y));
            }
        }
    }
    freq_map
}

fn count_antinodes_b(
    grid: &[Vec<char>],
    freq_map: &HashMap<char, Vec<(usize, usize)>>,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();
    for (_, antennas) in freq_map.iter() {
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                // compares antenna pairs
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;

                antinodes.insert((x1, y1));
                antinodes.insert((x2, y2));
                let mut new_loc_x1 = x1 as i32 - dx;
                let mut new_loc_y1 = y1 as i32 - dy;

                let mut new_loc_x2 = x2 as i32 + dx;
                let mut new_loc_y2 = y2 as i32 + dy;

                // while not out of bounds
                while new_loc_x1 >= 0
                    && new_loc_x1 < grid[0].len() as i32
                    && new_loc_y1 >= 0
                    && new_loc_y1 < grid.len() as i32
                {
                    antinodes.insert((new_loc_x1 as usize, new_loc_y1 as usize));
                    new_loc_x1 -= dx;
                    new_loc_y1 -= dy;
                }
                while new_loc_x2 >= 0
                    && new_loc_x2 < grid[0].len() as i32
                    && new_loc_y2 >= 0
                    && new_loc_y2 < grid.len() as i32
                {
                    antinodes.insert((new_loc_x2 as usize, new_loc_y2 as usize));
                    new_loc_x2 += dx;
                    new_loc_y2 += dy;
                }
            }
        }
    }
    antinodes
}

fn count_antinodes_a(
    grid: &[Vec<char>],
    freq_map: &HashMap<char, Vec<(usize, usize)>>,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();
    for (_, antennas) in freq_map.iter() {
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                // compares antenna pairs
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;

                let new_loc_x1 = x1 as i32 - dx;
                let new_loc_y1 = y1 as i32 - dy;

                let new_loc_x2 = x2 as i32 + dx;
                let new_loc_y2 = y2 as i32 + dy;

                // while not out of bounds
                if new_loc_x1 >= 0
                    && new_loc_x1 < grid[0].len() as i32
                    && new_loc_y1 >= 0
                    && new_loc_y1 < grid.len() as i32
                {
                    antinodes.insert((new_loc_x1 as usize, new_loc_y1 as usize));
                }
                if new_loc_x2 >= 0
                    && new_loc_x2 < grid[0].len() as i32
                    && new_loc_y2 >= 0
                    && new_loc_y2 < grid.len() as i32
                {
                    antinodes.insert((new_loc_x2 as usize, new_loc_y2 as usize));
                }
            }
        }
    }
    antinodes
}

fn main() {
    let example = fs::read_to_string(r"2024/rustaoc2024/resources/day8-example.txt").unwrap();
    let _input = fs::read_to_string(r"2024/rustaoc2024/resources/day8-input.txt").unwrap();

    println!("Part a: {}", part_a(&example));
    println!("Part b: {}", part_b(&example));
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut grid = Vec::new();
    for line in lines {
        let row = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }
    grid
}
