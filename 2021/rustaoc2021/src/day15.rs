use crate::grid::{Grid, Point};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::str::FromStr;

pub fn part_1(input: &str) -> usize {
    let mut grid = parse_to_grid(input);
    println!("search for distance from 0,0 to {} {}", grid.xmax, grid.ymax);
    let start = Point::new(0, 0);
    let end = Point::new(grid.xmax, grid.ymax);

    run_dijkstra(&mut grid, start, end)
}

pub fn part_2(input: &str) -> usize {
    let mut grid = parse_to_grid_x5(input);
    println!("search for distance from 0,0 to {} {}", grid.xmax, grid.ymax);
    let start = Point::new(0, 0);
    let end = Point::new(grid.xmax, grid.ymax);

    run_dijkstra(&mut grid, start, end)
}

fn run_dijkstra(grid: &mut Grid, start: Point, end: Point) -> usize {
// initialize everything at max distance
    let mut distances: HashMap<Point, u64> = HashMap::new();
    // for point in grid.get_map().keys().copied() {
    //     distances.insert(point, u64::MAX);
    // }
    //grid.print_grid();

    // initialize for start
    let mut visited_points: HashSet<Point> = HashSet::new();
    distances.insert(start, 0);

    // now loop until condition is satisfied
    let mut current_node = start;
    let mut current_node_distance = 0;

    loop {
        //println!("current node is: {}, current distance: {}", current_node, current_node_distance);
        grid
            .get_neighbour_key_value(&current_node)
            .iter()
            .for_each(|(neighbour, weight)| {
                let mut current_neighour_distance = distances.entry(*neighbour).or_insert(u64::MAX);
                if weight + current_node_distance < *current_neighour_distance {
                    *current_neighour_distance = weight + current_node_distance;
                }
            });

        visited_points.insert(current_node);
        distances.remove(&current_node);
        grid.remove_loc(&current_node);

        let mut scores: Vec<(Point, u64)> = distances.iter().map(|(key, value)| (*key, *value)).collect();
        scores.sort_by(|(a, b), (c, d)| b.cmp(d));

        let (node, distance, ) = scores[0];
        current_node = node;
        current_node_distance = distance;

        if end == current_node {
            println!("Stopping because our next node is the end node, with distance {}", distances.get(&end).unwrap());
            return *distances.get(&end).unwrap() as usize
        }
    }
}

fn parse_to_grid(input: &str) -> Grid {
    let mut grid = Grid::new();
    for (y, line) in input.split("\n").enumerate() {
        //println!("{:?}", line.clone().trim().chars().collect::<Vec<char>>());
        for (x, nr) in line
            .trim()
            .chars()
            .map(|s| u64::from_str(&*s.to_string()).unwrap())
            .enumerate()
        {
            grid.add_to_grid(Point::new(x as isize, y as isize), nr);
        }
    }
    grid
}

fn parse_to_grid_x5(input: &str) -> Grid {
    let mut grid = Grid::new();
    for (y, line) in input.split("\n").enumerate() {
        //println!("{:?}", line.clone().trim().chars().collect::<Vec<char>>());
        for (x, nr) in line
            .trim()
            .chars()
            .map(|s| u64::from_str(&*s.to_string()).unwrap())
            .enumerate()
        {
            grid.add_to_grid(Point::new(x as isize, y as isize), nr);
            let grid_size = line.len()-1;
            for n_x in 0..5 {
                for n_y in 0..5 {
                    if n_x != 0 || n_y != 0 {
                        let new_x = n_x*grid_size+x;
                        let new_y = n_y*grid_size+y;
                        let new_nr = 1+ ((nr-1+n_x as u64 +n_y as u64) % 9);
                        grid.add_to_grid(Point::new(new_x as isize, new_y as isize ), new_nr);
                    }

                }
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use crate::day15::{part_1, part_2};

    #[test]
    fn test_part_1_example() {
        let input = include_str!(r"../resources/inputs/day15-example.txt");
        assert_eq!(40, part_1(input));
    }

    #[test]
    fn test_part_2_example() {
        let input = include_str!(r"../resources/inputs/day15-example.txt");
        assert_eq!(315, part_2(input));
    }

    #[test]
    fn test_part1_input() {
        let input2 = include_str!(r"../resources/inputs/day15-input.txt");
        assert_eq!(398, part_1(input2));
    }

    #[test]
    fn test_part2_input() {
        let input2 = include_str!(r"../resources/inputs/day15-input.txt");
        assert_eq!(398, part_2(input2));
    }
}
