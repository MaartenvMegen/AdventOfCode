use rustaoc2022::grid::{Grid, Point};
use std::fs;

const DAY: u32 = 8;

fn part1(input: &str) -> u64 {
    let grid = parse_input_to_grid(input);

    grid.get_content()
        .iter()
        .enumerate()
        .map(|(index, tree)| {
            let point = grid.index_to_point(index);
            let max_x = grid.get_size_x() - 1;
            let max_y = grid.get_size_y() - 1;

            match point {
                (x, y) if x == 0 || x == max_x || y == 0 || y == max_y => true,
                (x, y) => {
                    let orientations = get_positions_per_orientation(x, y, max_y, max_x);
                    orientations
                        .iter()
                        .map(|orientation| {
                            orientation
                                .iter()
                                .map(|pos| grid.get_item_at_pos(pos))
                                .all(|visible_tree| visible_tree < *tree)
                        })
                        .any(|visible| visible)
                }
            }
        })
        .filter(|visible| *visible)
        .count() as u64
}

fn part2(input: &str) -> u64 {
    let grid = parse_input_to_grid(input);

    grid.get_content()
        .iter()
        .enumerate()
        .map(|(index, tree)| {
            let (x, y) = grid.index_to_point(index);
            let max_y = grid.get_size_y() - 1;
            let max_x = grid.get_size_x() - 1;

            get_positions_per_orientation(x, y, max_y, max_x)
                .iter()
                .map(|orientation| get_amount_trees_visible(&grid, tree, orientation))
                .product::<usize>() as u64
        })
        .max()
        .unwrap() as u64
}

fn get_positions_per_orientation(
    x: usize,
    y: usize,
    max_y: usize,
    max_x: usize,
) -> Vec<Vec<Point>> {
    let north_pos: Vec<Point> = (0..y).rev().map(|y_range| (x, y_range)).collect();
    let south_pos: Vec<Point> = (y + 1..=max_y).map(|y_range| (x, y_range)).collect();
    let east_pos: Vec<Point> = (x + 1..=max_x).map(|x_range| (x_range, y)).collect();
    let west_pos: Vec<Point> = (0..x).rev().map(|x_range| (x_range, y)).collect();
    vec![north_pos, south_pos, east_pos, west_pos]
}

fn parse_input_to_grid(input: &str) -> Grid<u32> {
    let mut lines = input.trim().lines();
    let first = lines.next().unwrap();
    let mut contents = first
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    while let Some(item) = lines.next() {
        contents.append(
            &mut item
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        )
    }

    Grid::new(first.len(), contents)
}

fn get_amount_trees_visible(grid: &Grid<u32>, tree: &u32, positions: &Vec<Point>) -> usize {
    let large_tree_pos = positions
        .iter()
        .map(|pos| grid.get_item_at_pos(&pos))
        .position(|iter_tree| iter_tree >= *tree);

    if let Some(large_tree_pos) = large_tree_pos {
        large_tree_pos + 1
    } else {
        positions.len()
    }
}

fn main() {
    let example = fs::read_to_string(format!("../../resources/day{}-example.txt", DAY))
        .expect("Should have been able to read the file");
    let input = fs::read_to_string(format!("../../resources/day{}-input.txt", DAY))
        .expect("Should have been able to read the file");

    rustaoc2022::run_matrix(part1, part2, example.as_str(), input.as_str());
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, DAY};
    use std::fs;

    #[test]
    fn test_example() {
        let input = fs::read_to_string(format!("./resources/day{}-example.txt", DAY))
            .expect("Should have been able to read the file");
        assert_eq!(21, part1(input.as_str()));
        assert_eq!(8, part2(input.as_str()));
    }

    #[test]
    fn test_input() {
        let input = fs::read_to_string(format!("./resources/day{}-input.txt", DAY))
            .expect("Should have been able to read the file");
        assert_eq!(1849, part1(input.as_str()));
        assert_eq!(201600, part2(input.as_str()));
    }
}
