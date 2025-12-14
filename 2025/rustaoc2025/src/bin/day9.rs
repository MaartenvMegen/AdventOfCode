use crate::Tile::{Green, Red, Unsupported};
use rustaoc2025::grid::{Grid, Point, DIRS8};
use rustaoc2025::{get_input, run_timed};
use std::cmp::PartialEq;
use std::collections::{HashSet, VecDeque};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Red,
    Green,
    Unsupported,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Red => 'r',
            Green => 'g',
            Tile::Unsupported => 'u',
        };
        write!(f, "{ch}")
    }
}

fn get_area(corner1: Point, corner2: Point) -> isize {
    (1 + (corner1.x - corner2.x).abs()) * (1 + (corner1.y - corner2.y).abs())
}

fn get_sorted_rectangle_size(corners: Vec<Point>) -> Vec<(isize, Point, Point)> {
    let n = corners.len();

    // Build all unique pairs (i < j) with distances
    let mut rectangles: Vec<(isize, Point, Point)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let area = get_area(corners[i], corners[j]);
            rectangles.push((area, corners[i], corners[j]));
        }
    }

    // Sort edges by distance
    rectangles.sort_by(|(d1, _, _), (d2, _, _)| d1.cmp(d2));
    rectangles.reverse();
    rectangles
}

fn get_corners_from_input(input: &str) -> Vec<Point> {
    let corners: Vec<Point> = input
        .trim()
        .split("\n")
        .map(|line| Point::parse(line).unwrap())
        .collect();
    corners
}

fn is_valid_area(grid: &Grid<Tile>, p1: Point, p2: Point) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    // Bottom + top edges
    for x in min_x..=max_x {
        // bottom
        if grid.get_value(&Point { x, y: min_y }) == Some(&Unsupported) {
            return false;
        }
        // top (avoid duplicate when height == 0)
        if max_y != min_y && grid.get_value(&Point { x, y: max_y }) == Some(&Unsupported) {
            return false;
        }
    }

    // Left + right edges (inner y only, so we don't double-check corners)
    if max_x != min_x {
        for y in (min_y + 1)..max_y {
            if grid.get_value(&Point { x: min_x, y }) == Some(&Unsupported) {
                return false;
            }
            if grid.get_value(&Point { x: max_x, y }) == Some(&Unsupported) {
                return false;
            }
        }
    }

    true
}

fn mark_boundary_segments(grid: &mut Grid<Tile>, reds: &[Point]) {
    if reds.is_empty() {
        return;
    }

    // All consecutive pairs, including last->first (wrap-around)
    let n = reds.len();
    for i in 0..n {
        let a = reds[i];
        let b = reds[(i + 1) % n];

        if a.x == b.x {
            // Vertical segment
            let x = a.x;
            let y_min = a.y.min(b.y);
            let y_max = a.y.max(b.y);
            for y in y_min..=y_max {
                make_green_if_not_red(grid, x, y);
            }
        } else if a.y == b.y {
            // Horizontal segment
            let y = a.y;
            let x_min = a.x.min(b.x);
            let x_max = a.x.max(b.x);
            for x in x_min..=x_max {
                make_green_if_not_red(grid, x, y);
            }
        } else {
            // According to the puzzle, this shouldn't happen
            panic!("Non-axis-aligned segment between {:?} and {:?}", a, b);
        }
    }
}

fn make_green_if_not_red(grid: &mut Grid<Tile>, x: isize, y: isize) {
    let point = Point::new(x, y);
    if let Some(_neighbor) = grid.get_value(&point) {
        // already filled
    } else {
        grid.add_to_grid(point, Green);
    }
}

fn flood_outside_band(grid: &mut Grid<Tile>) {
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    // Start from a guaranteed-outside corner
    let lower_corner = grid
        .get_locations()
        .iter()
        .copied()
        .min_by_key(|p| (p.x, p.y))
        .unwrap();
    let start = Point {
        x: lower_corner.x - 1,
        y: lower_corner.y,
    };
    visited.insert(start);
    q.push_back(start);

    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some(p) = q.pop_front() {
        for (dx, dy) in dirs {
            let np = Point {
                x: p.x + dx,
                y: p.y + dy,
            };

            if visited.contains(&np) {
                continue;
            }

            // Can't flood through walls (red/green)
            if is_wall(grid, &np) {
                continue;
            }

            // Only care about cells that “touch” the loop/boundary:
            if !has_wall_neighbor(grid, &np) {
                continue;
            }

            visited.insert(np);
            grid.add_to_grid(np, Unsupported);
            q.push_back(np);
        }
    }
}

fn is_wall(grid: &Grid<Tile>, p: &Point) -> bool {
    matches!(grid.get_value(p), Some(Red) | Some(Green))
}

fn has_wall_neighbor(grid: &Grid<Tile>, p: &Point) -> bool {
    DIRS8.iter().any(|(dx, dy)| {
        let np = Point {
            x: p.x + dx,
            y: p.y + dy,
        };
        is_wall(grid, &np)
    })
}

fn solve(input: &str) -> usize {
    let corners = get_corners_from_input(input);
    let rectangles = get_sorted_rectangle_size(corners);
    rectangles[0].0 as usize
}

fn solve2(input: &str) -> usize {
    let mut grid: Grid<Tile> = Grid::new();

    println!("retrieving corners");
    // get the corners and fill grid with them
    let corners = get_corners_from_input(input);
    corners
        .iter()
        .for_each(|corner| grid.add_to_grid(*corner, Red));

    print!("marking boundery segments");
    // fill other points in between with green tiles
    mark_boundary_segments(&mut grid, &corners);

    print!("flooding");
    // now flood fill around the shape
    flood_outside_band(&mut grid);

    println!("sorting");
    let rectangles = get_sorted_rectangle_size(corners);

    println!("validating");
    // Find first that is valid
    let (size, _p1, _p2) = rectangles
        .iter()
        .find(|(_area, p1, p2)| is_valid_area(&grid, *p1, *p2))
        .unwrap();
    // report its size
    *size as usize
}

fn main() {
    let input = get_input("day9-input.txt");
    println!("{}", solve(&input));
    run_timed(solve2, &input, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        assert_eq!(35, get_area(Point { x: 7, y: 1 }, Point { x: 11, y: 7 }));
    }
}
