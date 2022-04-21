// Potential improvements:
//

use itertools::zip;

pub fn day05(_input_lines: &[Vec<String>], _solution_ver: &String) -> (String, String) {
    let answer1 = length_of_reacted_polymer(_input_lines.get(0).unwrap().get(0).unwrap());
    let answer2 = modify_and_find_shortest_poly_len(_input_lines.get(0).unwrap().get(0).unwrap());
    (format!("{}", answer1), format!("{}", answer2))
}

fn length_of_reacted_polymer(polymer: &String) -> i32 {
    let mut input_characters = polymer.chars();
    let mut left_char = input_characters.next().unwrap();
    let mut checked_chars: Vec<char> = vec![];
    for right_char in input_characters {
        if check_pair(left_char, right_char) {
            left_char = match checked_chars.pop() {
                Some(val) => val,
                None => right_char,
            }
        } else {
            checked_chars.push(left_char);
            left_char = right_char;
        }
    }
    checked_chars.len() as i32
}

fn check_pair(left_char: char, right_char: char) -> bool {
    if left_char.eq_ignore_ascii_case(&right_char)
        || ((left_char.is_ascii_lowercase() && right_char.is_ascii_uppercase())
            || (right_char.is_ascii_lowercase() && left_char.is_ascii_uppercase()))
    {
        true
    } else {
        false
    }
}

fn modify_and_find_shortest_poly_len(polymer: &String) -> i32 {
    let lowercase = "abcdefghijklmnopqrstuvwxyz".chars();
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let mut lengths: Vec<i32> = vec![];
    for (lower, upper) in zip(lowercase, uppercase) {
        let new_poly = polymer.replace(lower, "").replace(upper, "");
        lengths.push(length_of_reacted_polymer(&new_poly));
    }
    lengths.iter().min().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::day05;
    use crate::utils::load_input;

    #[test]
    fn check_day05_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day05(&input_lines, &"a".to_string()),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
