use std::collections::HashMap;

use rustaoc2025::get_input;

fn parse_line(line: &str) -> Option<(String, Vec<String>)> {
    // Split at the first colon
    let (src_part, rest) = line.split_once(':')?;

    // Source is left of the colon, trimmed
    let source = src_part.trim().to_string();

    // Right side: trim whitespace and trailing dot, then split by whitespace
    let rest = rest.trim().trim_end_matches('.');

    let destinations = rest
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    Some((source, destinations))
}

type MemoKey<'a> = (&'a str, bool, bool);

fn find_paths_memo<'a>(
    graph: &'a HashMap<String, Vec<String>>,
    node: &'a str,
    target: &str,
    has_fft: bool,
    has_dac: bool,
    memo: &mut HashMap<MemoKey<'a>, u64>,
) -> u64 {
    let key = (node, has_fft, has_dac);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let has_fft = has_fft || node == "fft";
    let has_dac = has_dac || node == "dac";

    let result = if node == target {
        if has_fft && has_dac {
            1
        } else {
            0
        }
    } else {
        let mut count = 0;
        if let Some(neighbors) = graph.get(node) {
            for dest in neighbors {
                count += find_paths_memo(graph, dest, target, has_fft, has_dac, memo);
            }
        }
        count
    };

    memo.insert(key, result);
    result
}

fn solve(input: &str) -> usize {
    let mut graph = std::collections::HashMap::new();

    for (source, destinations) in input.trim().split('\n').filter_map(parse_line) {
        graph.insert(source, destinations);
    }

    // get element you from the list, recurce until destination string is "out"

    find_paths_memo(&graph, "svr", "out", false, false, &mut HashMap::new()) as usize
}
fn main() {
    let input = get_input("day11-input.txt");

    println!("{}", solve(&input));
}
