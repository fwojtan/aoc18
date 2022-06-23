// Potential improvements:
//

// Possible approaches:
//      - sort list by timestamp and iterate through
//      - cycle through unsorted list and add things out of order

// part one - which guard is asleep for the longest (answer is id * duration)
// part two - which guard has the most frequent minut (answer is id * minute or frequency)

// [1518-02-09 23:58] Guard #2267 begins shift
// [1518-04-25 00:07] falls asleep
// [1518-09-10 00:09] falls asleep
// [1518-11-10 00:06] falls asleep
// [1518-11-11 00:15] wakes up

use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime, Timelike};
use counter::Counter;
use regex::Regex;

pub fn day04(_input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    let answer1: i32;
    let answer2: i32;
    match _solution_ver {
        "unsorted" => {
            let mut data = Data::new();
            data.parse_input(_input_lines);
            data.match_shifts_to_guards();
            answer1 = data.part_one();
            answer2 = data.part_two();
        }
        "sorted" => {
            answer1 = 0;
            answer2 = 0;
        }
        _ => {
            answer1 = 0;
            answer2 = 0;
        }
    }
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone, Copy, Debug)]
struct Nap {
    start: u32,
    end: u32,
}

#[derive(Clone, Debug)]
struct Shift {
    nap_start_times: Vec<u32>,
    nap_end_times: Vec<u32>,
    naps: Vec<Nap>,
    id: Option<u32>,
}

#[derive(Debug)]
struct Guard {
    // guard has a set of unique shifts
    naps: Vec<Nap>,
}

#[derive(Debug)]
struct Data {
    guards: HashMap<u32, Guard>,
    // can't identify guards by ID for non-shift start lines
    // so need a map for that maps date to guard ID
    shifts: HashMap<NaiveDate, Shift>,
}

impl Nap {
    fn new(start: u32, end: u32) -> Nap {
        Nap { start, end }
    }

    fn duration(&self) -> u32 {
        // subtracting unsigned ints here... pay attention...
        self.end - self.start
    }

    fn minutes(&self) -> Counter<u32> {
        (self.start..self.end).collect::<Counter<u32>>()
    }
}

impl Shift {
    fn new_from_id(id: u32) -> Shift {
        Shift {
            nap_start_times: vec![],
            nap_end_times: vec![],
            naps: vec![],
            id: Some(id),
        }
    }
    fn new_from_start_time(start_time: u32) -> Shift {
        Shift {
            nap_start_times: vec![start_time],
            nap_end_times: vec![],
            naps: vec![],
            id: None,
        }
    }
    fn new_from_end_time(end_time: u32) -> Shift {
        Shift {
            nap_start_times: vec![],
            nap_end_times: vec![end_time],
            naps: vec![],
            id: None,
        }
    }
    fn push_nap_start(&mut self, start_time: u32) {
        self.nap_start_times.push(start_time);
    }
    fn push_nap_end(&mut self, end_time: u32) {
        self.nap_end_times.push(end_time);
    }
    fn update_id(&mut self, id: u32) {
        self.id = Some(id);
    }
    fn generate_naps(&mut self) {
        self.nap_start_times.sort_unstable();
        self.nap_end_times.sort_unstable();
        for _ in 0..self.nap_start_times.len() {
            self.naps.push(Nap::new(
                self.nap_start_times.pop().unwrap(),
                self.nap_end_times.pop().unwrap(),
            ))
        }
    }
}

impl Guard {
    fn new() -> Guard {
        Guard { naps: vec![] }
    }
    fn add_naps(&mut self, naps: Vec<Nap>) {
        self.naps.extend(naps);
    }
    fn total_nap_duration(&self) -> u32 {
        self.naps.iter().map(|nap| nap.duration()).sum::<u32>()
    }
    fn get_most_frequent_minute(&self) -> (u32, usize) {
        let mut counter: Counter<u32, usize> = Counter::new();
        for nap in &self.naps {
            counter.extend(nap.minutes())
        }
        if let Some(thing) = counter.most_common().get(0) {
            *thing
        } else {
            (0, 0)
        }
    }
}

