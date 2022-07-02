// Potential improvements:
// render an animation showing points converging
// making skipping stages more efficient (sample rather 
// than check all particles?)
// use a less expensive distance calculation
// image to text with openCV to get result

use std::{fmt::Display, collections::HashSet, ops::Div};

use num_integer::Roots;
use plotters::{prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, Circle}, style::{WHITE, Color, BLACK, GREEN}};
use regex::Regex;

pub fn day10(input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
    let mut input_particles = parse_input(&input_lines[0]);
    let mut answer1 = "".to_string();
    let mut answer2 = 0;
    find_correct_positions(&mut input_particles, &mut answer1, &mut answer2);
    
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn _new(x:i32, y:i32) -> Coord {
        Coord {x, y}
    }

    // this is basically nonsense code I wrote trying to get better at generics..
    fn new_scale<T>(size:T) -> Coord where i32: From<T> {
        let int:i32 = size.try_into().expect("Failed to convert to i32!");
        Coord{x:int, y:int}
    }

    fn distance(self, other: &Coord) -> i32 {
        let x = (self.x - other.x) as i64;
        let y = (self.y - other.y) as i64;
        let dist = (x*x + y*y).sqrt() as i32;
        // println!("{}", dist);
        dist
    }

    fn above(self) -> Coord {
        Coord{x:self.x,y:self.y+1}
    }

    fn below(self) -> Coord {
        Coord{x:self.x,y:self.y+1}
    }

    fn left(self) -> Coord {
        Coord{x:self.x-1,y:self.y}
    }

    fn right(self) -> Coord {
        Coord{x:self.x+1,y:self.y}
    }
}

impl Div for Coord {
    type Output = Coord;
    fn div(self, rhs: Self) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        Coord{x, y}
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Particle {
    pos: Coord,
    vel: Coord,
}

impl Particle {
    fn new(x_pos:i32, y_pos:i32, x_vel:i32, y_vel:i32) -> Particle {
        Particle { pos: Coord { x:x_pos, y:y_pos }, vel: Coord { x:x_vel, y:y_vel } }
    }

    fn move_one(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }

    fn _move_ten(&mut self) {
        self.pos.x += 10 * self.vel.x;
        self.pos.y += 10 * self.vel.y;
    }

    fn move_hundred(&mut self) {
        self.pos.x += 100 * self.vel.x;
        self.pos.y += 100 * self.vel.y;
    }

    fn move_thousand(&mut self) {
        self.pos.x += 1000 * self.vel.x;
        self.pos.y += 1000 * self.vel.y;
    }

}

impl Display for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos:{}, Vel:{}", self.pos, self.vel)
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Particle> {
    let mut particles: HashSet<Particle> = HashSet::new(); // want to remove duplicates initially
    for line in input {
        let re = Regex::new(r"position=<\s?(-?\d+), \s?(-?\d+)> velocity=<\s?(-?\d+), \s?(-?\d+)>").unwrap();
        let caps = re.captures(line).expect(format!("Failed to unwrap line {}", line).as_str());
        let mut vals = caps
            .iter()
            .skip(1)
            .map(|hit| hit.unwrap().as_str().parse::<i32>().unwrap());
        let x_pos = vals.next().expect("Failed to unwrap x_pos");
        let y_pos = vals.next().expect("Failed to unwrap y_pos");
        let x_vel = vals.next().expect("Failed to unwrap x_vel");
        let y_vel = vals.next().expect("Failed to unwrap y_vel");
        particles.insert(Particle::new(x_pos, y_pos, x_vel, y_vel));
    }
    particles.into_iter().collect()
}

fn plot(particles: &Vec<Particle>) -> Result<(), Box<dyn std::error::Error>> {
    let x_min = particles.clone().iter().min_by(|a,b| a.pos.x.cmp(&b.pos.x)).expect("Couldn't get min x coord").pos.x - 10;
    let x_max = particles.clone().iter().max_by(|a,b| a.pos.x.cmp(&b.pos.x)).expect("Couldn't get max x coord").pos.x + 10;
    let y_min = particles.clone().iter().min_by(|a,b| a.pos.y.cmp(&b.pos.y)).expect("Couldn't get min y coord").pos.y - 10;
    let y_max = particles.clone().iter().max_by(|a,b| a.pos.y.cmp(&b.pos.y)).expect("Couldn't get max y coord").pos.y + 10;
    let root = BitMapBackend::new("images/day10_result.png", (640, 240)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size::<i32>(0)
        .y_label_area_size::<i32>(0)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?; // TODO: Fix these limits automagically
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;
    scatter_ctx.draw_series(
        particles
            .iter()
            .map(|particle| Circle::<(i32, i32), i32>::new((particle.pos.x, particle.pos.y), 5, GREEN.filled())),
    )?;

    scatter_ctx
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn find_average_pos(particles: &Vec<Particle>) -> Coord {
    let mut coord = Coord {x:0, y:0};
    for particle in particles {
        coord.x += particle.pos.x;
        coord.y += particle.pos.y;
    }
    coord / Coord::new_scale(particles.len() as i32)
}

fn in_range_of_target(particles: &mut Vec<Particle>, target: &Coord, range:i32) -> bool {
    let result = particles.iter().all(|particle| particle.pos.distance(&target) < range);
    result
}

fn lineyness(positions: &HashSet<Coord>) -> f32 {
    let mut count = 0;
    for pos in positions.clone().into_iter() {
        if positions.contains(&pos.left()) && positions.contains(&pos.right()) {
            count += 1;
        }
        if positions.contains(&pos.above()) && positions.contains(&pos.below()) {
            count += 1;
        }
    }
    let line_val = (count as f32) / (positions.len() as f32);
    line_val
}

fn in_final_position(particles: &Vec<Particle>) -> bool {

    // find number of overlapping particles
    let mut positions:HashSet<Coord> = HashSet::new();
    let mut overlap_count = 0;
    for particle in particles {
        if !positions.insert(particle.pos) {
            overlap_count += 1;
        }
    }

    if overlap_count >= 5 { // overlap count cheated a little here by using debug output to estimate this value
        if lineyness(&positions) > 0.6 { // but 60% was a genuine guess based on the problem (I did get lucky though XD)
            return true
        }
    }
    false
}

fn find_correct_positions(particles: &mut Vec<Particle>, _part_one: &mut String, part_two: &mut i32) {
    let mut target = find_average_pos(&particles);
    while !in_range_of_target(particles, &target, 10000) {
        particles.iter_mut().for_each(|particle| particle.move_thousand());
        target = find_average_pos(&particles);
        *part_two += 1000;
    }
    while !in_range_of_target(particles, &target, 1000) {
        particles.iter_mut().for_each(|particle| particle.move_hundred());
        target = find_average_pos(&particles);
        *part_two += 100;
    }
    while !in_final_position(&particles) {
        particles.iter_mut().for_each(|particle| particle.move_one());
        *part_two += 1;
    }
    particles.iter_mut().for_each(|particle| particle.pos.y *= -1);
    plot(&particles).unwrap();
}

#[cfg(test)]
mod tests {
    use super::day10;
    use crate::utils::load_input;

    #[test]
    fn check_day10_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day10(&input_lines, "a"),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
