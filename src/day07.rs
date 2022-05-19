// Potential improvements:
//
// thoughts:
// can maybe do something with graphs or graphmaps here
// make a directed graph, get starting node, find neighbours,
// rank alphabetical, pick next node,
// add all previous node's neighbours as neighbours to current node,
// remove previous node (add to list),
// find neighbours, rank alphabetical, rinse and repeat...

use petgraph::{
    dot::Dot,
    graphmap::{DiGraphMap, GraphMap},
    Directed,
    EdgeDirection::{Incoming, Outgoing},
    Graph,
};
use regex::Regex;

pub fn day07(input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    let mut graph = build_graph_from_input(&input_lines[0]);
    // print_graph_dot(&graph); // print out dot graph viz format to view as picture

    let mut items = vec![];
    let mut last_item = find_first_item(&mut graph);
    while graph.node_count() > 1 {
        let next_item = find_next_item(&mut graph, last_item);
        graph.remove_node(last_item);
        items.push(last_item);
        last_item = next_item;
    }
    items.push(last_item);

    let answer1 = items.iter().collect::<String>();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn find_next_item(graph: &mut GraphMap<char, i32, Directed>, previous_item: char) -> char {
    // find all dependent nodes
    // for each dependent node, if it has more than one dependency, it's not available
    let mut item_set: Vec<char> = graph
        .neighbors_directed(previous_item, Outgoing)
        .filter(|node| graph.neighbors_directed(*node, Incoming).count() == 1)
        .collect();

    // debug
    println!(
        "From node {}, nodes {:?} are available",
        previous_item, item_set
    );

    // alphabetise the options
    item_set.sort();
    let first_item = item_set.get(0).unwrap().to_owned();
    item_set.remove(0);

    // make the remaining items depend on the selected node
    for node in item_set {
        graph.add_edge(first_item, node, 1);
    }
    println!("Choosing {}", first_item);

    first_item
}

fn find_first_item(graph: &mut GraphMap<char, i32, Directed>) -> char {
    // find all items with no dependencies
    let mut starting_set = vec![];
    for node in graph.nodes() {
        if graph.neighbors_directed(node, Incoming).count() == 0 {
            starting_set.push(node);
        }
    }

    // alphabetise the starting options
    starting_set.sort();
    let first_item = starting_set.get(0).unwrap().to_owned();
    starting_set.remove(0);

    // make the remaining items depend on the first node
    for node in starting_set {
        graph.add_edge(first_item, node, 1);
    }

    first_item
}

fn build_graph_from_input(input: &Vec<String>) -> DiGraphMap<char, i32> {
    let mut graph = DiGraphMap::new();
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    for input_line in input {
        let caps = re.captures(input_line).unwrap();
        let mut vals = caps
            .iter()
            .skip(1)
            .map(|hit| hit.unwrap().as_str().chars().nth(0).unwrap());
        let x = vals.next().unwrap();
        let y = vals.next().unwrap();
        graph.add_edge(x, y, 1);
    }
    graph
}

fn print_graph_dot(graph: &GraphMap<char, i32, Directed>) {
    let dot = Dot::new(graph);
    println!("{:?}", dot);
}

#[cfg(test)]
mod tests {
    use super::day07;
    use crate::utils::load_input;

    #[test]
    fn check_day07_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day07(&input_lines, &"a".to_string()),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
