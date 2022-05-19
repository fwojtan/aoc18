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
// have a 'next task' ready to go. Only remove tasks from the graph once complete, only
// find the next task when a worker picks a task up

// this approach for part two may be completely broken because we can end up with the same task twice??

// maybe it's better to prune all incoming edges when a task is first assigned..
// ooh also this definitely won't work because tasks will block on the previously asigned task

// this is a silly solution, but could use edge weight to distinguish between blocking and non-blocking dependency
// that unblocked it but this algo I've been using for part two is fundamentally busted
// what if a task finishes and unblocks a task, but that task never gets picked up because it's not most recently assigned

// I think the algo needs to be ->
// assign as many available tasks as poss
// when task is complete delete it's node
// repeat
// that seems way simpler..?


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
    let mut last_item = find_first_item(&mut graph);
    loop {
        let next_item = find_next_item(&mut graph, last_item, false);
        graph.remove_node(last_item);
        items.push(last_item);
        if let Some(next_item) = next_item {
            last_item = next_item;
        } else {
            break
        }
    }

    items.iter().collect::<String>()
}

fn part_two(input: &Vec<String>) -> i32 {
    let mut graph = build_graph_from_input(&input);

    // track the task letter and the end-time of a task
    let mut active_tasks: Vec<Option<(char, i32)>> = vec!(None; 5);

    let mut most_recent_task = find_first_item(&mut graph);
    active_tasks[0] = Some((most_recent_task, task_letter_to_duration(&most_recent_task)));
    
    let mut time: i32 = 0;
    // each iteration represents one second
    loop {

        println!("{}, {:?}", time, active_tasks);
        println!("most recent: {}, next: {:?}", most_recent_task, find_next_item(&mut graph, most_recent_task, true));
        let n_neighbours: Vec<char> = graph
        .neighbors_directed('B', Outgoing)
        .filter(|node| graph.neighbors_directed(*node, Incoming).count() == 1)
        .collect();
        println!("{:?}", n_neighbours);
        // pickup tasks for all available workers (check if None)
        for task in &mut active_tasks {
            if task.is_none() {
                // work out next task (if one exists)
                if let Some(new_task) = find_next_item(&mut graph, most_recent_task, true) {
                    most_recent_task = new_task;

                    // calculate task end-time and add to active tasks
                    task.replace((new_task, time + task_letter_to_duration(&new_task)));
                    
                    // removing incoming edges from the task's node so it can't be selected again
                    prune_incoming_edges(&mut graph, new_task)
                }
            }
        }

        if active_tasks.iter().all(|task| task.is_none()) {
            break
        }

        // check if this is final second of task for any workers
        for i in 0..active_tasks.len() {
            if let Some(task_details) = active_tasks[i] {
                if task_details.1 == time {
                    active_tasks[i] = None;
                }
            }
        }

        time += 1;
    }

    // print_graph_dot(&graph);

    time
}

fn find_next_item(graph: &mut GraphMap<char, i32, Directed>, previous_item: char, part_two: bool) -> Option<char> {
    let mut item_set: Vec<char>;
    if !part_two {
        // find all dependent nodes
        // for each dependent node, if it has more than one dependency, it's not available
        item_set = graph
            .neighbors_directed(previous_item, Outgoing)
            .filter(|node| graph.neighbors_directed(*node, Incoming).count() == 1)
            .collect();
    } else {
        // for part two we don't want to exclude items from the item set if their dependencies are the 'fake' ones I added
        item_set = graph
            .neighbors_directed(previous_item, Outgoing)
            .filter(|node| graph.neighbors_directed(*node, Incoming)
                .filter(|neighbour| graph.edge_weight(*neighbour, *node).unwrap().to_owned() == 1)
                .count() == 1)
            .collect();
    }

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
        graph.add_edge(first_item, node, 0);
    }

    first_item
}

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
