use rustaoc2022::grid::{Grid, Location};
use std::collections::{HashMap, HashSet, LinkedList};

const EXAMPLE: &str = include_str!(r"../../resources/day20-example.txt");
const INPUT: &str = include_str!(r"../../resources/day20-input.txt");

#[derive(Debug)]
struct Node {
    value: i64,
    prev: usize,
    next: usize,
}

fn part1(input: &str) -> i64 {
    let nrs = collect_numbers(input);


    let (mut nodes, zero_index) = create_nodes(&nrs, 1);

    switch(nrs, nodes.as_mut_slice(), 1);

    // print results
    let mut output : Vec<i64> = Vec::new();
    let mut next_idx = zero_index;
    for i in 0..=3000 {
        if i % 1000 == 0 {
            output.push(nodes[next_idx].value);
            println!("{}", nodes[next_idx].value);
        }
        next_idx = nodes[next_idx].next;
    }

    output.iter().sum()
}

fn collect_numbers(input: &str) -> Vec<i64> {
    let nrs: Vec<i64> = input
        .trim()
        .split('\n')
        //.inspect(|value| println!("-{}-", value))
        .map(|nr| nr.parse::<i64>().unwrap())
        .collect();
    nrs
}

fn create_nodes(nrs: &Vec<i64>, key : i64) -> (Vec<Node>, usize) {
    let mut nodes: Vec<Node> = Vec::new();
    let mut zero_index = 0;

    for (index, element) in nrs.iter().enumerate() {
        let prev = if index > 0 { index - 1 } else { nrs.len() - 1 };
        if *element == 0 {
            zero_index = index;
        }
        nodes.push(Node {
            value: *element * key,
            prev,
            next: (index + 1) % (nrs.len()),
        })
    }
    (nodes, zero_index)
}

fn switch(nrs: Vec<i64>, nodes: &mut [Node], rounds : usize) {
    for _ in 0..rounds {
        for index in 0..nrs.len() {
            // switch x amounts prev
            let value = nodes[index].value;
            // 0 never moves and complete loop around is useless
            if value == 0 || value == (nodes.len() - 1) as i64 {
                continue
            }

            // unlink self
            let node_prev_idx = nodes[index].prev;
            let node_next_idx = nodes[index].next;
            nodes[node_prev_idx].next = node_next_idx;
            nodes[node_next_idx].prev = node_prev_idx;

            // switch x steps
            if value < 0 {
                let mut before_index = index;
                for _ in 0..value.abs() % (nodes.len() -1) as i64{
                    before_index = nodes[before_index].prev;
                }
                // insert
                let prev_ids = nodes[before_index].prev;
                //println!("inserting {} between value {} and {}", value, nodes[before_index].value, nodes[prev_ids].value);

                nodes[index].next = before_index;
                nodes[index].prev = prev_ids;
                nodes[before_index].prev = index;
                nodes[prev_ids].next = index;
            } else {
                let mut next_index = index;
                for _ in 0..value.abs() % (nodes.len() -1) as i64 {
                    next_index = nodes[next_index].next;
                }
                // insert
                let next_ids = nodes[next_index].next;
                //println!("inserting {} between value {} and {}", value, nodes[next_index].value, nodes[next_ids].value);

                nodes[index].next = next_ids;
                nodes[index].prev = next_index;
                nodes[next_index].next = index;
                nodes[next_ids].prev = index;

            }

            //print_nodes(nodes);
        }
    }

}

fn print_nodes(nodes: &mut [Node]) {
    let mut next_index = 0;
    for _ in 0..nodes.len() {
        print!(", {} ", nodes[next_index].value);
        next_index = nodes[next_index].next
    }
    // for (index, node) in nodes.iter().enumerate() {
    //     println!("{}: value {}, prev value: {}, next value: {}", index, node.value, nodes[node.prev].value, nodes[node.next].value)
    // }
    println!();
    println!();
}

fn part2(input: &str) -> i64 {
    let nrs: Vec<i64> = collect_numbers(input);

    let (mut nodes, zero_index) = create_nodes(&nrs, 811589153);

    switch(nrs, nodes.as_mut_slice(), 10);

    // print results
    let mut output : Vec<i64> = Vec::new();
    let mut next_idx = zero_index;
    for i in 0..=3000 {
        if i % 1000 == 0 {
            output.push(nodes[next_idx].value);
            println!("{}", nodes[next_idx].value);
        }
        next_idx = nodes[next_idx].next;
    }

    output.iter().sum()

}

fn main() {
    rustaoc2022::run_matrix(part1, part2, EXAMPLE, INPUT);
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, EXAMPLE, INPUT};

    #[test]
    fn test_example() {
        assert_eq!(3, part1(EXAMPLE));
        assert_eq!(1623178306, part2(EXAMPLE));
    }

    #[test]
    fn test_input() {
        assert_eq!(1087, part1(INPUT));
        assert_eq!(13084440324666, part2(INPUT));
    }

}
