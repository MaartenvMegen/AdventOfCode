

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufReader, Lines};
    use std::str::FromStr;
    use crate::reader::get_lines;
    use crate::grid::{Grid, Point};

    #[test]
    fn test_part1() {
        let input = get_lines("./resources/inputs/day9-input.txt");
        let grid = parse_to_grid(input);
        //grid.print_grid();
        let low_values = get_low_values(grid);
        let answer: u64 = low_values.iter().sum::<u64>() + low_values.len() as u64;
        assert_eq!(607, answer)
    }

    #[test]
    fn test_part2_example() {
        let input = get_lines("./resources/inputs/day9-example.txt");
        let grid = parse_to_grid(input);
        //grid.print_grid();
        let basins = get_basins(grid);
        let answer = get_answer(basins);

        assert_eq!(1134, answer)
    }

    #[test]
    fn test_part2_input() {
        let input = get_lines("./resources/inputs/day9-input.txt");
        let grid = parse_to_grid(input);
        //grid.print_grid();
        let basins = get_basins(grid);
        let answer = get_answer(basins);

        assert_eq!(900864, answer)
    }

    fn get_answer(basins: Vec<Vec<Point>>) -> usize {
        let mut basin_sizes: Vec<usize> = basins.iter().map(|basin| basin.len()).collect();
        basin_sizes.sort();
        basin_sizes.reverse();
        //println!("{:?}", basin_sizes);
        let answer: usize = basin_sizes.iter().take(3).product();

        answer
    }

    fn get_low_values(grid: Grid) -> Vec<u64> {
        let mut low_values: Vec<u64> = Vec::new();
        for (position, value) in grid.get_map() {
            let neighbours = grid.get_neighbours(position);
            let amount_low = neighbours.iter().map(|position| grid.get_map().get(position).unwrap()).filter(|n_value| n_value <= &value).count();
            if amount_low == 0 {
                low_values.push(*value)
            }
        }
        //println!("{:?}", low_values);
        low_values
    }

    fn get_low_positions(grid: &Grid) -> Vec<Point> {
        let mut low_points: Vec<Point> = Vec::new();
        for (position, value) in grid.get_map() {
            let neighbours = grid.get_neighbours(position);
            let amount_low = neighbours.iter().map(|position| grid.get_map().get(position).unwrap()).filter(|n_value| n_value <= &value).count();
            if amount_low == 0 {
                low_points.push(position.clone())
            }
        }
        //println!("{:?}", low_points);
        low_points
    }

    fn get_basins(grid: Grid) -> Vec<Vec<Point>> {
        // every low point is the start of the basin
        // from there add neighbours untill you reach a 9.
        let mut basins = Vec::new();
        let positions = get_low_positions(&grid);

        // while discovered neighbours is not 9 or known pos search deeper
        for low_point in positions {
            let mut basin = Vec::new();
            let mut search_positions = vec![low_point.clone()];
            loop {
                let mut new_sites = HashSet::new();
                for position in &search_positions {
                    let neighbours = grid.get_neighbours(&position);
                    for neighbour in neighbours {
                        if grid.get_map().get(&neighbour).unwrap() != &9 && !basin.contains(&neighbour) {
                            new_sites.insert(neighbour);
                        }
                    }
                }
                basin.append(&mut search_positions);
                // end of basin reached
                if new_sites.is_empty() {
                    basins.push(basin);
                    break
                // everything discovered is new front
                } else {
                    search_positions = new_sites.iter().map(|x| x.clone()).collect();


                }
            }

        }


        basins
    }

    fn parse_to_grid(input: Lines<BufReader<File>>) -> Grid {
        let mut grid = Grid::new();
        for (y, line) in input.enumerate() {
            let line = line.unwrap();
            //println!("{:?}", line.clone().trim().chars().collect::<Vec<char>>());
            for (x, nr) in line.trim().chars().map(|s| u64::from_str(&*s.to_string()).unwrap()).enumerate() {
                grid.add_to_grid(Point::new(x as isize, y as isize), nr);
            }
        }
        grid
    }
}