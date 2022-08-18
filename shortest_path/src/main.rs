use std::collections::{HashMap, HashSet};

type Node = usize;
type Cost = usize;

#[derive(Debug)]
struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));
            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

fn shortest_path(g: &Graph, start: Node, end: Node) -> Option<(Vec<Node>, Cost)> {
    if !g.nodes.contains(&start) || !g.nodes.contains(&end) {
        println!("exited the program in first check");
        return None;
    }

    fn inner(
        start: Node,
        end: Node,
        g: &Graph,
        mut result: HashMap<Cost, Vec<Node>>,
        mut seen_nodes: Vec<Node>,
    ) -> HashMap<Cost, Vec<Node>> {

        if let Some(next_nodes) = g.edges.get(&start) {
            println!("\n\n{}, {}, \nresult: {:?}, \nseen_nodes: {:?}\n\n", start, end, result, seen_nodes);
            let mut path: Vec<Node> = vec![start];
            let mut running_cost = 0;

            for &(node, cost) in next_nodes {
                if seen_nodes.contains(&node) {
                    println!("found in seen_nodes");
                    continue;
                }

                seen_nodes.push(node);
                running_cost += cost;
                path.push(node);

                if node == end {
                    result.insert(running_cost, path.clone());
                    break;
                }

                inner(node, end, g, result.clone(), seen_nodes.clone());
            }
        }

        println!("reached the end of inner function");
        result.clone()
    }

    let result = inner(start, end, g, HashMap::new(), Vec::new());

    if result.is_empty() {
        println!("recursion returned None");
        None
    } else {
        let cost = result.keys().min().unwrap();
        let path = result.get(cost).unwrap().to_vec();
        Some((path, *cost))
    }
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);
    // println!("{:#?}", g);

    if let Some((path, cost)) = shortest_path(&g, 1, 7) {
        println!("1000 -> 9000, {:?} {}", path, cost);
    } else {
        println!("no luck!");
    }
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}
