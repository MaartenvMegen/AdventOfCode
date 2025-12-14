use rustaoc2025::get_input;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
struct Machine {
    indicator_lights: u32,
    button_wiring: Vec<u32>,
    joltage: Vec<i32>,
}

const INF: i32 = 1_000_000; // big "impossible" value

#[derive(Debug, Clone)]
struct Pattern {
    contrib: Vec<i32>, // summed contribution over all positions
    cost: i32,         // how many buttons are used in this pattern
}

/// Build all patterns (2^buttons) from a generic button matrix.
/// buttons: Vec<button>, button: Vec<i32> (usually 0/1)
fn build_patterns(buttons: &[Vec<i32>]) -> Vec<Pattern> {
    let m = buttons.len();
    if m == 0 {
        return vec![Pattern {
            contrib: vec![],
            cost: 0,
        }];
    }

    let n = buttons[0].len();
    let mut patterns = Vec::new();

    for mask in 0..(1usize << m) {
        let mut contrib = vec![0i32; n];
        let mut cost = 0;

        for (btn_idx, btn) in buttons.iter().enumerate() {
            if (mask & (1 << btn_idx)) != 0 {
                cost += 1;
                for (i, &val) in btn.iter().enumerate() {
                    contrib[i] += val;
                }
            }
        }

        patterns.push(Pattern { contrib, cost });
    }

    patterns
}

/// Recursive solver: minimum presses to reach `target` using `patterns`.
/// Returns INF if impossible.
fn solve_min_presses(
    target: &[i32],
    patterns: &[Pattern],
    memo: &mut HashMap<Vec<i32>, i32>,
) -> i32 {
    // base case: all zeros
    if target.iter().all(|&x| x == 0) {
        return 0;
    }

    // memoization key
    let key = target.to_vec();
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut best = INF;

    'pattern_loop: for pattern in patterns {
        // dimension must match
        if pattern.contrib.len() != target.len() {
            // you can also panic here if mismatched data should never happen
            panic!(
                "Pattern length mismatch: {} vs {}",
                pattern.contrib.len(),
                target.len()
            );
        }

        let mut reduced = Vec::with_capacity(target.len());

        // diff = target - contrib; must be nonnegative and even
        for (i, target_value) in target.iter().enumerate() {
            let diff = target_value - pattern.contrib[i];
            if diff < 0 || diff % 2 != 0 {
                continue 'pattern_loop; // invalid pattern for this target
            }
            // valid pattern: add half of the diff to reduced
            reduced.push(diff / 2);
        }

        let min_presses = solve_min_presses(&reduced, patterns, memo);
        if min_presses >= INF {
            // break out of loop: impossible to reach target with this pattern
            continue;
        }

        let candidate = pattern.cost + 2 * min_presses;
        if candidate < best {
            best = candidate;
        }
    }

    memo.insert(key, best);
    best
}

/// Public helper: given a target joltage vector and button matrix,
/// compute minimal presses (or None if impossible).
pub fn min_presses_generic(target: Vec<i32>, buttons: Vec<Vec<i32>>) -> Option<i32> {
    if target.is_empty() {
        return Some(0);
    }

    // sanity: all buttons same length as target
    let n = target.len();
    for (i, b) in buttons.iter().enumerate() {
        if b.len() != n {
            panic!("button {i} has length {}, expected {n}", b.len());
        }
    }

    let patterns = build_patterns(&buttons);
    let mut memo = HashMap::new();
    let ans = solve_min_presses(&target, &patterns, &mut memo);
    if ans >= INF {
        None
    } else {
        Some(ans)
    }
}

