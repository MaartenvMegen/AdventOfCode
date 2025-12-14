use rustaoc2025::get_input;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
struct Machine {
    indicator_lights: u32,
    button_wiring: Vec<u32>,
    joltage: Vec<usize>,
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
        .map(|s| usize::from_str(s.trim()))
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
        println!("{:?}", machine);
        button_presses += set_joltage(machine);
    }
    button_presses as u64
}

fn set_indicator_lights(machine: Machine) -> usize {
    let mut experiments: VecDeque<u32> = VecDeque::new();
    experiments.push_back(0);
    let mut iterations: usize = 1;

    loop {
        let mut values = Vec::new();
        //println!("iterations {}", iterations);

        while let Some(value) = experiments.pop_front() {
            for button in &machine.button_wiring {
                //println!("button {} value {} result {}, desired value {}", button, value, value ^ button, machine.indicator_lights);
                let new_value = value ^ button;
                if new_value == machine.indicator_lights {
                    return iterations;
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

fn set_joltage(machine: Machine) -> usize {
    let mut experiments: VecDeque<Vec<usize>> = VecDeque::new();
    let mut visited: HashSet<Vec<usize>> = HashSet::new();

    // push in the empty vec
    experiments.push_back(vec![0; machine.joltage.len()]);
    let mut iterations: usize = 1;

    let mut button_vec: Vec<Vec<usize>> = Vec::new();

    for button in &machine.button_wiring {
        button_vec.push(
            (0..machine.joltage.len())
                .rev()
                .map(|i| (*button as usize >> i) & 1)
                .collect(),
        )
    }
    //println!("starting loop");
    loop {
        let mut values: Vec<Vec<usize>> = Vec::new();
        println!("iterations {}", iterations);

        while let Some(value) = experiments.pop_front() {
            if iterations > 18 {
                println!("value {:?} target {:?}", value, machine.joltage);
            }
            for button in &button_vec {
                //println!("button {} value {} result {}, desired value {}", button, value, value ^ button, machine.indicator_lights);
                // increment vec value by 1 if position matches criterium
                let new_value: Vec<usize> = button
                    .iter()
                    .zip(value.iter())
                    .map(|(x, y)| x + y)
                    .collect();

                if new_value == machine.joltage {
                    return iterations;
                }
                values.push(new_value);
            }
        }

        for value in &values {
            // only add novel values
            if !visited.contains(value) {
                experiments.push_back(value.clone());
                visited.insert(value.clone());
            }
        }

        iterations += 1;
        if iterations > 100 {
            panic!("Too many iterations");
        }
    }
}

fn lower_bound_extra_presses(remaining: &[i32]) -> i32 {
    *remaining.iter().max().unwrap_or(&0)
}

/// Find minimum total presses to exactly reach target joltage,
/// or None if impossible.
fn min_presses_backtracking(machine: &Machine) -> Option<i32> {
    let button_vec: Vec<Vec<i32>> = build_button_vec(machine);
    // for vec in &button_vec {
    //     println!("{:?}", vec);
    // }
    // return None;
    let n_buttons = button_vec.len();
    let n_indices = machine.joltage.len();

    // Edge cases
    if n_indices == 0 {
        return Some(0);
    }
    if n_buttons == 0 {
        return if machine.joltage.iter().all(|&x| x == 0) {
            Some(0)
        } else {
            None
        };
    }

    // Remaining joltage we still need at each index
    let mut remaining: Vec<i32> = machine.joltage.clone().iter().map(|&x| x as i32).collect();
    let mut best: Option<i32> = None;

    fn dfs(
        idx: usize,
        presses_so_far: i32,
        remaining: &mut [i32],
        buttons: &Vec<Vec<i32>>,
        best: &mut Option<i32>,
        memo: &mut HashMap<(usize, Vec<i32>), Option<i32>>,
    ) {
        // Global branch & bound with lower bound
        if let Some(b) = *best {
            let lb = lower_bound_extra_presses(remaining);
            if presses_so_far + lb >= b {
                return;
            }
        }

        // Goal reached
        if remaining.iter().all(|&r| r == 0) {
            match best {
                Some(b) if presses_so_far >= *b => {}
                _ => *best = Some(presses_so_far),
            }
            return;
        }

        // Out of buttons → dead end
        if idx == buttons.len() {
            return;
        }

        // Memoization: state is (idx, remaining vector)
        let state_key = (idx, remaining.to_vec());

        if let Some(cached_extra) = memo.get(&state_key) {
            if let (Some(global_best), Some(extra)) = (*best, *cached_extra) {
                // From this state, even optimal continuation can't beat global best
                if presses_so_far + extra >= global_best {
                    return;
                }
            }
            // If cached_extra is None, we know this state never improved best; we can still prune:
            if cached_extra.is_none() {
                return;
            }
        }

        // Save previous global best to compute local gain later
        let prev_best = *best;

        let btn = &buttons[idx];

        // Compute maximum times we can press this button without overshooting
        let mut max_cnt: i32 = i32::MAX;
        let mut has_effect = false;

        for (k, &val) in btn.iter().enumerate() {
            if val == 1 {
                has_effect = true;
                max_cnt = max_cnt.min(remaining[k]);
            }
        }

        if !has_effect {
            // button has no effect, just skip it
            dfs(idx + 1, presses_so_far, remaining, buttons, best, memo);
        } else if max_cnt <= 0 {
            // can't use this button at all, skip to next
            dfs(idx + 1, presses_so_far, remaining, buttons, best, memo);
        } else {
            // Try pressing this button cnt times, from 0..=max_cnt
            for cnt in 0..=max_cnt {
                if cnt > 0 {
                    for (k, &val) in btn.iter().enumerate() {
                        if val == 1 {
                            remaining[k] -= cnt;
                        }
                    }
                }

                dfs(
                    idx + 1,
                    presses_so_far + cnt,
                    remaining,
                    buttons,
                    best,
                    memo,
                );

                if cnt > 0 {
                    for (k, &val) in btn.iter().enumerate() {
                        if val == 1 {
                            remaining[k] += cnt;
                        }
                    }
                }
            }
        }

        // Compute how much better (locally) this state can do compared to when we entered it
        let local_extra: Option<i32> = match (prev_best, *best) {
            (None, Some(new_best)) => Some(new_best - presses_so_far),
            (Some(old_best), Some(new_best)) if new_best < old_best => {
                Some(new_best - presses_so_far)
            }
            _ => None, // this state never produced an improvement
        };

        memo.insert(state_key, local_extra);
    }
    let mut memo: HashMap<(usize, Vec<i32>), Option<i32>> = HashMap::new();
    dfs(0, 0, &mut remaining, &button_vec, &mut best, &mut memo);
    best
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

    // Sort buttons by "strength": more ones, or heavier indices first
    let mut indexed: Vec<(usize, Vec<i32>)> = button_vec.into_iter().enumerate().collect();

    indexed.sort_by_key(|(_, bits)| {
        // negative: we want descending
        let ones = bits.iter().filter(|&&x| x == 1).count() as i32;
        -ones
    });

    indexed.into_iter().map(|(_, bits)| bits).collect()
}

fn main() {
    let input = get_input("day10-input.txt");
    println!("{}", solve(&input));
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
        assert_eq!(10, min_presses_backtracking(&machine).unwrap());

        assert_eq!(10, set_joltage(machine));

        let line = "[...#...] (0,2,3,6) (0,1,4,6) (1,3,4,5) (1,2,4,6) (0,2,3,4,5) (2,3,6) (1,2) (2,3,4,5,6) {37,24,84,71,44,32,71}";
        let machine = parse_line(line).unwrap();
        println!("{:?}", min_presses_backtracking(&machine));

        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_line(line).unwrap();
        assert_eq!(12, min_presses_backtracking(&machine).unwrap());

        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_line(line).unwrap();
        assert_eq!(11, min_presses_backtracking(&machine).unwrap());
    }
}
