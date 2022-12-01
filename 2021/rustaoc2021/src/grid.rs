use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub(crate) fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x={},y={}", self.x, self.y)
    }
}

// fn get_distance(p1 : &Point, p2: &Point) -> isize {
//     let x_dist = (p1.x - p2.x).abs();
//     let y_dist = (p1.y - p2.y).abs();
//     x_dist + y_dist
// }

pub struct Grid {
    // hashmap backed grid class where top left is ymax, xmin and bottom right is ymin, xmax
    // this type is useful for sparse grids, dense grids are more efficiently approached using
    // a vector based grid
    map: HashMap<Point, u64>,
    pub xmax: isize,
    pub ymax: isize,
    ymin: isize,
    xmin: isize,
}



impl Grid {
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
            xmax: 0,
            ymax: 0,
            ymin: 0,
            xmin: 0,
        }
    }

    pub fn remove_loc(&mut self, loc: &Point) {
        self.map.remove(loc);
    }

    pub fn increment_loc(&mut self, loc : &Point, increment : u64) {
        let value = self.map.get_mut(loc).unwrap();
        *value += increment
    }

    pub fn update_loc(&mut self, loc : Point, value : u64) {
        self.map.insert(loc, value);
    }

    pub fn get_locations(&self) -> Vec<Point> {
        self.map.keys().map(|x| x.clone()).collect::<Vec<Point>>()
    }

    pub fn get_map(&self) -> &HashMap<Point, u64>{
        &self.map
    }
    pub fn print_grid(&mut self) {
        let mut digit_header = String::from("       ");
        let mut tenths_header = String::from("       ");
        let mut sign_header = String::from("x:     ");

        let mut header = String::from("y:   | ");
        for x in self.xmin..self.xmax+1 {

            header.push('-');

            let digit: Vec<_>= format!("{:+03}", x%10).chars().collect();

            digit_header.push(digit[2]);
            tenths_header.push(digit[1]);
            sign_header.push(digit[0]);
        }
        header.push_str(" |");
        digit_header.push_str("  ");
        tenths_header.push_str("  ");
        sign_header.push_str("  ");

        println!("{}", sign_header);
        println!("{}", tenths_header);
        println!("{}", digit_header);

        println!("{}", header);

        for y in (self.ymin..self.ymax + 1).rev() {

            let mut line = String::new();
            let line_nr = format!("{:+03}  | ", y);
            line.push_str(line_nr.as_str());

            for x in self.xmin..self.xmax + 1 {
                let symbol = self.map.get(&Point { x, y });
                if let Some(char) = symbol {
                    line = line.add(&*format!("{}", char));
                } else {
                    line = line.add("#");
                }
            }
            line = line.add(" |");
            println!("{}", line);
        }
        println!("{}", header);
    }

    pub fn add_to_grid(&mut self, point: Point, value: u64) {
        if point.y < self.ymin {
            self.ymin = point.y;
        }
        if point.y > self.ymax {
            self.ymax = point.y
        }
        if point.x < self.xmin {
            self.xmin = point.x
        }
        if point.x > self.xmax {
            self.xmax = point.x
        }
        self.map.insert(point, value);
    }

    pub fn get_neighbours(&self, point: &Point) -> Vec<Point> {
        let offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        offsets
            .iter()
            .map(|(x_off, y_off)| Point::new(point.x + x_off, point.y + y_off))
            .filter(|neighbour| self.map.contains_key(neighbour))
            .collect()
    }

    pub fn get_neighbour_key_value(&self, point: &Point) -> Vec<(Point, u64)> {
        let offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        offsets
            .iter()
            .map(|(x_off, y_off)| Point::new(point.x + x_off, point.y + y_off))
            .filter(|neighbour| self.map.contains_key(neighbour))
            .map(|neighbour| (neighbour, *self.map.get(&neighbour).unwrap()))
            .collect()
    }

    pub fn get_neighbours_diag(&self, point: &Point) -> Vec<Point> {
        let offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1,1), (-1, -1), (1,-1), (-1,1)];

        offsets
            .iter()
            .map(|(x_off, y_off)| Point::new(point.x + x_off, point.y + y_off))
            .filter(|neighbour| self.map.contains_key(neighbour))
            .collect()
    }
}
//
// #[cfg(test)]
// mod tests {
//     use crate::{get_distance, Grid, Point};

    // #[test]
    // fn test_print() {
    //     let mut grid = Grid::new();
    //     grid.add_to_grid(Point::new(0, 1), '*'.to_string());
    //     grid.add_to_grid(Point::new(2, -3), 'v'.to_string());
    //     grid.add_to_grid(Point::new(-2, 5), '*'.to_string());
    //
    //     grid.print_grid()
    // }
    //
    // #[test]
    // fn test_neighbour() {
    //     let mut grid = Grid::new();
    //     grid.add_to_grid(Point::new(0, 1), '*'.to_string());
    //     grid.add_to_grid(Point::new(0, 2), 'v'.to_string());
    //     assert_eq!(
    //         vec![Point::new(0, 2)],
    //         grid.get_neighbours(Point::new(0, 1))
    //     );
    // }

    // #[test]
    // fn test_distance() {
    //     let p1 = Point::new(0,0);
    //     let p2 = Point::new(0,2);
    //     assert_eq!(2, get_distance(&p1, &p2));
    //
    //     let p1 = Point::new(0,-1);
    //     let p2 = Point::new(-2,2);
    //     assert_eq!(5, get_distance(&p1, &p2));
    // }
//}