// Potential improvements:
// Vec turns out to be borderline impossible
// HashMap is much better but is stil quite slow
// Doubly linked list with a cursor sounds best but rust only has these in nightly..

// Rough runtimes:
// Vec solves in something in the order of a few hours
// HashMap solves in 2.5 to 3 seconds
// LinkedList solves in 250 to 300 milliseconds
use std::{collections::{HashMap, LinkedList, linked_list::CursorMut}, vec, fmt::Display};

pub fn day09(input_lines: &[Vec<String>], solution_ver: &str) -> (String, String) {
    let mut input = input_lines[0][0].split_ascii_whitespace();
    let n_players = input.next().unwrap().parse::<usize>().unwrap();
    let final_marble = input.nth(5).unwrap().parse::<i32>().unwrap();
    match solution_ver {
        "vec" => {
            let mut game = Game::new(n_players, final_marble, VecMarbles::new());
            game.play_game();
            let answer1 = game.high_score();
            game.final_marble = final_marble * 100;
            game.play_game();
            let answer2 = game.high_score();
            (format!("{}", answer1), format!("{}", answer2))
        },
        "hash" => {
            let mut game = Game::new(n_players, final_marble, HashMarbles::new());
            game.play_game();
            let answer1 = game.high_score();
            let new_final = final_marble * 100;
            let mut game = Game::new(n_players, new_final, HashMarbles::new());
            game.play_game();
            let answer2 = game.high_score();
            (format!("{}", answer1), format!("{}", answer2))
        },
        "llist" => {
            let mut list:LinkedList<i32> = LinkedList::new();
            let marbles = LListMarbles::new_from_list(&mut list);
            let mut game = Game::new(n_players, final_marble, marbles);
            game.play_game();
            let answer1 = game.high_score();
            let new_final = final_marble * 100;
            let mut list:LinkedList<i32> = LinkedList::new();
            let marbles = LListMarbles::new_from_list(&mut list);
            let mut game = Game::new(n_players, new_final, marbles);
            game.play_game();
            let answer2 = game.high_score();
            (format!("{}", answer1), format!("{}", answer2))
        }
        _ => panic!("Didn't reconginise solution version")
    }
}

struct Game<T> {
    players: Vec<Player>,
    marbles: T,
    final_marble: i32,
    next_marble: i32,
}

impl<T> Game<T>  where T:MarbleCollection+Display {
    fn new(n_players: usize, final_marble: i32, marble_collection: T) -> Game<T> {
        let players = (0..n_players).map(|_| Player::new()).collect();
        let marbles = marble_collection;
        Game{players, marbles, final_marble, next_marble: 1}
    }

    fn play_round(&mut self, player_idx: usize) {
        let player = self.players.get_mut(player_idx).expect("Player index out of range");

        if self.next_marble % 23 == 0 {
            // if marble scores do the scoring stuff
            player.score += self.next_marble as u64;
            player.score += self.marbles.pop_seventh_and_set_sixth() as u64;          
        } else {
            // otherwise place the marble as normal
            self.marbles.move_clockwise_and_place_new(self.next_marble);
        }
        self.next_marble += 1;
    }

    fn play_game(&mut self) {
        for player_idx in (0..self.players.len()).cycle() {
            // println!("Processing round {}", self.next_marble);
            self.play_round(player_idx);
            if self.next_marble == self.final_marble + 1 {
                break
            }
            // println!("[{}] {}", player_idx, self.marbles);
        }
    }

    fn high_score(&self) -> u64 {
        let highest_player = self.players.iter().max_by(|x,y| x.score.cmp(&y.score)).expect("Couldn't find a high score");
        highest_player.score
    }
}

#[derive(Clone)]
struct Player {
    score: u64,
}

impl Player {
    fn new() -> Player {
        Player{score: 0}
    }
}

trait MarbleCollection {
    /// place a marble between the 1st and 2nd clockwise marbles from the current
    /// marble and sets that marble as current
    fn move_clockwise_and_place_new(&mut self, marble_value: i32);

    /// removes the seventh count-clockwise marble and sets the sixth counter-clockwise
    /// marble as the current marble
    fn pop_seventh_and_set_sixth(&mut self) -> i32;
}

struct VecMarbles {
    marble_ring: Vec<i32>, // list of marble IDs (defined by me, not the marble value)
    current_marble_idx: usize, // index of the 'current marble' in that list
}

impl VecMarbles {
    /// creates a marble set with a single marble in it
    fn new() -> VecMarbles {
        VecMarbles {marble_ring: vec!(0), current_marble_idx: 0}
    }

    fn move_seven_anti_clockwise(&mut self) {
        if self.current_marble_idx < 7 {
            self.current_marble_idx = self.marble_ring.len() + self.current_marble_idx - 7;
        } else {
            self.current_marble_idx -= 7;
        }
    }

    /// removes the current marble, returns its value and sets the next clockwise marble as current
    fn pop_current(&mut self) -> i32 {
        let value = self.marble_ring.remove(self.current_marble_idx);
        self.current_marble_idx %= self.marble_ring.len();
        value
    }
}

impl MarbleCollection for VecMarbles {

    fn move_clockwise_and_place_new(&mut self, marble_value: i32) {
        self.current_marble_idx = ((self.current_marble_idx + 1) % self.marble_ring.len()) + 1;
        self.marble_ring.insert(self.current_marble_idx, marble_value);
    }

    fn pop_seventh_and_set_sixth(&mut self) -> i32 {
        self.move_seven_anti_clockwise();
        self.pop_current()
    }
}

