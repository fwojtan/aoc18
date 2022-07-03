// Potential improvements:
//

use std::{collections::{VecDeque, HashMap}, vec, fmt::Display};

pub fn day09(input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    let mut input = input_lines[0][0].split_ascii_whitespace();
    let n_players = input.nth(0).unwrap().parse::<usize>().unwrap();
    let final_marble = input.nth(5).unwrap().parse::<i32>().unwrap();
    let mut game = Game::new(n_players, final_marble);
    game.play_game();
    let answer1 = game.high_score();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

struct Game {
    players: Vec<Player>,
    marbles: Marbles,
    final_marble: i32,
    next_marble: i32,
}

impl Game {
    fn new(n_players: usize, final_marble: i32) -> Game {
        let players = (0..n_players).map(|idx| Player::new()).collect();
        let marbles = Marbles::new();
        Game{players, marbles, final_marble, next_marble: 0}
    }

    fn play_round(&mut self, player_idx: usize) {
        let player = self.players.get_mut(player_idx).expect("Player index out of range");

        if self.marbles.marble_ring.len() == 0 {
            self.marbles.marble_ring.push(self.next_marble);
            self.next_marble += 1;
        } else if self.next_marble % 23 == 0 {
            // if marble scores do the scoring stuff
            player.score += self.next_marble;

            for _ in 0..7 {self.marbles.move_anti_clockwise()};

            player.score += self.marbles.pop_current();          

            self.next_marble += 1;
        } else {
            // otherwise place the marble as normal

            self.marbles.move_clockwise();
            self.marbles.insert_marble_right(self.next_marble);

            self.next_marble += 1;
        }
    }

    fn play_game(&mut self) {
        for player_idx in (0..self.players.len()).cycle() {
            self.play_round(player_idx);
            if self.next_marble == self.final_marble + 1 {
                break
            }
        }
    }

    fn high_score(&self) -> i32 {
        let highest_player = self.players.iter().max_by(|x,y| x.score.cmp(&y.score)).expect("Couldn't find a high score");
        highest_player.score
    }
}

#[derive(Clone)]
struct Player {
    score: i32,
}

impl Player {
    fn new() -> Player {
        Player{score: 0}
    }
}

struct Marbles {
    marble_ring: Vec<i32>, // list of marble IDs (defined by me, not the marble value)
    current_marble_idx: usize, // index of the 'current marble' in that list
}

impl Marbles {
    /// creates a marble set with a single marble in it
    fn new() -> Marbles {
        Marbles {marble_ring: vec!(), current_marble_idx: 0}
    }

    fn move_clockwise(&mut self) {
        self.current_marble_idx = (self.current_marble_idx + 1) % self.marble_ring.len()
    }

    fn move_anti_clockwise(&mut self) {
        self.current_marble_idx = (self.current_marble_idx - 1) % self.marble_ring.len()
    }

    fn insert_marble_left(&mut self, marble_value: i32) {
        self.marble_ring.insert(self.current_marble_idx, marble_value);
        self.current_marble_idx += 1;
    }

    fn insert_marble_right(&mut self, marble_value: i32) {
        self.marble_ring.insert(self.current_marble_idx+1, marble_value);
    }

    /// removes the current marble, returns its value and sets the next clockwise marble as current
    fn pop_current(&mut self) -> i32 {
        let value = self.marble_ring.remove(self.current_marble_idx);
        self.current_marble_idx %= self.marble_ring.len();
        value
    }
}

impl Display for Marbles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            day09(&input_lines, &"a".to_string()),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
