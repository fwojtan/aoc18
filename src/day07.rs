// Potential improvements:
//
// part one thoughts:
// can maybe do something with graphs or graphmaps here
// make a directed graph, get starting node, find neighbours,
// rank alphabetical, pick next node,
// add all previous node's neighbours as neighbours to current node,
// remove previous node (add to list),
// find neighbours, rank alphabetical, rinse and repeat...
//
// part two thoughts:
// can have a loop running through seconds, tracking whether each worker is occupied,
// Only remove tasks from the graph once complete, only
// find the next task when a worker picks a task up

use petgraph::{
    dot::Dot,
    graphmap::{DiGraphMap, GraphMap},
    Directed,
    EdgeDirection::{Incoming, Outgoing},
};
use regex::Regex;

pub fn day07(input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    let answer1 = part_one(&input_lines[0]);
    let answer2 = part_two(&input_lines[0]);
    (format!("{}", answer1), format!("{}", answer2))
}

fn part_one(input: &Vec<String>) -> String {
    let mut graph = build_graph_from_input(&input);
    // print_graph_dot(&graph); // print out dot graph viz format to view as picture
    let mut items = vec![];

    // finding the starting point is slightly different to finding subsequent nodes
    let mut last_item = find_first_avail_item(&mut graph).unwrap();
    loop {
        let next_item = find_next_item(&mut graph, last_item);
        graph.remove_node(last_item);
        items.push(last_item);
        if let Some(next_item) = next_item {
            last_item = next_item;
        } else {
            break;
        }
    }

    items.iter().collect::<String>()
}

fn part_two(input: &Vec<String>) -> i32 {
    let mut graph = build_graph_from_input(&input);

    // track the task letter and the end-time of a task
    let mut active_tasks: Vec<Option<(char, i32)>> = vec![None; 5];
    let mut time: i32 = 0;
    // each iteration represents one second
    loop {
        // pickup tasks for all available workers (check if None)
        for i in 0..active_tasks.len() {
            if active_tasks[i].is_none() {
                // work out next task (if one exists)
                if let Some(new_task) = find_first_avail_item(&mut graph) {
                    // calculate task end-time and add to active tasks (need the -1, missing it turned out to be an annoying bug)
                    active_tasks[i] =
                        Some((new_task, time + task_letter_to_duration(&new_task) - 1));

                    // add the task as its own dependency so it doesn't get picked up again
                    graph.add_edge(new_task, new_task, 2);
                }
            }
        }

        if active_tasks.iter().all(|task| task.is_none()) {
            break;
        }

        // check if this is final second of task for any workers
        for i in 0..active_tasks.len() {
            if let Some(task_details) = active_tasks[i] {
                if task_details.1 == time {
                    // stop tracking the task as active
                    active_tasks[i] = None;
                    // remove the task as a dependency in the task graph
                    graph.remove_node(task_details.0);
                }
            }
        }

        time += 1;
    }

    time
}

fn find_next_item(graph: &mut GraphMap<char, i32, Directed>, previous_item: char) -> Option<char> {
    // find all dependent nodes
    // for each dependent node, if it has more than one dependency, it's not available
    let mut item_set: Vec<char> = graph
        .neighbors_directed(previous_item, Outgoing)
        .filter(|node| graph.neighbors_directed(*node, Incoming).count() == 1)
        .collect();

    if item_set.is_empty() {
        None
    } else {
        // alphabetise the options
        item_set.sort();
        let first_item = item_set.get(0).unwrap().to_owned();
        item_set.remove(0);

        // make the remaining items depend on the selected node
        for node in item_set {
            graph.add_edge(first_item, node, 0);
        }

        Some(first_item)
    }
}

