use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ParseError;

#[derive(Debug, PartialEq)]
pub struct PositionSpec {
    pos_a : (i64, i64),
    pos_b : (i64, i64),
}

impl FromStr for PositionSpec {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Vec<&str> = s.split(" -> ").collect();
        let pos_a = PositionSpec::get_pos(&result[0]);
        let pos_b = PositionSpec::get_pos(&result[1]);
        Ok(Self { pos_a, pos_b})
    }
}

impl PositionSpec {
    fn get_pos(result: &str) -> (i64, i64) {
        let mut pos_a = result.split(",");
        let pos_a_x: i64 = pos_a.next().unwrap().parse().unwrap();
        let pos_a_y: i64 = pos_a.next().unwrap().parse().unwrap();
        (pos_a_x, pos_a_y)
    }
}

pub fn part_1(input : &Vec<PositionSpec>) -> usize {
    let visisted_postions = get_visited_locations(input, false);
    visisted_postions.iter().filter( | (_key, value)| **value >= 2 as u64).count()
}

pub fn part_2(input : &Vec<PositionSpec>) -> usize {
    let visisted_postions = get_visited_locations(input, true);
    visisted_postions.iter().filter( | (_key, value)| **value >= 2 as u64).count()
}

fn get_visited_locations(input: &Vec<PositionSpec>, allow_diagonal : bool) -> HashMap<(i64, i64), u64> {
    let mut visisted_postions: HashMap<(i64, i64), u64> = HashMap::new();

    for spec in input {
        let (range, x_step, y_step) = get_stepsize_and_range(spec);

        if allow_diagonal || x_step == 0 || y_step == 0 {
            let (mut x, mut y) = spec.pos_a;
            let count = visisted_postions.entry((x, y)).or_insert(0);
            *count += 1;
            for _ in 0..range.abs() {
                x += x_step;
                y += y_step;
                let count = visisted_postions.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
    }
    visisted_postions
}

fn get_stepsize_and_range(spec: &PositionSpec) -> (i64, i64, i64) {
    let (x1, y1) = spec.pos_a;
    let (x2, y2) = spec.pos_b;

    let dx = x2 - x1;
    let dy = y2 - y1;
    let range = {
        if dx.abs() > dy.abs() {
            dx
        } else {
            dy
        }
    };
    let x_step = { if dx != 0 { dx / dx.abs() } else { 0 } };
    let y_step = { if dy != 0 { dy / dy.abs() } else { 0 } };
    (range, x_step, y_step)
}


#[cfg(test)]
mod tests {
    use crate::day5::{part_1, part_2};
    use crate::day5::PositionSpec;
    use crate::reader::{parse_lines_to_vec};

    #[test]
    fn test_example() {
        let input : Vec<PositionSpec>= parse_lines_to_vec("./resources/inputs/day5-example.txt").unwrap();

        assert_eq!(5, part_1(&input));
        assert_eq!(12, part_2(&input));
    }

    #[test]
    fn test_input() {
        let input : Vec<PositionSpec>= parse_lines_to_vec("./resources/inputs/day5-input.txt").unwrap();
        assert_eq!(6225, part_1(&input));
        assert_eq!(22116, part_2(&input));

    }

}