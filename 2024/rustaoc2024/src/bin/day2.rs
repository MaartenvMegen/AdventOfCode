#[derive(PartialEq)]
enum SequenceState {
    Increasing,
    Decreasing,
    Mixed,
    Equal,
    ToBig,
    Start,
}

fn is_safe_sequence(levels: &[i32]) -> bool {
    let mut state = SequenceState::Start;
    let mut prev = &levels[0];

    for curr in levels.iter().skip(1) {
        let diff = curr - prev;

        state = match diff {
            0 => SequenceState::Equal,
            _ if diff.abs() > 3 => SequenceState::ToBig,
            _ if diff > 0 && state == SequenceState::Decreasing => SequenceState::Mixed,
            _ if diff < 0 && state == SequenceState::Increasing => SequenceState::Mixed,
            _ if diff > 0 && state != SequenceState::Decreasing => SequenceState::Increasing,
            _ if diff < 0 && state != SequenceState::Increasing => SequenceState::Decreasing,
            _ => SequenceState::Equal,
        };

        if state == SequenceState::Mixed
            || state == SequenceState::ToBig
            || state == SequenceState::Equal
        {
            return false;
        }

        prev = curr;
    }

    true
}

fn part1(input: &str) -> u64 {
    let parts: Vec<&str> = input.trim().split('\n').collect();
    let mut safe_count = 0;

    for line in parts {
        let iter = line.split_whitespace();

        let mut levels: Vec<i32> = Vec::new();
        for num_str in iter {
            levels.push(num_str.parse::<i32>().unwrap());
        }

        if is_safe_sequence(&levels) {
            safe_count += 1;
        }
    }

    safe_count
}

fn part2(input: &str) -> u64 {
    let parts: Vec<&str> = input.trim().split('\n').collect();
    let mut safe_count = 0;

    for line in parts {
        let iter = line.split_whitespace();

        let mut levels: Vec<i32> = Vec::new();
        for num_str in iter {
            levels.push(num_str.parse::<i32>().unwrap());
        }

        // Check if the report is safe without the Problem Dampener
        // If not safe, check if it can be made safe by removing one level
        let mut is_safe = is_safe_sequence(&levels);
        if !is_safe {
            for i in 0..levels.len() {
                let mut modified_levels = levels.clone();
                modified_levels.remove(i);

                if is_safe_sequence(&modified_levels) {
                    is_safe = true;
                    break;
                }
            }
        }

        if is_safe {
            safe_count += 1;
        }
    }

    safe_count
}

fn main() {
    let example = include_str!(r"../../resources/day2-example.txt");
    let input = include_str!(r"../../resources/day2-input.txt");
    rustaoc2024::run_matrix(part1, part2, example, input);
}
