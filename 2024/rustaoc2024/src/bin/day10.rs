use rustaoc2024::{get_input, get_map_of_things, print_grid};

fn dfs(
    map: &[Vec<usize>],
    visited: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
    height: usize,
    allow_multipath: bool,
) -> usize {
    if map[row][col] != height || (visited[row][col] && !allow_multipath) {
        return 0;
    }

    visited[row][col] = true;
    if height == 9 {
        print_grid(visited, |&n| if n { '#' } else { '.' });
        return 1;
    }

    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .filter_map(|&(dr, dc)| {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            if is_in_bounds(map, new_row, new_col) {
                Some(dfs(
                    map,
                    visited,
                    new_row as usize,
                    new_col as usize,
                    height + 1,
                    allow_multipath,
                ))
            } else {
                None
            }
        })
        .sum()
}

fn is_in_bounds(map: &[Vec<usize>], row: i32, col: i32) -> bool {
    row >= 0 && row < map.len() as i32 && col >= 0 && col < map[0].len() as i32
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
    use super::*;

    #[test]
    fn test_part_a() {
        let example = include_str!(r"../../resources/day10-example.txt");
        assert_eq!(part_a(example), 36);
    }

    #[test]
    fn test_part_b() {
        let example = include_str!(r"../../resources/day10-example.txt");
        assert_eq!(part_b(example), 81);
    }
}
