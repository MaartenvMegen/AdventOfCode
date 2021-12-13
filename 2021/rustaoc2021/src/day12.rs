use crate::reader::get_lines;
use std::collections::{HashMap, HashSet};


fn part_1(filename: &str) -> usize {
    get_paths(filename, false)
}

fn part_2(filename: &str) -> usize {
    get_paths(filename, true)
}

fn get_paths(filename: &str, allow_revisits : bool) -> usize {
    let lines = get_lines(filename);
    // map node to other nodes

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let line = line.unwrap().clone();
        let parts: Vec<String> = line.split("-").map(|x| x.to_string()).collect();
        let node_a = parts[0].to_string();
        let node_b = parts[1].to_string();
        let nodes_for_a = graph.entry(node_a.clone()).or_insert(Vec::new());
        nodes_for_a.push(node_b.clone());
        let nodes_for_b = graph.entry(node_b.clone()).or_insert(Vec::new());
        nodes_for_b.push(node_a.clone());
    }

    println!("{:?}", graph);
    let visited_nodes: HashSet<String> = HashSet::new();
    let paths = find_path(&graph, "start".to_string(), visited_nodes, true, allow_revisits);
    let nr_paths: usize = paths.len();
    nr_paths
}

fn find_path<'a>(graph: &HashMap<String, Vec<String>>, target : String, visited_caves: HashSet<String>, free_pass: bool, allow_revisit : bool) -> Vec<Vec<String>> {
    // add itself to every path found on a lower level
    let mut paths_found : Vec<Vec<String>> = Vec::new();
    let mut visisted_caves = visited_caves;
    let cave_ref = &mut visisted_caves;

    if target == "end".to_string() {
        return vec![vec!["end".to_string()]]
    }

    if target.chars().any(|c| matches!(c, 'a'..='z')) {
        cave_ref.insert(target.clone());
    }

    for node in graph.get(&target).unwrap() {
        let mut free_pass = free_pass.clone();
        if node != "start" && (!visisted_caves.contains(node) || (free_pass && allow_revisit)) {
            //println!("now arriving at small cave {}, free pass {}, visited caves {:?}", node, free_pass, visisted_caves);
            if visisted_caves.contains(node) {
                //println!("disabling free pass");
                free_pass = false;
            }
            let caves = visisted_caves.clone();
            let mut paths = find_path(graph, node.clone(), caves, free_pass, allow_revisit);
            for path in &mut paths {
                path.push(target.clone())
            }
            paths_found.extend(paths);


        }
    }

    paths_found
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
