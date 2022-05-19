// Potential improvements:
//

use std::collections::{HashMap, HashSet};

use itertools::iproduct;
use regex::{self, Regex};

pub fn day03(_input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    test_function();

    // let answer1 = part_one(_input_lines.get(0).unwrap());
    // let answer2 = part_two(_input_lines.get(0).unwrap());
    let (answer1, answer2) = combined(_input_lines.get(0).unwrap());
    (format!("{}", answer1), format!("{}", answer2))
}

fn test_function() {
    // let _matrix = nalgebra::SMatrix::<i32, 1000, 1000>::zeros();
    //let matrix = nalgebra::SMatrix::<i32, 1000, 1000>::zeros();
}

fn parse_input_line(line: &String) -> (i32, i32, i32, i32, i32) {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let caps = re.captures(line.as_str()).unwrap();
    let mut vals = caps
        .iter()
        .skip(1)
        .map(|hit| hit.unwrap().as_str().parse::<i32>().unwrap());
    let result: (i32, i32, i32, i32, i32) = (
        vals.next().unwrap(),
        vals.next().unwrap(),
        vals.next().unwrap(),
        vals.next().unwrap(),
        vals.next().unwrap(),
    );
    result
}

fn part_one(input: &Vec<String>) -> i32 {
    // let _total = nalgebra::SMatrix::<i32, 1000, 1000>::zeros();
    let mut map: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    for line in input {
        let (_id, x_off, y_off, x_len, y_len) = parse_input_line(line);

        for (x, y) in iproduct!(x_off + 1..x_off + x_len + 1, y_off + 1..y_off + y_len + 1) {
            if let Some(inner_map) = map.get_mut(&x) {
                if let Some(value) = inner_map.get(&y) {
                    inner_map.insert(y, value + 1);
                } else {
                    inner_map.insert(y, 1);
                }
            } else {
                let mut inner_map: HashMap<i32, i32> = HashMap::new();
                inner_map.insert(y, 1);
                map.insert(x, inner_map);
            }
        }
    }
    map.into_values()
        .map(|inner_map| inner_map.into_values().filter(|val| *val > 1).count())
        .sum::<usize>() as i32
}

fn patch_outline(
    x_off: i32,
    y_off: i32,
    x_len: i32,
    y_len: i32,
) -> impl Iterator<Item = (i32, i32)> {
    let top = (x_off + 1..x_off + x_len + 1).map(move |x| (x, y_off + 1));
    let bottom = top.clone().map(move |coord| (coord.0, y_off + y_len + 1));
    let left = (y_off + 2..y_off + y_len).map(move |y| (x_off + 1, y));
    let right = left.clone().map(move |coord| (x_off + x_len + 1, coord.1));
    let xs = top.chain(bottom);
    let ys = left.chain(right);
    xs.chain(ys)
}

fn part_two(input: &Vec<String>) -> i32 {
    // for speed should move this to be in the same loop as part one (parsing input is expensive!)
    let mut map2: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut untouched_ids: HashSet<i32> = HashSet::new();
    for line in input {
        let (id, x_off, y_off, x_len, y_len) = parse_input_line(line);
        let mut overlapped = false;
        for (x, y) in iproduct!(x_off + 1..x_off + x_len + 1, y_off + 1..y_off + y_len + 1) {
            //patch_outline(x_off, y_off, x_len, y_len) {
            if let Some(inner_map) = map2.get_mut(&x) {
                if let Some(value) = inner_map.get(&y) {
                    overlapped = true;
                    untouched_ids.remove(&value);
                } else {
                    inner_map.insert(y, id);
                }
            } else {
                let mut inner_map: HashMap<i32, i32> = HashMap::new();
                inner_map.insert(y, id);
                map2.insert(x, inner_map);
            }
        }
        if !overlapped {
            untouched_ids.insert(id);
        }
    }
    *untouched_ids.iter().nth(0).unwrap()
}

fn combined(input: &Vec<String>) -> (i32, i32) {
    let mut map: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut map2: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut untouched_ids: HashSet<i32> = HashSet::new();
    for line in input {
        let (id, x_off, y_off, x_len, y_len) = parse_input_line(line);
        let mut overlapped = false;
        for (x, y) in iproduct!(x_off + 1..x_off + x_len + 1, y_off + 1..y_off + y_len + 1) {
            if let Some(inner_map) = map2.get_mut(&x) {
                if let Some(value) = inner_map.get(&y) {
                    overlapped = true;
                    untouched_ids.remove(&value);
                } else {
                    inner_map.insert(y, id);
                }
            } else {
                let mut inner_map: HashMap<i32, i32> = HashMap::new();
                inner_map.insert(y, id);
                map2.insert(x, inner_map);
            }
            if let Some(inner_map) = map.get_mut(&x) {
                if let Some(value) = inner_map.get(&y) {
                    inner_map.insert(y, value + 1);
                } else {
                    inner_map.insert(y, 1);
                }
            } else {
                let mut inner_map: HashMap<i32, i32> = HashMap::new();
                inner_map.insert(y, 1);
                map.insert(x, inner_map);
            }
        }
        if !overlapped {
            untouched_ids.insert(id);
        }
    }
    let part_one = map
        .into_values()
        .map(|inner_map| inner_map.into_values().filter(|val| *val > 1).count())
        .sum::<usize>() as i32;
    let part_two = *untouched_ids.iter().nth(0).unwrap();
    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    fn check_day03_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day03(&input_lines, &"a".to_string()),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
