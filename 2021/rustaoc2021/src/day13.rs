use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part_1(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut dots: HashSet<(u64, u64)> = parts[0]
        .trim()
        .lines()
        .map(|line| line_to_coord(line))
        .collect();

    let folds: Vec<(String, u64)> = parts[1].lines().map(|line| line_to_fold(line)).collect();

    let (fold_info, index) = folds[0].clone();
    if fold_info == "fold along y".to_string() {
        dots = fold(dots, None, Some(index))
    } else {
        dots = fold(dots, Some(index), None)
    }
    //print_dots(&dots) ;

    dots.len()
}

pub fn part_2(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut dots: HashSet<(u64, u64)> = parts[0]
        .trim()
        .lines()
        .map(|line| line_to_coord(line))
        .collect();

    let folders: Vec<(String, u64)> = parts[1].lines().map(|line| line_to_fold(line)).collect();

    for (fold_info, index) in folders {
        if fold_info == "fold along y".to_string() {
            dots = fold(dots, None, Some(index))
        } else {
            dots = fold(dots, Some(index), None)
        }
    }

    print_dots(&dots);

    dots.len()
}

fn print_dots(dots: &HashSet<(u64, u64)>) {
    let xvalues: Vec<u64> = dots.iter().map(|(x, _y)| *x).collect();
    let yvalues: Vec<u64> = dots.iter().map(|(_x, y)| *y).collect();
    let (xmin, xmax) = min_max(&xvalues);
    let (ymin, ymax) = min_max(&yvalues);

    for y in ymin..=ymax {
        let mut line = String::new();
        for x in xmin..=xmax {
            if dots.contains(&(x, y)) {
                line.push('#');
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);
    }
}

pub fn min_max(input: &Vec<u64>) -> (u64, u64) {
    let min_max = (u64::MAX, u64::MIN);
    input.iter().fold(min_max, |(mut min, mut max), nr| {
        if min > *nr {
            min = *nr
        }
        if max < *nr {
            max = *nr
        }
        (min, max)
    })
}

fn fold(
    mut dots: HashSet<(u64, u64)>,
    fold_x: Option<u64>,
    fold_y: Option<u64>,
) -> HashSet<(u64, u64)> {
    if let Some(x_fold) = fold_x {
        let shifted_dots: HashSet<(u64, u64)> = HashSet::from_iter(
            dots.iter()
                .filter(|(x, _y)| x > &x_fold)
                .map(|(x, y)| (x_fold - (x - &x_fold), *y))
                .collect::<Vec<(u64, u64)>>(),
        );
        dots.retain(|(x, _y)| x < &x_fold);
        return dots.union(&shifted_dots).copied().collect();
    }

    if let Some(y_fold) = fold_y {
        let shifted_dots: HashSet<(u64, u64)> = HashSet::from_iter(
            dots.iter()
                .filter(|(_x, y)| y > &y_fold)
                .map(|(x, y)| (*x, y_fold - (y - &y_fold)))
                .collect::<Vec<(u64, u64)>>(),
        );
        dots.retain(|(_x, y)| y < &y_fold);
        return dots.union(&shifted_dots).copied().collect();
    }

    HashSet::new()
}

fn line_to_fold(line: &str) -> (String, u64) {
    let spec = line.split("=").collect::<Vec<&str>>();
    let axis = spec[0].to_string();
    let amount: u64 = spec[1].parse().unwrap();
    (axis, amount)
}

fn line_to_coord(line: &str) -> (u64, u64) {
    let xy = line.trim().split(",").collect::<Vec<&str>>();
    (xy[0].parse().unwrap(), xy[1].parse().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::day13::{part_1, part_2};

    #[test]
    fn test_part1() {
        let input = include_str!(r"../resources/inputs/day13-example.txt");
        assert_eq!(17, part_1(input));
        let input2 = include_str!(r"../resources/inputs/day13-input.txt");
        assert_eq!(745, part_1(input2));
    }

    #[test]
    fn test_part2() {
        let input1 = include_str!(r"../resources/inputs/day13-example.txt");
        assert_eq!(16,part_2(input1));
        let input2 = include_str!(r"../resources/inputs/day13-input.txt");
        assert_eq!(99,part_2(input2));
    }
}
