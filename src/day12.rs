// Potential improvements:
//

use std::collections::HashSet;

use itertools::{Itertools, zip};
use wildmatch::WildMatch;

pub fn day12(input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    let mut state = input_lines[0][0][15..].chars().map(|char| char_to_digit(char)).collect::<String>();
    let mappings = &input_lines[0][1..].iter()
        .filter(|string| string.ends_with("#"))
        .map(|string| string[0..5].chars()
            .map(|char| char_to_digit(char)).collect::<String>())
        .map(|value_string| pot_value(&value_string))    
        .collect::<HashSet<u8>>();

    println!("state: {}", state);
    println!("mappings: {:?}", mappings);

    let mut start_idx = 0;

    for _ in 0..20 {
        (state, start_idx) = new_gen_state(&state, &mappings, &start_idx);
        println!("{:?}", state);
    }

    let mut total = 0;
    for (pot, contents) in zip(start_idx..start_idx+(state.len() as i32), state.chars()) {
        if contents == '1' {
            total += pot;
        }
    }

    let answer1 = total; // 926 is too low
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn pot_value(pots:&str) -> u8 {
    u8::from_str_radix(pots, 2).unwrap()
}

fn new_gen_state(old_gen_state:&str, mappings: &HashSet<u8>, start_idx: &i32) -> (String, i32) {
    let mut old_state = old_gen_state.to_string();
    let mut new_state = "".to_string();
    let mut start_idx = *start_idx;

    old_state = {
        if WildMatch::new("1*").matches(&old_state) {
            new_state.push_str("0000");
            start_idx -= 4;
            format!("0000{}", old_state)
        } else if WildMatch::new("?1*").matches(&old_state) {
            new_state.push_str("000");
            start_idx -= 3;
            format!("000{}", old_state)
        } else if WildMatch::new("??1*").matches(&old_state) {
            new_state.push_str("00");
            start_idx -= 2;
            format!("00{}", old_state)
        } else if WildMatch::new("???1*").matches(&old_state) {
            new_state.push_str("0");
            start_idx -= 1;
            format!("0{}", old_state)
        } else {
            old_state
        }        
    };

    old_state = {
        if WildMatch::new("*1").matches(&old_state) {
            format!("{}0000", old_state)
        } else if WildMatch::new("*1?").matches(&old_state) {
            format!("{}000", old_state)
        } else if WildMatch::new("*1??").matches(&old_state) {
            format!("{}00", old_state)
        } else if WildMatch::new("*1??").matches(&old_state) {
            format!("{}0", old_state)
        } else {
            old_state
        }
    };

    for (a,b,c,d,e) in old_state.chars().tuple_windows() {
        let value = pot_value(&format!("{}{}{}{}{}", a, b, c, d, e)); // I'm interested to know if there's a better way to go about this..!
        if mappings.contains(&value) {
            new_state.push('1');
        } else {
            new_state.push('0');
        }

    }
    
    if WildMatch::new("*1").matches(&old_state) {
        new_state.push_str("00");
    } else if WildMatch::new("*1?").matches(&old_state) {
        new_state.push('0');
    }

    (new_state, start_idx)
}

fn char_to_digit(input:char) -> char {
    match input {
        '#' => '1',
        '.' => '0',
        _ => panic!("Found bad character")
    }
}



#[cfg(test)]
mod tests {
    use super::day12;
    use crate::utils::load_input;

    #[test]
    fn check_day12_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day12(&input_lines, "a"),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
