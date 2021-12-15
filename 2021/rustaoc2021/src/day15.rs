use crate::grid::{Grid, Point};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::str::FromStr;

pub fn part_1(input: &str) -> usize {
    let grid = parse_to_grid(input);
    println!("search for distance from 0,0 to {} {}", grid.xmax, grid.ymax);
    let start = Point::new(0, 0);
    let end = Point::new(grid.xmax, grid.ymax);

    // perform search through grid

    // keep track of visited nodes
    // keep track of "distances" to each node
    // initialize this map with maximum values for the integer used
    let mut distances: HashMap<Point, u64> = HashMap::new();

    for point in grid.get_map().keys().copied() {
        distances.insert(point, u64::MAX);
    }

    let mut visited_points: HashSet<Point> = HashSet::new();
    // initialize for start
    let start_value = grid.get_map().get(&start).unwrap();
    distances.insert(start, 0);

    // now loop until condition is satisfied
    let mut current_node = start;
    let mut current_node_distance = 0;

    loop {
        //println!("current node is: {}, current distance: {}", current_node, current_node_distance);

        grid
            .get_neighbour_key_value(&current_node)
            .iter()
            .filter(|(neighbour, _weight)| !visited_points.contains(neighbour))
            .for_each(|(neighbour, weight)| {
                let mut current_neighour_distance = distances.get_mut(neighbour).unwrap();
                if weight + current_node_distance < *current_neighour_distance {
                    *current_neighour_distance = weight + current_node_distance;
                }
            });

        visited_points.insert(current_node);

        let mut scores : Vec<(Point, u64)> = distances.iter().filter(|(point, value)| !visited_points.contains(point)).map( | (key, value)| (*key, *value)).collect();
        scores.sort_by(|(a,b), (c,d)| b.cmp(d));

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

#[cfg(test)]
mod tests {
    use crate::day15::part_1;

    #[test]
    fn test_part_1_example() {
        let input = include_str!(r"../resources/inputs/day15-example.txt");
        assert_eq!(40, part_1(input));
    }

    #[test]
    fn test_part1_input() {
        let input2 = include_str!(r"../resources/inputs/day15-input.txt");
        assert_eq!(0, part_1(input2));
    }
}
