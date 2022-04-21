// Potential improvements:
//

use nalgebra;

pub fn day02(_input_lines: &[Vec<String>], _solution_ver: &String) -> (String, String) {
    let answer1: i32;
    let answer2: String;
    match _solution_ver.as_str() {
        "basic" => {
            answer1 = part_one(_input_lines.get(0).unwrap());
            answer2 = part_two_basic(_input_lines.get(0).unwrap());
        }
        "part_one" => {
            answer1 = part_one(_input_lines.get(0).unwrap());
            answer2 = "pass".to_string();
        }
        "swapped" => {
            answer1 = part_one_swapped(_input_lines.get(0).unwrap());
            answer2 = "pass".to_string();
        }
        "early_exit" => {
            answer1 = part_one(_input_lines.get(0).unwrap());
            answer2 = part_two_faster(_input_lines.get(0).unwrap());
        }
        "sorted" => {
            answer1 = part_one(_input_lines.get(0).unwrap());
            answer2 = part_two_sorted(_input_lines.get(0).unwrap());
        }
        "both" => {
            answer1 = part_one(_input_lines.get(0).unwrap());
            answer2 = part_two_both(_input_lines.get(0).unwrap());
        }
        _ => {
            answer1 = 0;
            answer2 = "some answer".to_string();
        }
    }
    (format!("{}", answer1), format!("{}", answer2))
}

fn part_one(input: &Vec<String>) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for item in input {
        let mut matched_two = false;
        let mut matched_three = false;
        for character in item.as_str().chars() {
            if item.matches(character).count() == 2 && !matched_two {
                twos += 1;
                matched_two = true;
            }
            if item.matches(character).count() == 3 && !matched_three {
                threes += 1;
                matched_three = true;
            }
        }
    }
    twos * threes
}

fn part_one_swapped(input: &Vec<String>) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for item in input {
        let mut matched_two = false;
        let mut matched_three = false;
        for character in item.as_str().chars() {
            if !matched_two && item.matches(character).count() == 2 {
                twos += 1;
                matched_two = true;
            }
            if !matched_three && item.matches(character).count() == 3 {
                threes += 1;
                matched_three = true;
            }
        }
    }
    twos * threes
}

fn part_two_basic(input: &Vec<String>) -> String {
    for (index, item) in input.clone().into_iter().enumerate() {
        for second_item in input.into_iter().skip(index + 1) {
            let mut differences = 0;
            let mut pos = 0;
            for (position, (a, b)) in item.clone().chars().zip(second_item.chars()).enumerate() {
                if a != b {
                    differences += 1;
                    pos = position;
                }
            }
            if differences == 1 {
                let mut result = item.clone();
                result.remove(pos);
                return result;
            }
        }
    }
    "Some answer".to_string()
}

fn part_two_faster(input: &Vec<String>) -> String {
    let sorted = input.clone();
    for (index, item) in sorted.clone().into_iter().enumerate() {
        for second_item in sorted.clone().into_iter().skip(index + 1) {
            let mut differences = 0;
            let mut pos = 0;
            for (position, (a, b)) in item.clone().chars().zip(second_item.chars()).enumerate() {
                if a != b {
                    differences += 1;
                    pos = position;
                    if differences > 1 {
                        continue;
                    }
                }
            }
            if differences == 1 {
                let mut result = item.clone();
                result.remove(pos);
                return result;
            }
        }
    }
    "Some answer".to_string()
}

fn part_two_sorted(input: &Vec<String>) -> String {
    let mut sorted = input.clone();
    sorted.sort();
    for (index, item) in sorted.clone().into_iter().enumerate() {
        for second_item in sorted.clone().into_iter().skip(index + 1) {
            let mut differences = 0;
            let mut pos = 0;
            for (position, (a, b)) in item.clone().chars().zip(second_item.chars()).enumerate() {
                if a != b {
                    differences += 1;
                    pos = position;
                }
            }
            if differences == 1 {
                let mut result = item.clone();
                result.remove(pos);
                return result;
            }
        }
    }
    "Some answer".to_string()
}

fn part_two_both(input: &Vec<String>) -> String {
    let mut sorted = input.clone();
    sorted.sort();
    for (index, item) in sorted.clone().into_iter().enumerate() {
        for second_item in sorted.clone().into_iter().skip(index + 1) {
            let mut differences = 0;
            let mut pos = 0;
            for (position, (a, b)) in item.clone().chars().zip(second_item.chars()).enumerate() {
                if a != b {
                    differences += 1;
                    pos = position;
                    if differences > 1 {
                        continue;
                    }
                }
            }
            if differences == 1 {
                let mut result = item.clone();
                result.remove(pos);
                return result;
            }
        }
    }
    "Some answer".to_string()
}

fn _part_two_matrix(input: &Vec<String>) -> String {
    let encoded_values: Vec<u32> = input
        .into_iter()
        .flat_map(|val| {
            val.chars()
                .into_iter()
                .map(|char| char.to_digit(36).unwrap())
        })
        .collect();

    let mat = nalgebra::DMatrix::from_vec(26, input.len(), encoded_values);
    let mat_t = mat.transpose();
    let product = mat_t * mat;

    for row in product.row_iter() {
        let total_matches = row
            .into_iter()
            .map(|value| (*value as f64).sqrt())
            .map(|value| if value.fract() == 0.0 { 1 } else { 0 })
            .sum::<u32>();
        if total_matches == 25 {
            break;
        }
    }

    println!("{:?}", product);

    "matrix_result".to_string()
}

#[cfg(test)]
mod tests {
    use super::day02;
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day02(&input_lines, &"a".to_string()),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
