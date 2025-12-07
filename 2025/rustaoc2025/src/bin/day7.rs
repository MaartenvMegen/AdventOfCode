use rustaoc2025::get_input;
use rustaoc2025::grid::{Grid, Point};
use std::collections::{HashMap, HashSet};
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

fn solve(input: &str) -> usize {
    let mut grid: Grid<Cell> = Grid::parse_to_grid(input, |c| Cell::from_char(c).unwrap());
    let (source, _cell) = grid
        .get_map()
        .iter()
        .find(|(_location, object)| **object == Cell::Start)
        .unwrap();

    let mut beamfront: HashSet<Point> = HashSet::new();
    beamfront.insert(*source);

    let mut splits: u64 = 0;

    // al beams have point y at grid max
    while !beamfront.is_empty() {
        // check down loc, if empty move if splitter create 2 new beams
        let search_list = beamfront.clone();
        beamfront.clear();

        search_list.iter().for_each(|point| {
            let new_loc = Point::new(point.x, point.y + 1);
            match Grid::get_value(&grid, &new_loc) {
                Some(Cell::Empty) => {
                    beamfront.insert(new_loc);
                    grid.update_loc(new_loc, Cell::Beam)
                }
                Some(Cell::Beam) => {
                    // nothing to do, we have been here already
                }
                Some(Cell::Start) => {
                    // how did we end up at the start?
                }
                Some(Cell::Splitter) => {
                    splits += 1;
                    if point.x < grid.xmax {
                        let new_point = Point::new(point.x + 1, point.y + 1);
                        beamfront.insert(new_point);
                        grid.update_loc(new_point, Cell::Beam)
                    }
                    if point.x > 0 {
                        let new_point = Point::new(point.x - 1, point.y + 1);
                        beamfront.insert(new_point);
                        grid.update_loc(new_point, Cell::Beam)
                    }
                }
                None => {
                    // nothing to do end of y postions
                }
            }
        })
    }

    grid.print_grid();
    splits as usize
}

fn solve2(input: &str) -> u64 {
    let grid: Grid<Cell> = Grid::parse_to_grid(input, |c| Cell::from_char(c).unwrap());
    let (source, _cell) = grid
        .get_map()
        .iter()
        .find(|(_location, object)| **object == Cell::Start)
        .unwrap();
    let mut memory: HashMap<Point, u64> = HashMap::new();
    find_timelines(&grid, source, &mut memory)
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
            println!("timeline at point: {}", current_point);
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
    println!("{}", solve(&input));
    println!("{}", solve2(&input));
}