impl Display for VecMarbles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "".to_string();
        for (i, marble) in self.marble_ring.iter().enumerate() {
            if i == self.current_marble_idx {
                result += format!(" ({})", marble).as_str();
            } else {
                result += format!(" {}", marble).as_str();
            }
        }
        write!(f, "{}", result)
    }
}

struct HMarbleMetadata {
    left_marble: i32,
    right_marble: i32,
}

impl HMarbleMetadata {
    fn new(left:i32, right:i32) -> HMarbleMetadata {
        HMarbleMetadata { left_marble: left, right_marble: right}
    }
}

struct HashMarbles {
    marble_ring: HashMap<i32, HMarbleMetadata>,
    current_marble: i32,
}

impl HashMarbles {
    fn new() -> HashMarbles {
        let mut marble_ring = HashMap::new();
        let meta = HMarbleMetadata::new(0, 0);
        marble_ring.insert(0, meta);
        HashMarbles{marble_ring, current_marble:0}
    }

    fn get_nth_left_of_current(&self, n: usize) -> i32 {
        let mut result = self.current_marble;
        for _ in 0..n {
            result = self.marble_ring.get(&result).expect("Left marble was not in map").left_marble;
        }
        result
    }

}

impl MarbleCollection for HashMarbles {
    fn move_clockwise_and_place_new(&mut self, marble_value: i32) {
        let one_over = self.marble_ring.get(&self.current_marble).expect("Current was not in map").right_marble;
        let two_over = self.marble_ring.get(&one_over).expect("Right marble was not in map").right_marble;

        let new_meta = HMarbleMetadata::new(one_over, two_over);
        self.marble_ring.insert(marble_value, new_meta);

        self.marble_ring.get_mut(&one_over).expect("Right marble was not in map").right_marble = marble_value;
        self.marble_ring.get_mut(&two_over).expect("Two over marble was not in map").left_marble = marble_value;
        
        self.current_marble = marble_value;
    }

    fn pop_seventh_and_set_sixth(&mut self) -> i32 {
        let sixth_left = self.get_nth_left_of_current(6);
        let seventh_left = self.marble_ring.get(&sixth_left).expect("7th left marble was not in map").left_marble;
        let eigth_left = self.marble_ring.get(&seventh_left).expect("8th left marble was not in map").left_marble;

        self.marble_ring.remove(&seventh_left);

        self.marble_ring.get_mut(&sixth_left).expect("Sixth left was not in map").left_marble = eigth_left;
        self.marble_ring.get_mut(&eigth_left).expect("Eigth left marble was not in map").right_marble = sixth_left;

        self.current_marble = sixth_left;
        seventh_left
    }
}

impl Display for HashMarbles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "".to_string();
        let first = self.marble_ring.iter().next().unwrap();
        let mut current = first;
        loop {
            if current.0 == first.0 {
                break
            } else if *current.0 == self.current_marble {
                result += format!(" ({})", current.0).as_str();
            } else {
                result += format!(" {}", current.0).as_str();
            }
            current = (&current.1.right_marble, self.marble_ring.get(&current.1.right_marble).unwrap());
        }
        write!(f, "{}", result)
    }
}
struct LListMarbles<'a> {
    // list: LinkedList<i32>,
    cursor: CursorMut<'a, i32>,
}

impl<'a> LListMarbles<'a> {
    fn new_from_list(list: &'a mut LinkedList<i32>) -> LListMarbles<'a> {
        list.push_back(0);
        LListMarbles { cursor:list.cursor_back_mut() }
    }

    fn move_n_anti_clockwise(&mut self, n: usize) {
        for _ in 0..n {
            self.cursor.move_prev();
            if self.cursor.current().is_none() {
                self.cursor.move_prev();
            }
        }
    }

}

impl<'a> MarbleCollection for LListMarbles<'a> {
    fn move_clockwise_and_place_new(&mut self, marble_value: i32) {
        self.cursor.move_next();
        if self.cursor.current().is_none() {
            self.cursor.move_next();
        }
        self.cursor.insert_after(marble_value);
        self.cursor.move_next();
    }

    fn pop_seventh_and_set_sixth(&mut self) -> i32 {
        self.move_n_anti_clockwise(7);
        self.cursor.remove_current().expect("Cursor removed None..!")
    }
}

impl <'a> Display for LListMarbles<'a> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // I think it's impossible to display this given that cursor is mutable
        // and that's the only thing I'm storing in the struct
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::day09;
    use crate::utils::load_input;

    #[test]
    fn check_day09_case00() {
        full_test(
            
            "9 players; last marble is worth 25 points",  // INPUT STRING
            "32", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }
    #[test]
    fn check_day09_case01() {
        full_test(
            "10 players; last marble is worth 1618 points",  // INPUT STRING
            "8317", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }
    #[test]
    fn check_day09_case02() {
        full_test(
            "13 players; last marble is worth 7999 points",  // INPUT STRING
            "146373", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }
    #[test]
    fn check_day09_case03() {
        full_test(
            "17 players; last marble is worth 1104 points",  // INPUT STRING
            "2764", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }
    #[test]
    fn check_day09_case04() {
        full_test(
            "21 players; last marble is worth 6111 points",  // INPUT STRING
            "54718", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }
    #[test]
    fn check_day09_case05() {
        full_test(
            "30 players; last marble is worth 5807 points",  // INPUT STRING
            "37305", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }



    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day09(&input_lines, "llist"),

            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
