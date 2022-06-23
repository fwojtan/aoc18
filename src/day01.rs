// Potential improvements:
//
use std::collections::HashSet;

pub fn day01(_input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    let answer1 = _input_lines
        .get(0)
        .unwrap()
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>();
    let answer2 = part2(_input_lines.get(0).unwrap());
    (format!("{}", answer1), format!("{}", answer2))
}

fn part2(input: &Vec<String>) -> i32 {
    let mut seen: HashSet<i32> = HashSet::new(); //Vec<i32> = vec!();
    let values: Vec<i32> = input
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let mut freq: i32 = 0;
    let mut outer_loop_counter = 0;

    while !seen.contains(&freq) {
        for value in &values {
            seen.insert(freq);
            //println!("loop {}: {} (+ {}) becomes {}... have seen {} values", outer_loop_counter, freq, value, freq+value, seen.len());
            freq += value;
            if seen.contains(&freq) {
                //println!("We have seen {} twice!", freq);
                break;
            }
        }
        outer_loop_counter += 1;
    }
    println!("Went round {} full times", outer_loop_counter);
    freq
}

// I thought I had a bug here where I somehow was overflowing a value at
// relatively small numbers. Turns out there are just some stupidly big
// numbers in my puzzle input

// And as alex says - vec is disgustingly slow. HashSet is waaaay faster

#[cfg(test)]
mod tests {
    use super::day01;
    use crate::utils::load_input;

    #[test]
    fn check_day01_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day01(&input_lines, "a"),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