fn find_first_avail_item(graph: &mut GraphMap<char, i32, Directed>) -> Option<char> {
    // find all items with no 'weight 1' dependencies
    // weight 1 dependencies are ones specified by the input
    // weight 0 ones are ones I have added that shouldn't count
    // weight 2 should block the node again (number not significant in any way)
    // this is stupidly complicated, but I promise it made sense when there was
    // just a part 1
    let mut starting_set = vec![];
    for node in graph.nodes() {
        if graph
            .neighbors_directed(node, Incoming)
            .filter(|neighbour| graph.edge_weight(*neighbour, node).unwrap().to_owned() > 0)
            .count()
            == 0
        {
            starting_set.push(node);
        }
    }

    if starting_set.is_empty() {
        None
    } else {
        // alphabetise the starting options
        starting_set.sort();
        let first_item = starting_set.get(0).unwrap().to_owned();
        starting_set.remove(0);

        // make the remaining items depend on the first node
        for node in starting_set {
            graph.add_edge(first_item, node, 0);
        }

        Some(first_item)
    }
}

#[allow(dead_code)] // I implemented this when I thought I needed it (I didn't..)
fn prune_incoming_edges(graph: &mut GraphMap<char, i32, Directed>, node: char) {
    let neighbours_to_prune: Vec<char> = graph.neighbors_directed(node, Incoming).collect();
    for neighbour in neighbours_to_prune {
        graph.remove_edge(neighbour, node);
    }
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

fn task_letter_to_duration(letter: &char) -> i32 {
    (*letter as u8 - 4) as i32
}

#[allow(dead_code)]
fn print_graph_dot(graph: &GraphMap<char, i32, Directed>) {
    let dot = Dot::new(graph);
    println!("{:?}", dot);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_input;

    #[test]
    fn check_day07_case01() {
        full_test(
            "Step A must be finished before step B can begin.
            Step B must be finished before step C can begin.
            Step A must be finished before step D can begin.", // INPUT STRING
            "ABCD", // PART 1 RESULT
            "186",  // PART 2 RESULT 61 + 62 + 63
        )
    }

    #[test]
    fn check_day07_case02() {
        full_test(
            "Step X must be finished before step Y can begin.
            Step Y must be finished before step Z can begin.
            Step Y must be finished before step P can begin.", // INPUT STRING
            "XYPZ", // PART 1 RESULT
            "255",  // PART 2 RESULT 84 + 85 + 86
        )
    }

    #[test]
    fn check_letter_value() {
        let val = task_letter_to_duration(&'A');
        assert_eq!(val, 61);
    }

    #[test]
    fn check_graph_building() {
        let input = vec!["Step A must be finished before step B can begin.".to_string()];
        let mut expected: GraphMap<char, i32, Directed> = DiGraphMap::new();
        expected.add_edge('A', 'B', 1);
        let actual = build_graph_from_input(&input);
        assert!(expected.edges('A').eq(actual.edges('A')));
    }

    #[test]
    fn check_find_first() {
        let mut input = dummy_graph();
        let expected = Some('A');
        let actual = find_first_avail_item(&mut input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn check_find_next() {
        let mut input = dummy_graph();
        let expected = Some('C');
        let actual = find_next_item(&mut input, 'A');
        assert_eq!(expected, actual);
    }

    #[test]
    fn check_find_first_from_empty() {
        let mut input: GraphMap<char, i32, Directed> = DiGraphMap::new();
        let expected = None;
        let actual = find_first_avail_item(&mut input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn check_find_next_from_last() {
        let mut input: GraphMap<char, i32, Directed> = DiGraphMap::new();
        input.add_node('A');
        let expected = None;
        let actual = find_next_item(&mut input, 'A');
        assert_eq!(expected, actual);
    }

    fn dummy_graph() -> GraphMap<char, i32, Directed> {
        let mut input: GraphMap<char, i32, Directed> = DiGraphMap::new();
        input.add_edge('A', 'Z', 1);
        input.add_edge('B', 'Z', 1);
        input.add_edge('A', 'C', 1);
        input.add_edge('C', 'D', 1);
        input
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day07(&input_lines, &"a".to_string()),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
