use std::collections::{HashSet, VecDeque};
use rustaoc2024::get_input;

fn main() {
    let input = get_input("day12-input.txt");
    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}

fn part_a(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for r in 0..rows {
        for c in 0..cols {
            if !visited.contains(&(r, c)) {
                let plant = grid[r][c];
                let (area, perimeter) = explore_region(&grid, &mut visited, r, c, plant);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

fn explore_region(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
    start_r: usize,
    start_c: usize,
    plant: char,
) -> (u64, u64) {
    let mut queue = VecDeque::new();
    queue.push_back((start_r, start_c));
    visited.insert((start_r, start_c));

    let mut area = 0;
    let mut perimeter = 0;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((r, c)) = queue.pop_front() {
        area += 1;
        for (dr, dc) in directions.iter() {
            let new_r = r as isize + dr;
            let new_c = c as isize + dc;

            if new_r < 0 || new_r >= grid.len() as isize || new_c < 0 || new_c >= grid[0].len() as isize {
                perimeter += 1; // Edge of the grid
            } else {
                let new_r = new_r as usize;
                let new_c = new_c as usize;
                if grid[new_r][new_c] != plant {
                    perimeter += 1; // Adjacent to a different plant type
                } else if !visited.contains(&(new_r, new_c)) {
                    visited.insert((new_r, new_c));
                    queue.push_back((new_r, new_c));
                }
            }
        }
    }

    (area, perimeter)
}

fn part_b(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    calculate_total_price(&grid)

}

fn calculate_total_price(grid: &Vec<Vec<char>>) -> u64 {
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if !visited.contains(&(r, c)) {
                let region_char = grid[r][c];
                let (area, sides) = flood_fill_and_corners(grid, r, c, region_char, &mut visited);
                total_price += area * sides;
            }
        }
    }

    total_price
}
fn flood_fill_and_corners(
    grid: &Vec<Vec<char>>,
    start_r: usize,
    start_c: usize,
    region_char: char,
    visited: &mut HashSet<(usize, usize)>,
) -> (u64, u64) {
    let mut stack = vec![(start_r, start_c)];
    let mut area = 0;
    let mut corners = HashSet::new(); // Track unique corners

    while let Some((r, c)) = stack.pop() {
        if !visited.insert((r, c)) {
            continue; // Already visited
        }

        area += 1;

        // Check each corner of the current cell
        let mut count = 0;
        for corner in cell_corners(r, c) {
            if is_outer_corner(grid, r, c, corner, region_char) || is_inner_corner(grid, r, c, corner, region_char) {
                corners.insert(corner);
                count += 1;
            }
        }
        println!("location {} {} has {} corners", r, c, count);

        // Add neighbors to the stack for flood fill
        for (nr, nc) in neighbors(r, c, grid.len(), grid[0].len()) {
            if grid[nr][nc] == region_char && !visited.contains(&(nr, nc)) {
                stack.push((nr, nc));
            }
        }
    }
    println!("area of type {} has {} corners", region_char, corners.len());
    (area, corners.len() as u64)
}

fn neighbors(r: usize, c: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    if r > 0 {
        result.push((r - 1, c));
    }
    if r + 1 < rows {
        result.push((r + 1, c));
    }
    if c > 0 {
        result.push((r, c - 1));
    }
    if c + 1 < cols {
        result.push((r, c + 1));
    }

    result
}

fn cell_corners(r: usize, c: usize) -> Vec<(usize, usize)> {
    vec![
        (r, c),       // Top-left corner
        (r, c + 1),   // Top-right corner
        (r + 1, c),   // Bottom-left corner
        (r + 1, c + 1), // Bottom-right corner
    ]
}

fn is_outer_corner(
    grid: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    corner: (usize, usize),
    region_char: char,
) -> bool {
    let (cr, cc) = corner;

    // Top-left corner conditions
    if corner == (r, c) {
        return (r == 0 || grid[r - 1][c] != region_char) &&
            (c == 0 || grid[r][c - 1] != region_char) &&
            (r == 0 || c == 0 || grid[r - 1][c - 1] != region_char);
    }

    // Top-right corner conditions
    if corner == (r, c + 1) {
        return (r == 0 || grid[r - 1][c] != region_char) &&
            (c + 1 >= grid[0].len() || grid[r][c + 1] != region_char) &&
            (r == 0 || c + 1 >= grid[0].len() || grid[r - 1][c + 1] != region_char);
    }

    // Bottom-left corner conditions
    if corner == (r + 1, c) {
        return (r + 1 >= grid.len() || grid[r + 1][c] != region_char) &&
            (c == 0 || grid[r][c - 1] != region_char) &&
            (r + 1 >= grid.len() || c == 0 || grid[r + 1][c - 1] != region_char);
    }

    // Bottom-right corner conditions
    if corner == (r + 1, c + 1) {
        return (r + 1 >= grid.len() || grid[r + 1][c] != region_char) &&
            (c + 1 >= grid[0].len() || grid[r][c + 1] != region_char) &&
            (r + 1 >= grid.len() || c + 1 >= grid[0].len() || grid[r + 1][c + 1] != region_char);
    }

    false
}

fn is_inner_corner(
    grid: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    corner: (usize, usize),
    region_char: char,
) -> bool {
    let (cr, cc) = corner;

    // Top-left inner corner conditions
    if corner == (r, c) {
        return (r > 0 && c > 0 && grid[r - 1][c-1] == region_char) &&
            (c > 0 && grid[r][c - 1] == region_char) &&
            (r > 0 && c > 0 && grid[r - 1][c - 1] != region_char);
    }

    // Top-right inner corner conditions
    if corner == (r, c + 1) {
        return (c + 1 < grid[0].len() && grid[r][c+1] == region_char) &&
            (r > 0 && grid[r-1][c] != region_char) &&
            (r + 1 < grid.len() && c + 1 < grid[0].len() && grid[r + 1][c + 1] == region_char);
    }

    // Bottom-left inner corner conditions
    if corner == (r + 1, c) {
        return (r > 0  && grid[r-1][c] != region_char) &&
            (c > 0 && grid[r][c - 1] == region_char) &&
            (r >0  && c > 0 && grid[r - 1][c - 1] == region_char);
    }

    // Bottom-right inner corner conditions
    if corner == (r + 1, c + 1) {
        return (c+1 < grid[0].len() && grid[r][c+1] == region_char) &&
            (r > 0 && grid[r-1][c] != region_char) &&
            (r > 0  && c + 1 < grid[0].len() && grid[r - 1][c+1] == region_char);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../.././resources/day12-example.txt");

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(EXAMPLE), 1930);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(EXAMPLE), 1206);
    }
}

