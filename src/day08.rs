// Potential improvements:
//


use std::collections::VecDeque;
use trees::{Tree, tr, TreeWalk};

pub fn day08(input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    let tree = create_tree(&input_lines[0][0]);
    let answer1 = part_one(&tree);
    let answer2 = tree.root().data().value.unwrap();
    (format!("{}", answer1), format!("{}", answer2))
}

fn part_one(tree: &Tree<NodeData>) -> i32 {
    let mut walk: TreeWalk<NodeData> = TreeWalk::from(tree.to_owned());
    let mut metatotal:i32 = walk.get().unwrap().node().data().metadata.as_ref().unwrap().iter().sum();
    while let Some(visit) = walk.next() {
        let new_meta:i32 = match visit {
            trees::walk::Visit::End(_) => 0,
            _ => visit.node().data().get_sum_metadata(),
        };
        metatotal += new_meta;
    }
    metatotal
}


#[derive(Debug, Clone)]
struct NodeData {
    metadata: Option<Vec<i32>>, 
    value: Option<i32>
}

impl NodeData {
    fn new_empty() -> NodeData{
        NodeData{metadata: None, value: None}
    }

    fn update_metadata(&mut self, metadata: Vec<i32>) {
        self.metadata = Some(metadata);
    }

    fn update_value(&mut self, value: i32) {
        self.value = Some(value);
    }

    fn get_sum_metadata(&self) -> i32 {
        self.metadata.as_ref().unwrap().iter().sum()
    }
}

fn create_tree(input: &String) -> Tree<NodeData> {
    let mut input_values: VecDeque<i32> = input.split_ascii_whitespace().map(|val| val.parse::<i32>().unwrap()).collect();
    let mut tree = tr(NodeData::new_empty());
    extract_child(&mut input_values, &mut tree);
    tree
}

fn extract_child(data: &mut VecDeque<i32>, tree: &mut Tree<NodeData>) {
    let num_children = data.pop_front();
    let num_metadata = data.pop_front().expect("Node has no metadata entries");
    if let Some(num_children) = num_children{
        for _ in 0..num_children {
            let mut current_tree: Tree<NodeData> = tr(NodeData::new_empty());
            extract_child(data, &mut current_tree);
            tree.push_back(current_tree);
        }
    }
    // wanted to use spit_off to split the deque but I want data to retain the second half
    let mut metavals = vec!();
    for _ in 0..num_metadata {
        metavals.push(data.pop_front().expect("Failed to unwrap metadata"))
    }
    tree.root_mut().data_mut().update_metadata(metavals);

    if tree.iter().len() == 0 {
        let value = tree.root().data().get_sum_metadata();
        tree.root_mut().data_mut().update_value(value);

    } else {
        let mut value = 0;
        for (i, child) in tree.iter().enumerate() {
            for meta in tree.data().metadata.as_ref().unwrap().iter() {
                if (meta.to_owned() - 1) == i.try_into().unwrap() { // this off by one indexing error made me sad
                    value += child.data().value.unwrap();
                }
            }
        }
        tree.root_mut().data_mut().update_value(value);
    }

}

#[cfg(test)]
mod tests {
    use super::day08;
    use crate::utils::load_input;

    #[test]
    fn check_day08_case01() {
        full_test(
            "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2",  // INPUT STRING
            "138", // PART 1 RESULT
            "66", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day08(&input_lines, &"a".to_string()),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