fn parse_line(line: &str) -> Result<Machine, String> {
    // === Parse bracket block [.##.] ===
    let bracket_start = line.find('[').ok_or("No '[' found")?;
    let bracket_end = line[bracket_start..].find(']').ok_or("No ']' found")? + bracket_start;

    let bracket_content = &line[bracket_start + 1..bracket_end];

    // # =1, . =0
    let mut bracket_value: u32 = 0;
    for ch in bracket_content.chars().rev() {
        bracket_value <<= 1;
        if ch == '#' {
            bracket_value |= 1;
        }
    }

    // This is the bit-width for parentheses bitmasks
    let bit_width = bracket_content.len();

    // === Parse ( ... ) blocks ===
    let mut paren_values = Vec::new();
    let mut rest = &line[bracket_end + 1..];

    while let Some(open) = rest.find('(') {
        let close = rest[open..].find(')').ok_or("Unmatched '('")? + open;
        let inside = &rest[open + 1..close].trim();

        if !inside.is_empty() {
            let mut value = 0u32;

            for part in inside.split(',') {
                let idx: usize = usize::from_str(part.trim())
                    .map_err(|e| format!("Invalid index '{}': {}", part, e))?;

                if idx >= bit_width {
                    return Err(format!(
                        "Index {} out of range for width {}",
                        idx, bit_width
                    ));
                }

                // NEW: direct bit position (0 = rightmost)
                value |= 1 << idx;
            }

            paren_values.push(value);
        }

        rest = &rest[close + 1..];
    }

    // === Parse { ... } block ===
    let curly_start = line.find('{').ok_or("No '{' found")?;
    let curly_end = line[curly_start..].find('}').ok_or("No '}' found")? + curly_start;

    let curly_content = &line[curly_start + 1..curly_end];
    let curly_values = curly_content
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| i32::from_str(s.trim()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid number in {{}}: {}", e))?;

    Ok(Machine {
        indicator_lights: bracket_value,
        button_wiring: paren_values,
        joltage: curly_values,
    })
}

fn solve(input: &str) -> u64 {
    let machines = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut button_presses = 0;
    for machine in machines {
        button_presses += set_indicator_lights(machine) as u64;
    }
    button_presses
}

fn solve2(input: &str) -> u64 {
    let machines = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut button_presses = 0;
    for machine in machines {
        let button_vec: Vec<Vec<i32>> = build_button_vec(&machine);
        button_presses += min_presses_generic(machine.joltage, button_vec).unwrap();
    }
    button_presses as u64
}

fn set_indicator_lights(machine: Machine) -> usize {
    let buttons = machine.button_wiring;
    let target_value = machine.indicator_lights;

    if let Some(value) = get_iterations(&buttons, target_value) {
        return value;
    }
    10000000
}

fn get_iterations(buttons: &Vec<u32>, target_value: u32) -> Option<usize> {
    let mut experiments: VecDeque<u32> = VecDeque::new();
    experiments.push_back(0);
    let mut iterations: usize = 1;

    loop {
        let mut values = Vec::new();
        //println!("iterations {}", iterations);

        while let Some(value) = experiments.pop_front() {
            for button in buttons {
                //println!("button {} value {} result {}, desired value {}", button, value, value ^ button, machine.indicator_lights);
                let new_value = value ^ button;
                if new_value == target_value {
                    return Some(iterations);
                }
                values.push(new_value);
            }
        }
        experiments.append(&mut values.into_iter().collect());

        iterations += 1;
        if iterations > 10 {
            panic!("Too many iterations");
        }
    }
}

fn build_button_vec(machine: &Machine) -> Vec<Vec<i32>> {
    let mut button_vec: Vec<Vec<i32>> = Vec::new();
    for button in &machine.button_wiring {
        button_vec.push(
            (0..machine.joltage.len())
                .map(|i| ((*button >> i) & 1) as i32)
                .collect(),
        )
    }

    button_vec
}

fn main() {
    let input = get_input("day10-input.txt");
    println!("{}", solve(&input));
    println!("{}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_line(line).unwrap();

        let value = machine.button_wiring[0] ^ machine.button_wiring[1] ^ machine.button_wiring[2];
        assert_eq!(value, machine.indicator_lights);
        assert_eq!(2, set_indicator_lights(machine));

        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_line(line).unwrap();
        assert_eq!(3, set_indicator_lights(machine));
    }

    #[test]
    fn test_joltage() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_line(line).unwrap();
        let button_vec: Vec<Vec<i32>> = build_button_vec(&machine);

        assert_eq!(
            10,
            min_presses_generic(machine.joltage, button_vec).unwrap()
        );

        let line = "[...#...] (0,2,3,6) (0,1,4,6) (1,3,4,5) (1,2,4,6) (0,2,3,4,5) (2,3,6) (1,2) (2,3,4,5,6) {37,24,84,71,44,32,71}";
        let machine = parse_line(line).unwrap();
        let button_vec: Vec<Vec<i32>> = build_button_vec(&machine);
        assert_eq!(
            90,
            min_presses_generic(machine.joltage, button_vec).unwrap()
        );

        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_line(line).unwrap();
        let button_vec: Vec<Vec<i32>> = build_button_vec(&machine);

        assert_eq!(
            12,
            min_presses_generic(machine.joltage, button_vec).unwrap()
        );

        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_line(line).unwrap();
        let button_vec: Vec<Vec<i32>> = build_button_vec(&machine);

        assert_eq!(
            11,
            min_presses_generic(machine.joltage, button_vec).unwrap()
        );
    }
}
