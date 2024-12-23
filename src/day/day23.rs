use crate::day::utils;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn run() {
    let input_string = utils::read_input(23);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

#[derive(Debug)]
struct NetworkGraph {
    nodes: HashSet<String>,
    edges: Vec<(String, String)>
}

impl NetworkGraph {
    fn new() -> Self {
        NetworkGraph {
            nodes: HashSet::new(),
            edges: vec![]
        }
    }

    fn insert_edge(&mut self, edge: (String, String)) {
        self.nodes.insert(edge.0.to_string());
        self.nodes.insert(edge.1.to_string());
        self.edges.push(edge);
    }

    fn adjacent_nodes(&self, node: &String) -> HashSet<&String> {
        let mut result = HashSet::new();
        for edge in &self.edges {
            if edge.0 == *node {
                result.insert(&edge.1);
            }
            if edge.1 == *node {
                result.insert(&edge.0);
            }
        }

        result
    }

    fn from_input(input_string: &str) -> Self {
        let mut graph = Self::new();

        input_string.lines()
            .map(|line| line.split("-"))
            .for_each(|mut split| {
                graph.insert_edge((split.next().unwrap().to_string(), split.next().unwrap().to_string()));
            });
        graph
    }
}

fn run_part_one(input_string: &str) -> usize {
    let graph = NetworkGraph::from_input(input_string);

    let mut three_connections = HashSet::new();
    for node in &graph.nodes {
        let adjacent_nodes = graph.adjacent_nodes(&node);
        for adjacent_node in &adjacent_nodes {
            let jump_adjacent_nodes = graph.adjacent_nodes(&&adjacent_node);
            for jump_adjacent_node in &jump_adjacent_nodes {
                if node != *jump_adjacent_node &&  adjacent_nodes.contains(jump_adjacent_node) {
                    let mut three_connection = vec![node.clone(), adjacent_node.to_string(), jump_adjacent_node.to_string()];
                    three_connection.sort();
                    three_connections.insert(three_connection);
                }
            }
        }
    }

    three_connections.iter()
        .filter(|triple|
            triple.iter().any(|node| node.starts_with("t")))
        .count()
}

fn run_part_two(input_string: &str) -> String {
    let graph = NetworkGraph::from_input(input_string);

    let mut max_path = vec![];
    for node in &graph.nodes {
        let mut path = HashSet::from([node]);
        let mut queue = VecDeque::from([node]);

        while let Some(new_node) = queue.pop_front() {
            let mut adjacent_nodes = graph.adjacent_nodes(&new_node);
            adjacent_nodes.insert(new_node);
            // adjacent nodes have to contain all path so far then process non-path elements

            if path.is_subset(&adjacent_nodes) {
                let new_nodes_to_explore: Vec<_> = adjacent_nodes.difference(&path).copied().collect();
                queue.extend(new_nodes_to_explore);
                path.insert(new_node);
            }
        }

        if path.len() > max_path.len() {
            max_path = path.into_iter().collect_vec();
            max_path.sort();
        }
    }

    max_path.into_iter().join(",")
}

#[cfg(test)]
mod tests {
    use crate::day::day23::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 7);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(23)), 1194);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), "co,de,ka,ta");
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(23)), "bd,bu,dv,gl,qc,rn,so,tm,wf,yl,ys,ze,zr");
    }
}
