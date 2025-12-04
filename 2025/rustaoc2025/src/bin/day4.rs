use rustaoc2025::get_input;
use rustaoc2025::grid::{Grid, Point};

const DESIRED_SYMBOL: char = '@';

fn amount_equal_neighbours(point: &Point, grid: &Grid<char>) -> usize {
    grid.get_neighbours_diag(point)
        .iter()
        .map(|location| grid.get_value(location))
        .filter(|value| value.unwrap().eq(&DESIRED_SYMBOL))
        .count()
}

fn solve(input: &Grid<char>) -> usize {
    input
        .get_locations()
        .iter()
        .filter(|location| {
            input.get_value(location).unwrap().eq(&DESIRED_SYMBOL)
                && amount_equal_neighbours(location, input) < 4
        })
        .count()
}

fn solve2(grid: &mut Grid<char>) -> usize {
    let mut removed = 0;
    let mut locations: Vec<Point> = grid
        .get_locations()
        .into_iter()
        .filter(|location| {
            grid.get_value(location).unwrap().eq(&DESIRED_SYMBOL)
                && amount_equal_neighbours(location, grid) < 4
        })
        .collect();

    while !locations.is_empty() {
        removed += locations.len();
        locations.iter().for_each(|loc| grid.remove_loc(loc));
        locations = grid
            .get_locations()
            .into_iter()
            .filter(|location| {
                grid.get_value(location).unwrap().eq(&DESIRED_SYMBOL)
                    && amount_equal_neighbours(location, grid) < 4
            })
            .collect();
    }
    removed
}

fn main() {
    let input = get_input("day4-example.txt");
    let mut map: Grid<char> = Grid::parse_to_grid(&input, |s| s);
    println!("{}", solve(&map));
    println!("{}", solve2(&mut map));
}
