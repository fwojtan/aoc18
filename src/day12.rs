// Potential improvements:
//

pub fn day12(_input_lines: &[Vec<String>], _solution_ver: &String) -> (String, String) {
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
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
            day12(&input_lines, &"a".to_string()),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
