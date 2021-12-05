use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Debug)]
struct Board {
    id: u64,
    rows: Vec<HashSet<u64>>,
    columns: Vec<HashSet<u64>>,
}

impl Board {
    fn new(id: u64) -> Self {
        Self {
            id: id,
            rows: Vec::new(),
            columns: Vec::new(),
        }
    }

    fn add_row(&mut self, row: Vec<u64>) {
        let mut new_row: HashSet<u64> = HashSet::new();

        for (index, value) in row.iter().enumerate() {
            new_row.insert(*value);
            if let Some(column) = self.columns.get_mut(index) {
                column.insert(*value);
            } else {
                let mut new_column: HashSet<u64> = HashSet::new();
                new_column.insert(*value);
                self.columns.push(new_column);
            }
        }
        self.rows.push(new_row);
    }

    fn check_nr(&mut self, number: &u64) -> bool {
        for row in &mut self.rows {
            row.remove(number);
            if row.is_empty() {
                return true;
            }
        }
        for column in &mut self.columns {
            column.remove(number);
            if column.is_empty() {
                return true;
            }
        }

        false
    }

    fn get_remaining_values(&self) -> u64 {
        let mut sum: u64 = 0;
        for row in &self.rows {
            for value in row {
                sum += value
            }
        }
        sum
    }
}

pub fn get_first_and_last_board_score(input: Lines<BufReader<File>>) -> (u64, u64) {
    let (mut boards, nrs) = parse_input(input);

    let mut winning_boards: HashSet<u64> = HashSet::new();
    let mut answer = 0;

    let mut first = true;
    let mut part_1 = 0;

    for nr in nrs {
        for board in &mut boards {
            let winning = board.check_nr(&nr);
            if winning {
                if !winning_boards.contains(&board.id) {
                    winning_boards.insert(board.id);
                    let remaining = board.get_remaining_values();
                    answer = remaining * nr;
                    if first {
                        part_1 = answer;
                        first = false;
                    }
                }
            }
        }
    }
    (part_1, answer)
}

fn parse_input(input: Lines<BufReader<File>>) -> (Vec<Board>, Vec<u64>) {
    let mut start = true;
    let mut boards: Vec<Board> = Vec::new();
    let mut current_board = Board::new(0);
    let mut index = 0;
    let mut nrs = Vec::new();
    let mut board_index = 0;

    for line in input {
        index += 1;
        let line = line.unwrap();
        //println!("{}", line);
        if start {
            nrs = line
                .trim()
                .split(",")
                .map(|nr| nr.parse().unwrap())
                .collect();
            //println!("{:?}", nrs);
            start = false;
        } else if line == "" && index > 3 {
            //println!("new board");
            boards.push(current_board);
            current_board = Board::new(board_index);
            board_index += 1;
        } else if line != "" {
            current_board.add_row(
                line.trim()
                    .split_whitespace()
                    .map(|nr| nr.parse().unwrap())
                    .collect(),
            );
        }
    }
    boards.push(current_board);
    (boards, nrs)
}

#[cfg(test)]
mod tests {
    use crate::day4::{get_first_and_last_board_score};
    use crate::reader::get_lines;

    #[test]
    fn test_example() {
        let input = get_lines("./resources/inputs/day4-example.txt");
        let (winning, loosing) = get_first_and_last_board_score(input);
        assert_eq!(4512, winning);
        assert_eq!(2192, loosing);
    }

    #[test]
    fn test_input() {
        let input = get_lines("./resources/inputs/day4-input.txt");
        let (winning, loosing) = get_first_and_last_board_score(input);
        assert_eq!(60368, winning);
        assert_eq!(17435, loosing);
    }
}
