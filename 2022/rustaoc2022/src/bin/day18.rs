use rustaoc2022::grid::{Grid, Location};
use std::collections::HashSet;

const EXAMPLE: &str = include_str!(r"../../resources/day18-example.txt");
const INPUT: &str = include_str!(r"../../resources/day18-input.txt");

const NEIGHBOURS: [(i64, i64, i64); 6] = [(1, 0, 0), (0, 1, 0), (0, 0, 1) , (-1, 0, 0), (0, -1, 0), (0, 0, -1)];

type Point3d = (i64,i64,i64);

struct Grid3d {
    contents : HashSet<Point3d>
}

impl Grid3d {
    pub(crate) fn get_faces(&self) -> i64 {
        let mut accumulator = 0;
        for (x,y,z) in &self.contents {
            for (xd,yd,zd) in NEIGHBOURS {
                let newloc : Point3d = (x+xd,y+yd,z+zd);
                if !self.contents.contains(&newloc) {
                    accumulator += 1
                }
            }
        }
        accumulator
    }
}

fn part1(input: &str) -> i64 {
    let mut grid = Grid3d { contents: Default::default() };
    for line in input.trim().lines() {
        let point = line.split(',').map(|char| char.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let point : Point3d = (point[0], point[1], point[2]);
        grid.contents.insert(point);
    }
    grid.get_faces()
}

fn part2(input: &str) -> i64 {
    let mut grid = Grid3d { contents: Default::default() };
    for line in input.trim().lines() {
        let point = line.split(',').map(|char| char.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let point : Point3d = (point[0], point[1], point[2]);
        grid.contents.insert(point);
    }

    // invert the grid in a safe array size
    let mut inverted_grid = Grid3d { contents: Default::default() };
    for x in -1..30 {
        for y in -1..30 {
            for z in -1..30 {
                let point = (x,y,z);
                if !grid.contents.contains( &point) {
                    inverted_grid.contents.insert(point);
                }
            }
        }
    }

    println!("starting BFS using grid of size : {} and inverted grid of size {}", grid.contents.len(), inverted_grid.contents.len());

    // assemble all inverted points from the outside
    // collect faces along the way
    let mut search_edge: HashSet<Point3d> = HashSet::new();
    let mut scanned_locs = HashSet::new();
    let mut accumulator = 0;

    search_edge.insert((0,0,0));
    'searchloop : loop {
        let mut new_edge: HashSet<Point3d> = HashSet::new();
        for (x,y,z) in &search_edge {
            //println!("now scanning x={},y={},z={}", x,y,z);
            for (xd,yd,zd) in NEIGHBOURS {
                let newloc = (x+xd, y+yd, z+zd);
                if inverted_grid.contents.contains(&newloc) && !scanned_locs.contains(&newloc) {
                    new_edge.insert(newloc);
                }
                if grid.contents.contains(&newloc)  {
                    //println!("found a face");
                    accumulator += 1;
                }
            }
        }
        for loc in search_edge {
            scanned_locs.insert(loc);
        }
        search_edge = new_edge;
        if search_edge.is_empty() {
            break 'searchloop;
        }
    }

    // for some reason the accumulator failed to detect every outside edge.
    // as a workaround just collect the nodes that were not found in the outside search
    // these are the air pockets. just get their faces.
    // it missed 5 faces. so this could be a single node that it failed on
    // since air pocket can be properly generated from the BFS scan, all outside nodes are properly scanned
    // then how does it still manage to miss a few faces?
    // option a: it fails to scan all directions properly from each outside point?
    // option b: the range is too limited -> THIS WAS IT! 0..30 needed to be -1..30
    // lesson learned. do not hardcode stuff.
    let mut bubble = Grid3d { contents: Default::default() };
    for loc in inverted_grid.contents {
        if !scanned_locs.contains(&loc) {
            bubble.contents.insert(loc);
        }
    }

    println!("created a bubble with {} blocks and {} faces", bubble.contents.len(), bubble.get_faces());
    println!("accumulator thinks {} while droplet-airpocket = {}", accumulator, grid.get_faces()-bubble.get_faces());
    println!("now scanned {} locs", scanned_locs.len());
    grid.get_faces() - bubble.get_faces()
}

fn main() {
    rustaoc2022::run_matrix(part1, part2, EXAMPLE, INPUT);
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, EXAMPLE, INPUT, Grid3d};

    #[test]
    fn test_example() {
        assert_eq!(64, part1(EXAMPLE));
        assert_eq!(58, part2(EXAMPLE));
    }

    #[test]
    fn test_input() {
        assert_eq!(4370, part1(INPUT));
        assert_eq!(2458, part2(INPUT));
    }

    #[test]
    fn test_example_small() {
        let mut grid = Grid3d { contents: Default::default() };
        grid.contents.insert( (1,1,1));
        grid.contents.insert((2,1,1));
        assert_eq!(10, grid.get_faces())
    }

}
