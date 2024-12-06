use std::collections::HashSet;
use std::fs;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn find_starting_position(grid: &[Vec<char>]) -> Option<Position> {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '^' {
                return Some(Position { x, y });
            }
        }
    }
    None
}

fn simulate_guard(grid: &[Vec<char>]) -> (usize, bool) {
    let mut guard_pos = find_starting_position(grid).unwrap();
    let mut guard_dir = Direction::Up;

    let mut visited = HashSet::new();
    let mut visisted_with_orientation = HashSet::new();

    visited.insert(guard_pos);

    let rows = grid.len();
    let cols = grid[0].len();
    let mut wandering = true;
    let mut loop_detected = false;

    while wandering {
        match guard_dir {
            Direction::Up => {
                // edge of grid, we are done
                if guard_pos.y == 0 {
                    wandering = false;
                }
                // lookahead to next pos for a #,  if so change dir
                else if grid[guard_pos.y - 1][guard_pos.x] == '#' {
                    guard_dir = Direction::Right;
                // else move there
                } else {
                    guard_pos.y -= 1;
                    if visisted_with_orientation.contains(&(guard_pos, guard_dir)) {
                        wandering = false;
                        loop_detected = true;
                    }
                    visisted_with_orientation.insert((guard_pos, guard_dir));
                    visited.insert(guard_pos);
                }
            }
            Direction::Right => {
                if guard_pos.x == cols - 1 {
                    wandering = false;
                }
                else if grid[guard_pos.y][guard_pos.x + 1] == '#' {
                    guard_dir = Direction::Down;
                } else {
                    guard_pos.x += 1;
                    if visisted_with_orientation.contains(&(guard_pos, guard_dir)) {
                        wandering = false;
                        loop_detected = true;
                    }
                    visisted_with_orientation.insert((guard_pos, guard_dir));
                    visited.insert(guard_pos);
                }
            }
            Direction::Down => {
                if guard_pos.y == rows -1 {
                    wandering = false;
                }
                else if grid[guard_pos.y + 1][guard_pos.x] == '#' {
                    guard_dir = Direction::Left;
                } else {
                    guard_pos.y += 1;
                    if visisted_with_orientation.contains(&(guard_pos, guard_dir)) {
                        wandering = false;
                        loop_detected = true;
                    }
                    visisted_with_orientation.insert((guard_pos, guard_dir));
                    visited.insert(guard_pos);
                }
            }
            Direction::Left => {
                if guard_pos.x == 0 {
                    wandering = false;
                }
                else if grid[guard_pos.y][guard_pos.x - 1] == '#' {
                    guard_dir = Direction::Up;
                } else {
                    guard_pos.x -= 1;
                    if visisted_with_orientation.contains(&(guard_pos, guard_dir)) {
                        wandering = false;
                        loop_detected = true;
                    }
                    visisted_with_orientation.insert((guard_pos, guard_dir));
                    visited.insert(guard_pos);
                }
            }
        }
    }

    (visited.len(), loop_detected)
}


fn part_a(input: &str) -> usize {
    let grid = parse_grid(input).unwrap();
    print_grid(&grid);
    let (positions, _loop_detected) = simulate_guard(&grid);
    positions
}

fn part_b(input: &str) -> usize {
    let grid = parse_grid(input).unwrap();
    loop_searcher(&grid)
}


fn parse_grid(input: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let lines : Vec<&str>= input.trim().split("\n").collect();

    let mut grid = Vec::new();
    for line in lines {
        let row = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }

    Ok(grid)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    // Now you can use the grid, e.g., to print it:
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
fn update_grid(grid: &mut [Vec<char>], position: Position) {
    grid[position.y][position.x] = '#';
}

fn deep_clone_grid(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    grid.to_vec()
}

fn loop_searcher(grid: &[Vec<char>]) -> usize {

    let mut loops = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '#' && grid[i][j] != '^' {
                let mut new_grid = deep_clone_grid(grid);
                update_grid(&mut new_grid, Position { x: j, y: i });
                let (_positions, loop_detected) = simulate_guard(&new_grid);
                if loop_detected {
                    loops += 1;
                }
            }
        }
    }
    loops
}
fn main() {
    let example = fs::read_to_string(r"2024/rustaoc2024/resources/day6-example.txt").unwrap();
    let _input = fs::read_to_string(r"2024/rustaoc2024/resources/day6-input.txt").unwrap();

    let positions = part_a(&example);
    println!("The guard visits {} positions.",positions);
    println!("searching loops");
    let loops = part_b(&example);
    println!("There are {} loops", loops);
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = fs::read_to_string(r"./resources/day6-example.txt").unwrap();
        assert_eq!(part_a(&example),41);
    }

    #[test]
    fn test_part2() {
        let example = fs::read_to_string(r"./resources/day6-example.txt").unwrap();
        assert_eq!(part_b(&example),6);
    }
}
