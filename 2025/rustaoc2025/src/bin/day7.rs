use rustaoc2025::get_input;
use rustaoc2025::grid::{Grid, Point};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Beam,     // '|'
    Start,    // 'S'
    Splitter, // '^'
    Empty,    // '.'
}

impl Cell {
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            '|' => Ok(Cell::Beam),
            'S' => Ok(Cell::Start),
            '^' => Ok(Cell::Splitter),
            '.' => Ok(Cell::Empty),
            other => Err(format!("invalid map symbol: '{}'", other)),
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Cell::Beam => '|',
            Cell::Start => 'S',
            Cell::Splitter => '^',
            Cell::Empty => '.',
        };
        write!(f, "{ch}")
    }
}

fn solve(input: &str) -> (u64, u64) {
    let grid: Grid<Cell> = Grid::parse_to_grid(input, |c| Cell::from_char(c).unwrap());
    let (source, _cell) = grid
        .get_map()
        .iter()
        .find(|(_location, object)| **object == Cell::Start)
        .unwrap();
    let mut memory: HashMap<Point, u64> = HashMap::new();
    let timelines = find_timelines(&grid, source, &mut memory);
    (memory.len() as u64, timelines)
}

fn find_timelines(
    grid: &Grid<Cell>,
    current_point: &Point,
    memory: &mut HashMap<Point, u64>,
) -> u64 {
    let new_loc = Point::new(current_point.x, current_point.y + 1);
    if memory.contains_key(&new_loc) {
        // we have been down this path before. Just return this value from here
        return *memory.get(&new_loc).unwrap();
    }
    match Grid::get_value(grid, &new_loc) {
        Some(Cell::Empty) => find_timelines(grid, &new_loc, memory),
        Some(Cell::Beam) | Some(Cell::Start) => {
            panic!("unexpected position")
        }
        Some(Cell::Splitter) => {
            let mut timelines = 0;
            if current_point.x < grid.xmax {
                let new_point = Point::new(current_point.x + 1, current_point.y + 1);
                timelines += find_timelines(grid, &new_point, memory)
            }
            if current_point.x > 0 {
                let new_point = Point::new(current_point.x - 1, current_point.y + 1);
                timelines += find_timelines(grid, &new_point, memory)
            }
            memory.insert(new_loc, timelines);
            timelines
        }
        None => {
            // End of y postions, this is a single timeline
            1
        }
    }
}

fn main() {
    let input = get_input("day7-input.txt");
    let (branches, timelines) = solve(&input);
    println!(
        "timeline has {} branches resulting in {} timelines",
        branches, timelines
    );
}
