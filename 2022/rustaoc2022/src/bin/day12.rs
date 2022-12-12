use rustaoc2022::grid::{Grid, Point};
use std::collections::HashSet;

const EXAMPLE: &str = include_str!(r"../../resources/day12-example.txt");
const INPUT: &str = include_str!(r"../../resources/day12-input.txt");

const DIRECTIONS: [(i64, i64); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

enum Target {
    Location(Point),
    Value(i32),
}

fn part1(input: &str) -> u64 {
    let (grid, start, end) = get_grid_with_start_and_end(input);
    search(
        &grid,
        start,
        Target::Location(end),
        valid_neighbour_forwards,
    )
}

fn part2(input: &str) -> u64 {
    let (grid, _start, end) = get_grid_with_start_and_end(input);
    search(
        &grid,
        end,
        Target::Value('a' as i32),
        valid_neighbour_backwards,
    )
}

fn parse_input_to_grid(input: &str) -> Grid<i32> {
    let mut lines = input.trim().lines();
    let first = lines.next().unwrap();
    let mut contents = first.chars().map(|char| char as i32).collect::<Vec<i32>>();

    while let Some(item) = lines.next() {
        contents.append(&mut item.chars().map(|char| char as i32).collect::<Vec<i32>>())
    }

    Grid::new(first.len(), contents)
}

fn get_grid_with_start_and_end(input: &str) -> (Grid<i32>, Point, Point) {
    let mut grid = parse_input_to_grid(input);
    let start = grid.index_to_point(
        grid.get_content()
            .iter()
            .position(|char| *char == 'S' as i32)
            .unwrap(),
    );
    let end = grid.index_to_point(
        grid.get_content()
            .iter()
            .position(|char| *char == 'E' as i32)
            .unwrap(),
    );
    grid.set_item_at_pos(&start, 'a' as i32);
    grid.set_item_at_pos(&end, 'z' as i32);
    (grid, start, end)
}

fn valid_neighbour_forwards(current: i32, considered: i32) -> bool {
    considered - current <= 1
}

fn valid_neighbour_backwards(current: i32, considered: i32) -> bool {
    current - considered <= 1
}

fn search(
    grid: &Grid<i32>,
    start_point: Point,
    target: Target,
    valid_neighbour_fn: fn(i32, i32) -> bool,
) -> u64 {
    let mut visited_locs: HashSet<Point> = HashSet::new();
    let mut search_edge: HashSet<Point> = HashSet::new();
    search_edge.insert(start_point);
    let mut distance = 0;

    'searchloop: loop {
        distance += 1;
        let mut new_nodes: HashSet<Point> = HashSet::new();
        for loc in search_edge {
            visited_locs.insert(loc);
            let current_height = grid.get_item_at_pos(&loc);

            for (x_offset, y_offset) in &DIRECTIONS {
                let new_x = (loc.0 as i64 + x_offset) as usize;
                let new_y = (loc.1 as i64 + y_offset) as usize;

                // check if coordinate exists (so not 1 above max or wrapped around negative value)
                if new_y >= grid.get_size_y() || new_x >= grid.get_size_x() {
                    continue;
                }

                let new_loc: Point = (new_x, new_y);
                let new_height = grid.get_item_at_pos(&new_loc);

                if valid_neighbour_fn(current_height, new_height) {
                    new_nodes.insert(new_loc);
                    match target {
                        Target::Location(point) => {
                            if point == new_loc {
                                break 'searchloop;
                            }
                        }
                        Target::Value(value) => {
                            if value == new_height {
                                break 'searchloop;
                            }
                        }
                    }
                }
            }
        }
        search_edge = new_nodes;
    }
    println!("finished searching, distance to end is: {}", distance);

    distance
}

fn main() {
    rustaoc2022::run_matrix(part1, part2, EXAMPLE, INPUT);
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, EXAMPLE, INPUT};

    #[test]
    fn test_example() {
        assert_eq!(31, part1(EXAMPLE));
        assert_eq!(29, part2(EXAMPLE));
    }

    #[test]
    fn test_input() {
        assert_eq!(456, part1(INPUT));
        assert_eq!(454, part2(INPUT));
    }
}
