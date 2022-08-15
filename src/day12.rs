// Potential improvements:
//

pub fn day12(input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    let initial_state = &input_lines[0];
    let mappings = &input_lines[1];

    println!("state: {:?}", initial_state);
    println!("mappings: {:?}", mappings);


    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn pot_value(pots:&str) -> u8 {
    u8::from_str_radix(pots, 2).unwrap()
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
