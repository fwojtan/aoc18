// Potential improvements:
//

use counter::Counter;
use std::collections::HashMap;

use regex::Regex;

pub fn day06(input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    let points = PointsList::new_from_puzzle_input(&input_lines[0]);
    let answer1 = count_largest_area(&points);
    let answer2 = count_included_distance(&points);
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn manhattan_distance(self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone)]
struct PointsList {
    list: Vec<Point>,
}

impl PointsList {
    fn new_from_puzzle_input(input: &Vec<String>) -> PointsList {
        let mut coords: Vec<Point> = vec![];
        for coord in input {
            let re = Regex::new(r"(\d+), (\d+)").unwrap();
            let caps = re.captures(coord).unwrap();
            let mut vals = caps
                .iter()
                .skip(1)
                .map(|hit| hit.unwrap().as_str().parse::<i32>().unwrap());
            let x = vals.next().unwrap();
            let y = vals.next().unwrap();
            coords.push(Point::new(x, y));
        }
        PointsList { list: coords }
    }

    fn x_min(&self) -> i32 {
        self.list
            .clone()
            .into_iter()
            .min_by_key(|point| point.x)
            .unwrap()
            .x
    }

    fn y_min(&self) -> i32 {
        self.list
            .clone()
            .into_iter()
            .min_by_key(|point| point.y)
            .unwrap()
            .y
    }

    fn x_max(&self) -> i32 {
        self.list
            .clone()
            .into_iter()
            .max_by_key(|point| point.x)
            .unwrap()
            .x
    }

    fn y_max(&self) -> i32 {
        self.list
            .clone()
            .into_iter()
            .max_by_key(|point| point.y)
            .unwrap()
            .y
    }

    fn get_closest_point(&self, point: Point) -> Option<Point> {
        let mut closest: Option<Point> = None;
        let mut distance = 1000000000;
        for coord in self.list.clone() {
            let dist = coord.manhattan_distance(&point);
            if dist < distance {
                closest = Some(coord);
                distance = dist;
            } else if dist == distance {
                closest = None;
            }
        }
        closest
    }

    fn get_total_distance(&self, point: Point) -> i32 {
        self.list
            .clone()
            .into_iter()
            .map(|coord| coord.manhattan_distance(&point))
            .sum()
    }
}

fn count_largest_area(points: &PointsList) -> i32 {
    let mut counter: Counter<Point, i32> = Counter::new();
    for x in points.x_min()..points.x_max() {
        for y in points.y_min()..points.y_max() {
            if let Some(point) = points.get_closest_point(Point::new(x, y)) {
                counter[&point] += 1;
            }
        }
    }
    counter.most_common()[0].1
}

fn count_included_distance(points: &PointsList) -> i32 {
    let mut counter: i32 = 0;
    for x in points.x_min()..points.x_max() {
        for y in points.y_min()..points.y_max() {
            if points.get_total_distance(Point::new(x, y)) < 10000 {
                counter += 1;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day06(&input_lines, &"a".to_string()),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
