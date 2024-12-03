

#[cfg(test)]
mod tests {
    use petgraph::dot::{Config, Dot};
    use petgraph::graphmap::UnGraphMap;
    use petgraph::prelude::EdgeRef;
    
    #[test]
    fn example2() {
        use petgraph::algo::dijkstra;

        let grid = [['*', '*', '*'], ['*', ' ', '*'], ['*', ' ', '*']];

        // Create a graph
        let mut graph = UnGraphMap::<(usize, usize), usize>::new();

        // Add nodes and edges
        const SEARCH_CHAR: char = '*';
        const DEFAULT_WEIGHT: usize = 1;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == SEARCH_CHAR {
                    let node_index = (row, col);
                    graph.add_node(node_index);

                    // Add edges to neighbors
                    if row > 0 && grid[row - 1][col] == SEARCH_CHAR {
                        let neighbor_index = (row - 1, col);
                        graph.add_edge(node_index, neighbor_index, DEFAULT_WEIGHT);
                    }
                    if row < grid.len() - 1 && grid[row + 1][col] == SEARCH_CHAR {
                        let neighbor_index = (row + 1, col);
                        graph.add_edge(node_index, neighbor_index, DEFAULT_WEIGHT);
                    }
                    if col > 0 && grid[row][col - 1] == SEARCH_CHAR {
                        let neighbor_index = (row, col - 1);
                        graph.add_edge(node_index, neighbor_index, DEFAULT_WEIGHT);
                    }
                    if col < grid[0].len() - 1 && grid[row][col + 1] == SEARCH_CHAR {
                        let neighbor_index = (row, col + 1);
                        graph.add_edge(node_index, neighbor_index, DEFAULT_WEIGHT);
                    }
                }
            }
        }

        // Find the shortest path
        let start_node = (0, 0);
        let end_node = (grid.len(), grid[0].len());

        let path = dijkstra(&graph, start_node, Some(end_node), |edge| *edge.weight());

        // print graph in dot format, use online graphviz visualizer
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        if let Some(distance) = path.get(&end_node) {
            println!("Shortest distance: {}", distance);
        } else {
            println!("No path found.");
        }
    }

}