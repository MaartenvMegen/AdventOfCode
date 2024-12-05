use std::fs;

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }
    grid
}

fn find_xmas(grid: &[Vec<char>], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();

    let mut count = 0;

    // Check horizontal and vertical
    for i in 0..rows {
        for j in 0..cols {
            if j + word_len <= cols {
                let leftright = grid[i][j..j + word_len].to_vec();
                let rightleft = leftright.clone().into_iter().rev().collect::<Vec<_>>();

                if leftright == word.chars().collect::<Vec<_>>() {
                    count += 1;
                }
                if rightleft == word.chars().collect::<Vec<_>>() {
                    count += 1;
                }
            }

            if i + word_len <= rows {
                let topdown = grid
                    .iter()
                    .skip(i)
                    .take(word_len)
                    .map(|row| row[j])
                    .collect::<Vec<_>>();
                let bottomup = topdown.clone().into_iter().rev().collect::<Vec<_>>();

                if topdown == word.chars().collect::<Vec<_>>() {
                    count += 1;
                }
                if bottomup == word.chars().collect::<Vec<_>>() {
                    count += 1;
                }
            }
        }
    }
    // Check diagonals
    for i in 0..rows {
        for j in 0..cols {
            // Top-left to bottom-right and bottom-right to top-left diagonals
            if i + word_len - 1 < rows && j + word_len - 1 < cols {
                let forward_diagonal = grid
                    .iter()
                    .skip(i)
                    .enumerate()
                    .take(word_len)
                    .map(|(k, row)| row[j + k])
                    .collect::<Vec<_>>();
                let backward_diagonal = forward_diagonal
                    .clone()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>();

                if forward_diagonal == word.chars().collect::<Vec<_>>() {
                    count += 1;
                }
                if backward_diagonal == word.chars().collect::<Vec<_>>() {
                    count += 1;
                }
            }

            // Top-right to bottom-left and bottom-left to top-right diagonals
            if i + word_len - 1 < rows && j + word_len - 1 < cols {
                let forward_diagonal = grid
                    .iter()
                    .rev()
                    .skip(i)
                    .enumerate()
                    .take(word_len)
                    .map(|(k, row)| row[j + k])
                    .collect::<Vec<_>>();
                let backward_diagonal = forward_diagonal
                    .clone()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>();

                if forward_diagonal == word.chars().collect::<Vec<_>>() {
                    count += 1;
                }
                if backward_diagonal == word.chars().collect::<Vec<_>>() {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_a(input: &str) -> usize {
    let word = "XMAS";
    find_xmas(&parse_grid(input), word)
}

fn find_xmas2(grid: &[Vec<char>], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row][col] == 'A' {
                // check neighbours form MAS
                let cross_a: String = [
                    grid[row - 1][col - 1],
                    grid[row][col],
                    grid[row + 1][col + 1],
                ]
                .into_iter()
                .collect();
                let cross_b: String = [
                    grid[row + 1][col - 1],
                    grid[row][col],
                    grid[row - 1][col + 1],
                ]
                .into_iter()
                .collect();
                let cross_c: String = cross_a.chars().rev().collect();
                let cross_d: String = cross_b.chars().rev().collect();
                if (cross_a == word || cross_c == word) && (cross_b == word || cross_d == word) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_b(input: &str) -> usize {
    find_xmas2(&parse_grid(input), "MAS")
}

fn main() {
    let input = fs::read_to_string(r"2024/rustaoc2024/resources/day4-input.txt").unwrap();
    println!("{}", part_a(&input));
    println!("{}", part_b(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_no_xmas() {
        let grid = vec![
            vec!['M', 'B', 'M', 'D'],
            vec!['E', 'A', 'G', 'H'],
            vec!['S', 'J', 'S', 'L'],
            vec!['M', 'N', 'O', 'P'],
        ];
        assert_eq!(find_xmas2(&grid, "MAS"), 1);
    }

    #[test]
    fn test_horizontal_match() {
        let grid = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
        ];

        let word = "XMAS";
        assert_eq!(find_xmas(&grid, word), 6);
    }

    #[test]
    fn test_vertical_match() {
        let grid = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['M', 'M', 'M', 'M'],
            vec!['A', 'A', 'A', 'A'],
            vec!['S', 'S', 'S', 'S'],
        ];

        let word = "XMAS";
        assert_eq!(find_xmas(&grid, word), 6);
    }

    #[test]
    fn test_diagonal_top_left_to_bottom_right() {
        let grid = vec![
            vec!['X', 'B', 'C', 'D'],
            vec!['E', 'M', 'G', 'H'],
            vec!['I', 'A', 'A', 'L'],
            vec!['M', 'N', 'O', 'S'],
        ];

        let word = "XMAS";
        assert_eq!(find_xmas(&grid, word), 1);
    }

    #[test]
    fn test_diagonal_bottom_left_to_top_right() {
        let grid = vec![
            vec!['A', 'B', 'C', 'S'],
            vec!['E', 'F', 'A', 'M'],
            vec!['I', 'M', 'K', 'D'],
            vec!['X', 'N', 'O', 'S'],
        ];

        let word = "XMAS";
        assert_eq!(find_xmas(&grid, word), 1);
    }

    #[test]
    fn test_diagonal_top_right_to_bottom_left() {
        let grid = vec![
            vec!['A', 'B', 'C', 'X'],
            vec!['E', 'F', 'M', 'M'],
            vec!['I', 'A', 'K', 'F'],
            vec!['S', 'N', 'O', 'D'],
        ];

        let word = "XMAS";
        assert_eq!(find_xmas(&grid, word), 1);
    }

    #[test]
    fn test_diagonal_bottom_right_to_top_left() {
        let grid = vec![
            vec!['S', 'B', 'C', 'X'],
            vec!['E', 'A', 'G', 'M'],
            vec!['I', 'J', 'M', 'A'],
            vec!['M', 'N', 'O', 'X'],
        ];

        let word = "XMAS";
        assert_eq!(find_xmas(&grid, word), 1);
    }
}
