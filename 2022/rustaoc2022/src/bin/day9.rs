use std::collections::HashSet;
use std::str::FromStr;

const EXAMPLE: &str = include_str!(r"../../resources/day9-example.txt");
const INPUT: &str = include_str!(r"../../resources/day9-input.txt");

type Position = (i32, i32);

struct Knot {
    position: Position,
}

impl Knot {
    fn is_adjacent(&self, other_knot: &Knot) -> bool {
        let (this_x, this_y) = self.position;
        let (other_x, other_y) = other_knot.position;
        this_y - other_y < 2
            && this_y > other_y - 2
            && this_x - other_x < 2
            && this_x - other_x > -2
    }

    fn get_pulled_position(&self, other_knot: &Knot) -> Position {
        let (this_x, this_y) = self.position;
        let (other_x, other_y) = other_knot.position;

        if self.is_adjacent(other_knot) {
            (this_x, this_y)
        } else {
            let move_x = Self::get_move_delta(this_x, other_x);
            let move_y = Self::get_move_delta(this_y, other_y);
            (this_x + move_x, this_y + move_y)
        }
    }

    fn get_move_delta(this: i32, other: i32) -> i32 {
        let delta = this - other;
        match delta {
            0 => 0,
            x if x > 0 => -1,
            x if x < 0 => 1,
            _ => panic!("something weird happened. delta_x = {}", delta),
        }
    }
}

fn part1(input: &str) -> usize {
    move_the_rope(input, 2)
}

fn part2(input: &str) -> usize {
    move_the_rope(input, 10)
}

fn move_the_rope(input: &str, nr_knots: u32) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut knots: Vec<Knot> = (0..nr_knots).map(|_| Knot { position: (0, 0) }).collect();

    input.trim().split('\n').for_each(|instruction| {
        let (direction, amount) = instruction.split_once(' ').unwrap();
        let amount = u64::from_str(amount).unwrap();
        let (delta_x, delta_y) = match direction {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => panic!("unexpected direction"),
        };
        for _ in 0..amount {
            // only the head is moved directly, others follow
            let (current_x, current_y) = knots[0].position;
            knots[0].position = (delta_x + current_x, delta_y + current_y);

            // move the others
            for index in 0..knots.len() - 1 {
                let mut knot_slice = &mut knots[index..=index + 1];
                // 0 = head, 1= tail
                let tail_pos = knot_slice[1].get_pulled_position(&knot_slice[0]);
                knot_slice[1].position = tail_pos;
            }
            visited.insert(knots[knots.len() - 1].position);
        }
    });

    visited.len()
}

fn main() {
    rustaoc2022::run_matrix(part1, part2, EXAMPLE, INPUT);
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, Knot, EXAMPLE, INPUT};

    #[test]
    fn test_example() {
        assert_eq!(13, part1(EXAMPLE));
        assert_eq!(1, part2(EXAMPLE));
    }

    #[test]
    fn test_input() {
        assert_eq!(6464, part1(INPUT));
        assert_eq!(2604, part2(INPUT));
    }

    #[test]
    fn get_pulled_pos() {
        let knot_a = Knot { position: (0, 0) };
        let knot_b = Knot { position: (0, 0) };
        assert_eq!((0, 0), knot_a.get_pulled_position(&knot_b));

        let knot_a = Knot { position: (0, 0) };
        let knot_b = Knot { position: (0, 1) };
        assert_eq!((0, 0), knot_a.get_pulled_position(&knot_b));

        let knot_a = Knot { position: (0, 0) };
        let knot_b = Knot { position: (0, 2) };
        assert_eq!((0, 1), knot_a.get_pulled_position(&knot_b));

        let knot_a = Knot { position: (0, 0) };
        let knot_b = Knot { position: (1, 2) };
        assert_eq!((1, 1), knot_a.get_pulled_position(&knot_b));
    }

    #[test]
    fn test_adjacency() {
        let knot_a = Knot { position: (0, 0) };
        let knot_b = Knot { position: (0, 0) };
        assert!(knot_a.is_adjacent(&knot_b));

        let knot_a = Knot { position: (0, 1) };
        let knot_b = Knot { position: (0, 0) };
        assert!(knot_a.is_adjacent(&knot_b));

        let knot_a = Knot { position: (1, 0) };
        let knot_b = Knot { position: (0, 0) };
        assert!(knot_a.is_adjacent(&knot_b));

        let knot_a = Knot { position: (1, 1) };
        let knot_b = Knot { position: (0, 0) };
        assert!(knot_a.is_adjacent(&knot_b));

        let knot_a = Knot { position: (-1, 0) };
        let knot_b = Knot { position: (0, 0) };
        assert!(knot_a.is_adjacent(&knot_b));

        let knot_a = Knot { position: (-1, -1) };
        let knot_b = Knot { position: (0, 0) };
        assert!(knot_a.is_adjacent(&knot_b));

        let knot_a = Knot { position: (0, -1) };
        let knot_b = Knot { position: (0, 0) };
        assert!(knot_a.is_adjacent(&knot_b));

        let knot_a = Knot { position: (0, -2) };
        let knot_b = Knot { position: (0, 0) };
        assert!(!knot_a.is_adjacent(&knot_b));
    }
}
