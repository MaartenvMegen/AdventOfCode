use rustaoc2025::{get_input, parse_range};

fn get_ranges_and_ids(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut parts = input.trim().split("\n\n");
    let ranges = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|s| parse_range(s).unwrap())
        .collect();
    let ids = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect();
    (ranges, ids)
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    // Sort by start
    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current = ranges[0];

    for &(start, end) in ranges.iter().skip(1) {
        if start <= current.1 {
            // Extend the current range
            if end > current.1 {
                current.1 = end;
            }
        } else {
            // No overlap: push the current range and start a new one
            merged.push(current);
            current = (start, end);
        }
    }

    // Push the last range
    merged.push(current);

    merged
}

fn in_range(id: &u64, ranges: &Vec<(u64, u64)>) -> bool {
    for (start, end) in ranges {
        if id >= start && id <= end {
            return true;
        }
    }
    false
}

fn solve(input: &str) -> usize {
    let (ranges, ids) = get_ranges_and_ids(input);
    ids.into_iter().filter(|s| in_range(s, &ranges)).count()
}

fn solve2(input: &str) -> u64 {
    let (ranges, _ids) = get_ranges_and_ids(input);
    let merged = merge_ranges(ranges);
    merged.into_iter().map(|(min, max)| max - min + 1).sum()
}

fn main() {
    let input = get_input("day5-input.txt");

    println!("{}", solve(&input));
    println!("{}", solve2(&input));
}
