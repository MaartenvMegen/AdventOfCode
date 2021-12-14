use crate::reader::get_lines;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};


pub fn part_1(filename: &str) -> usize {
    get_paths(filename, false)
}

pub fn part_2(filename: &str) -> usize {
    get_paths(filename, true)
}

fn get_paths(filename: &str, allow_revisits : bool) -> usize {
    let lines = get_lines(filename);

    let maze = parse_maze(lines);

    let visited_nodes: HashSet<&str> = HashSet::new();
    let paths = maze.find_paths_from_node( "start", visited_nodes, true, allow_revisits);
    let nr_paths: usize = paths.len();
    nr_paths
}

fn parse_maze(lines: Lines<BufReader<File>>) -> Maze {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let parts: Vec<String> = line.split("-").map(|x| x.to_string()).collect();
        let node_a = parts[0].to_string();
        let node_b = parts[1].to_string();
        let nodes_for_a = graph.entry(node_a.clone()).or_insert(Vec::new());
        nodes_for_a.push(node_b.clone());
        let nodes_for_b = graph.entry(node_b).or_insert(Vec::new());
        nodes_for_b.push(node_a);
    }

    let mut small_caves: HashSet<String> = HashSet::new();
    for node in graph.keys() {
        if node.chars().any(|c| matches!(c, 'a'..='z')) {
            small_caves.insert(node.clone());
        }
    }
    let maze = Maze { small_caves, cave_layout: graph };
    maze
}

struct Maze {
    small_caves : HashSet<String>,
    cave_layout : HashMap<String, Vec<String>>
}

impl Maze {
    fn find_paths_from_node<'a>(&'a self, node:  &'a str, mut visited_caves: HashSet<&'a str>, free_pass: bool, allow_revisit : bool) -> Vec<Vec<&'a str>> {
        if node == "end" {
            return vec![vec!["end"]]
        }

        if self.small_caves.contains(node) {
            visited_caves.insert(node);
        }

        let mut paths_found : Vec<Vec<&str>> = Vec::new();
        for node in self.cave_layout.get(&*node).unwrap() {
            if node != "start" && (!visited_caves.contains(&**node) || (free_pass && allow_revisit)) {
                // this clone needs to be here to make sure we don't infect other paths
                let mut free_pass = free_pass.clone();
                // this clone also needs to be here to prevent infecting other paths
                let caves = visited_caves.clone();

                if visited_caves.contains(&**node) {
                    free_pass = false;
                }
                let mut paths = self.find_paths_from_node(node, caves, free_pass, allow_revisit);
                // add current node to paths found from the next node
                for path in &mut paths {
                    path.push(node)
                }
                // extend total paths with current paths
                paths_found.extend(paths);
            }
        }
        paths_found
    }
}




#[cfg(test)]
mod tests {
    use crate::day12::{part_1, part_2};

    #[test]
    fn test_part1() {
        assert_eq!(10, part_1("./resources/inputs/day12-example.txt"));
        assert_eq!(5104, part_1("./resources/inputs/day12-input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(36, part_2("./resources/inputs/day12-example.txt"));
        assert_eq!(149220, part_2("./resources/inputs/day12-input.txt"));
    }
}
