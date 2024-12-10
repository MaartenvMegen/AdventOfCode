use rustaoc2024::{get_input, get_map_of_things};

fn print_grid(grid: &[Vec<bool>]) {
    for row in grid {
        for cell in row {
            print!("{}", if *cell { "#" } else { "." });
        }
        println!();
    }
}

fn dfs(
    map: &[Vec<usize>],
    visited: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
    height: usize,
    allow_multipath: bool,
) -> usize {
    if map[row][col] != height  || (visited[row][col] && !allow_multipath) {
        return 0;
    }

    visited[row][col] = true;
    if height == 9 {
        print_grid(visited);
        return 1;
    }

    let mut score = 0;
    for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;
        if new_row >= 0 && new_row < map.len() as i32 && new_col >= 0 && new_col < map[0].len() as i32 {
            score += dfs(map, visited, new_row as usize, new_col as usize, height + 1, allow_multipath);
        }
    }
    score
}

fn find_trailheads(map: &[Vec<usize>], allow_multipath: bool) -> Vec<(usize, usize, usize)> {
    let mut trailheads = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                let mut visited = vec![vec![false; map[0].len()]; map.len()];
                let score = dfs(map, &mut visited, i, j, 0, allow_multipath);
                if score > 0 {
                    trailheads.push((i, j, score));
                }
            }
        }
    }
    trailheads
}

fn main() {
    let input = get_input("day10-input.txt");

    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}

fn part_a(input: &str) -> usize {
    let map = get_map_of_things(input).unwrap();
    let trailheads = find_trailheads(&map, false);
    trailheads.iter().map(|(_, _, score)| score).sum::<usize>()
}

fn part_b(input: &str) -> usize {
    let map = get_map_of_things(input).unwrap();
    let trailheads = find_trailheads(&map, true);
    trailheads.iter().map(|(_, _, score)| score).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_part_a() {
        let example = fs::read_to_string(r"./resources/day10-example.txt").unwrap();
        assert_eq!(part_a(&example), 36);
    }

    #[test]
    fn test_part_b() {
        let example = fs::read_to_string(r"./resources/day10-example.txt").unwrap();
        assert_eq!(part_b(&example), 81);
    }
}