impl Data {
    fn new() -> Data {
        Data {
            guards: HashMap::new(),
            shifts: HashMap::new(),
        }
    }

    fn parse_input(&mut self, input: &[Vec<String>]) {
        let input_lines = input.get(0).unwrap();
        for line in input_lines {
            self.parse_line(line);
        }
    }

    fn parse_line(&mut self, line: &String) {
        let mut split = line.split(']');
        let timestamp = split.next().unwrap();
        let (date, time) = self.date_and_time_from_timestamp(timestamp);
        if line.ends_with("shift") {
            let message = split.next().unwrap();
            let id = self.id_from_message(message);
            // if a shift already exists, I just need to update the ID
            // otherwise create one using the method
            if let Some(shift) = self.shifts.get_mut(&date) {
                shift.update_id(id);
            } else {
                self.shifts.insert(date, Shift::new_from_id(id));
            }
        } else {
            // I need to create a Shift if one doesn't already exist using the other method
            if let Some(shift) = self.shifts.get_mut(&date) {
                if line.ends_with("asleep") {
                    shift.push_nap_start(time);
                } else if line.ends_with("up") {
                    shift.push_nap_end(time);
                }
            } else if line.ends_with("asleep") {
                self.shifts.insert(date, Shift::new_from_start_time(time));
            } else if line.ends_with("up") {
                self.shifts.insert(date, Shift::new_from_end_time(time));
            }
        }
    }

    fn match_shifts_to_guards(&mut self) {
        for shift in self.shifts.values_mut() {
            shift.generate_naps();
            // I want to use map.try_insert in situations like this but the feature is apparently unstable.. :-(
            if let Some(guard) = self.guards.get_mut(&shift.id.unwrap()) {
                guard.add_naps(shift.naps.to_owned());
            } else {
                let mut guard = Guard::new();
                guard.add_naps(shift.naps.to_owned());
                self.guards.insert(shift.id.unwrap(), guard);
            }
        }
    }

    fn date_and_time_from_timestamp(&self, timestamp: &str) -> (NaiveDate, u32) {
        let datetime = NaiveDateTime::parse_from_str(timestamp, "[%Y-%m-%d %H:%M").unwrap();
        let mut date = datetime.date();
        if datetime.hour() == 23 {
            date = date.succ();
        }
        let minute = datetime.minute();
        (date, minute)
    }

    fn id_from_message(&self, message: &str) -> u32 {
        let re = Regex::new(r" Guard #(\d+) begins shift").unwrap();
        let caps = re.captures(message).unwrap();
        let mut vals = caps
            .iter()
            .skip(1)
            .map(|hit| hit.unwrap().as_str().parse::<u32>().unwrap());
        vals.next().unwrap()
    }

    fn part_one(&mut self) -> i32 {
        let max_nap_guard_and_id = self.guards.iter_mut().max_by(|item, item2| {
            item.1
                .total_nap_duration()
                .cmp(&item2.1.total_nap_duration())
        });
        let longest_guard_id = *max_nap_guard_and_id.as_ref().unwrap().0;
        let most_freq_minute = self
            .guards
            .get(&longest_guard_id)
            .unwrap()
            .get_most_frequent_minute()
            .0;
        (longest_guard_id * most_freq_minute) as i32
    }

    fn part_two(&mut self) -> i32 {
        let most_frequent_guard_and_id = self.guards.iter_mut().max_by(|item, item2| {
            item.1
                .get_most_frequent_minute()
                .1
                .cmp(&item2.1.get_most_frequent_minute().1)
        });
        (most_frequent_guard_and_id.as_ref().unwrap().0
            * most_frequent_guard_and_id
                .unwrap()
                .1
                .get_most_frequent_minute()
                .0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::day04;
    use crate::utils::load_input;

    #[test]
    fn check_day04_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day04(&input_lines, "a"),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
