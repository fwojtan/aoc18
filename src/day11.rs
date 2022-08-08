// Potential improvements:
//

use std::{thread, sync::{mpsc, Arc, Mutex}, fmt::Display, collections::BTreeMap};

use rayon::prelude::*;
use threadpool::ThreadPool;

pub fn day11(input_lines: &[Vec<String>], solution_ver: &str) -> (String, String) {
    let grid_sn:i32 = input_lines[0][0].parse().unwrap();
    let grid:Vec<Vec<i32>> = (1..301).map(|x| {
        let row = (1..301).map(|y| power_level(x, y, grid_sn)).collect();
        row
    }).collect();
    let mut answer1 = (0,0);
    let mut answer2 =  (0,0,0);
    if solution_ver == "concurrent" {
        answer1 = largest_3x3(&grid);
        answer2 = largest_nxn(&grid);
    } else {
        answer1 = largest_3x3(&grid);
        answer2 = largest_nxn_serial(&grid)
    }
    
    (format!("{},{}", answer1.0, answer1.1), format!("{},{},{}", answer2.0, answer2.1, answer2.2))
}

fn power_level(x:i32, y:i32, grid_sn:i32) -> i32{
    // let out = if (x==90+16 || y==269+16) {true} else {false};
    let rack_id = x+10;
    // if out {println!("rack id:{}", rack_id);}
    let mut pl = rack_id * y;
    // if out {println!("pl pre sn:{}, sn:{}", pl,grid_sn);}
    pl += grid_sn;
    // if out {println!("pl pre mult: {}", pl);}
    pl *= rack_id;
    // if out {println!("pl pre 100s char:{}", pl);}
    pl = hundreds_digit(pl);
    // if out {println!("pl plus 5:{}", pl);}
    pl -= 5;
    // if out {println!("pl:{}", pl);}
    pl
}

fn hundreds_digit(pl:i32) -> i32 {
    pl.to_string().chars().rev().nth(2).unwrap_or_else(|| {println!("Couldn't unwrap, set to 0");'0'}).to_digit(10).unwrap() as i32
}

fn largest_3x3(grid: &Vec<Vec<i32>>) -> (i32, i32) {
    let arc_grid = Arc::new(grid);
    let mut max_power = Mutex::new((0, 0, 0)); // 0th is power, 1st is x, 2nd is y (should go write a struct really..)
    rayon::scope(|s|
        for (x_idx, col) in grid.iter().enumerate() {
            let arc_grid = *arc_grid;
            let max_power = &max_power;
            s.spawn(move|s| {
            for (y_idx, _) in col.iter().enumerate() {
                if x_idx < 297 && y_idx < 297 {
                    let mut local_power = 0;
                    for i in 0..3 {
                        for j in 0..3 {
                            local_power += arc_grid[x_idx + i][y_idx + j]
                        }
                    }
                    let mut max_power = max_power.lock().expect("Failed to get max power mutex for comparison");
                    if local_power > max_power.0 {
                        *max_power = (local_power, (x_idx+1) as i32, (y_idx+1) as i32);
                    }
                }
                }           
            });
        }
    );
    let max = *max_power.lock().expect("Failed to get max power mutex to report result");

    (max.1, max.2)
}

fn largest_nxn(grid: &Vec<Vec<i32>>) -> (i32, i32,i32) {
    let arc_grid = Arc::new(grid);
    let mut max_power = Mutex::new((0, 0, 0, 0)); // 0th is power, 1st is x, 2nd is y (should go write a struct really..), 3rd is size
    rayon::scope(|s|
        for (x_idx, col) in grid.iter().enumerate() {
            let arc_grid = *arc_grid;
            let max_power = &max_power;
            s.spawn(move|s| {
            for (y_idx, _) in col.iter().enumerate() {            
                let mut max_local_power = 0;
                let mut max_n = 0;
                find_max_power_with_size(&mut max_local_power, &mut max_n, arc_grid, x_idx, y_idx);

                
                let mut max_power = max_power.lock().expect("Failed to get max power mutex for comparison");
                if max_local_power > max_power.0 {
                    *max_power = (max_local_power, (x_idx+1) as i32, (y_idx+1) as i32, max_n);
                    // println!("Updating max_value {}, {}, {}", max_local_power, (x_idx+1) as i32, (y_idx+1) as i32);
                }
            }    
            // println!("Max power: {:?}", *max_power.lock().expect("Failed to get max power mutex to report result")); 
        });
    });
    
    let max = *max_power.lock().expect("Failed to get max power mutex to report result");
    // println!("max power:{}", max.0);
    (max.1, max.2, max.3 as i32)
}

fn add_to_grid(running_total: &mut i32, arc_grid: &Vec<Vec<i32>>, x_idx: usize, y_idx: usize, n: usize) {
    // if n==16 && y_idx+1==269 && x_idx+1 == 90 {
        // println!("Corner: {},", arc_grid[x_idx+n][y_idx+n]);
        // println!("Vertical:");
    // }
    *running_total += arc_grid[x_idx+n][y_idx+n]; // corner
    for y_idx2 in y_idx..(y_idx+n-1) {
        // if n==16 && y_idx+1==269 && x_idx+1 == 90 {
            // print!("{},", arc_grid[x_idx+n][y_idx2]);
        // }
        *running_total += arc_grid[x_idx+n][y_idx2]; // vertical strip
    }
    // if n==16 && y_idx+1==269 && x_idx+1 == 90 {
        // println!("\nHorizontal:");
    // }
    for x_idx2 in x_idx..(x_idx+n-1) {
        // if n==16 && y_idx+1==269 && x_idx+1 == 90 {
        //     print!("{},", arc_grid[x_idx2][y_idx+n]);
        // }
        *running_total += arc_grid[x_idx2][y_idx+n]; // horizontal strip
    }
    // if n==16 && y_idx+1==269 && x_idx+1 == 90 {
    // println!("");
    // }
    // if x_idx+1 == 90 && (y_idx+1 == 269 || y_idx+1 == 265){
    //     println!("x:{},y:{}calculating {}x{} grid, total is now {}", x_idx+1, y_idx+1, n, n, running_total);
    // }
}

fn find_max_power_with_size(max_local_power: &mut i32, max_n: &mut i32, arc_grid: &Vec<Vec<i32>>, x_idx: usize, y_idx: usize) {
    let mut running_total = 0;
    for n in 1..301 {
        if x_idx + n < 300 && y_idx + n < 300 {
            add_to_grid(&mut running_total, arc_grid, x_idx, y_idx, n)
        }
        if running_total > *max_local_power {
            // println!("running total updated {}", running_total);
            *max_local_power = running_total;
            *max_n = n as i32;
        }
    }
}

fn largest_nxn_serial(grid: &Vec<Vec<i32>>) -> (i32, i32,i32) {
    let arc_grid = Arc::new(grid);
    let mut max_power = (0, 0, 0, 0); // 0th is power, 1st is x, 2nd is y (should go write a struct really..), 3rd is size
        for (x_idx, col) in grid.iter().enumerate() {
            let grid = *arc_grid;
            for (y_idx, _) in col.iter().enumerate() {            
                let mut max_local_power = 0;
                let mut max_n = 0;
                find_max_power_with_size(&mut max_local_power, &mut max_n, grid, x_idx, y_idx);
                if max_local_power > max_power.0 {
                    max_power = (max_local_power, (x_idx+1) as i32, (y_idx+1) as i32, max_n);
                    // println!("Updating max_value {}, {}, {}", max_local_power, (x_idx+1) as i32, (y_idx+1) as i32);
                }
            }    
            // println!("Max power: {:?}", *max_power.lock().expect("Failed to get max power mutex to report result")); 
        }
    // println!("max power:{}", max.0);
    (max_power.1, max_power.2, max_power.3 as i32)
}

// Everything below this point is a bad, messy, broken implementation...
// some useful examples of concurrency though..

// pub fn day11(input_lines: &[Vec<String>], _solution_ver: &str) -> (String, String) {
//     let grid_sn:i32 = input_lines[0][0].parse().unwrap();
//     let mut grid: Vec<Vec<GridPoint>> = vec![];
//     // setup initial grid. This is a column-major 2D array 300x300
//     for x in 1..301 {
//         grid.push(vec![GridPoint::new(x); 300])
//     }

//     // note to self - probably better to just use a parallel iterator here
//     // that iterates over all points in the grid and initialises them

//     // setup a channel to recieve results
//     let (tx, rx) = mpsc::channel();
//     let mut handles = vec!();
//     let mut x = 1;
//     for col in grid {
//         // clone common things that need to be transferred to thread
//         let tx1 = tx.clone();
//         let grid_sn1 = grid_sn.clone();
//         let handle = thread::spawn(move || {
//             let res = set_power_level_for_col(col, grid_sn1);
//             tx1.send((x, res))
//         });
//         handles.push(handle);
//         x += 1;
//     }

//     // block on all threads having finished
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // extract results from the channel we were sending on
//     // (question: does this have a limited buffer..?)
//     let mut updated_grid = BTreeMap::new(); // probs don't need to do this, could really just index into existing grid
//     for _ in 0..300 {
//         let res = rx.recv().expect("Failed to read channel");
//         updated_grid.insert(res.0, res.1);
//     }

//     // debug output..

//     // println!("{:?}", updated_grid.keys());

//     // for col in updated_grid {
//     //     let out: Vec<i32> = col.1.iter().map(|point| point.power_level.unwrap()).collect();
//     //     println!("{:?}", out);
//     // }
//     let (tx, rx) = mpsc::channel();
//     let arc_grid = Arc::new(updated_grid);
//     let pool = ThreadPool::new(8);
//     for x in 1..301 {
//         for y in 1..301 {
//             let arc_grid = arc_grid.clone();
//             let tx = tx.clone();
//             pool.execute(move|| {
//                 // do stuff
//                 let mut values = vec!();
//                 for i in 1..3{
//                     for j in 1..3 {
//                         // at the moment I just let these threads panic if they don't fit in the array
//                         let value = &arc_grid.get(&(x+i)).unwrap()[(y-1)+j].power_level;
//                         values.push(value)
//                     }
//                 }
//                 let total_power: i32 = values.iter().map(|val| val.unwrap()).sum();
//                 tx.send((x, y, total_power)).unwrap();

//                 // println!("x:{}, y:{}, power:{}", x, y, total_power);
//             })
//         }
//     }

//     let max_value = rx.iter().take(298*298).max_by(|respA, respB| respA.2.cmp(&respB.2)).unwrap(); 

//     // this answer is currently wrong....
//     let answer1 = format!("({},{})", max_value.0, max_value.1);
//     let answer2 = 0;
//     (format!("{}", answer1), format!("{}", answer2))
// }

// #[derive(Debug, Clone)]
// struct GridPoint {
//     rack_id: i32,
//     power_level: Option<i32>,
//     power_zone_level: Option<i32>,
// }

// impl GridPoint {
//     fn new(x:i32) -> GridPoint{
//         GridPoint{rack_id:x+10, power_level:None, power_zone_level:None}
//     }

//     fn set_power_level(&mut self, y:i32, grid_sn:i32) {
//         let mut pl = self.rack_id * y;
//         pl += grid_sn;
//         pl *= self.rack_id;
//         pl = pl.to_string().chars().rev().nth(2).unwrap_or('0').to_digit(10).unwrap() as i32;
//         pl -= 5;
//         self.power_level = Some(pl);
//     }
// }

// impl Display for GridPoint {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.power_level.unwrap_or(-10000))
//     }
// }

// fn set_power_level_for_col(col: Vec<GridPoint>, grid_sn: i32) -> Vec<GridPoint>{
//     let mut new_col = vec!();
//     for (y, item) in col.iter().enumerate() {
//         let mut item1 = item.clone();
//         item1.set_power_level((y+1) as i32, grid_sn);
//         new_col.push(item1);
//     }
//     new_col
// }

#[cfg(test)]
mod tests {
    use super::{day11, add_to_grid};
    use crate::{utils::load_input, day11::find_max_power_with_size, day11::{power_level, hundreds_digit}};

    #[test]
    fn check_day11_case01() {
        full_test(
            "18",  // INPUT STRING
            "33,45", // PART 1 RESULT
            "90,269,16", // PART 2 RESULT
        )
    }
    #[test]
    fn check_day11_case02() {
        full_test(
            "42",  // INPUT STRING
            "21,61", // PART 1 RESULT
            "232,251,12", // PART 2 RESULT
        )
    }
    #[test]
    fn day11_check_grid_addition0() {
        let grid = vec!(vec!(1;20);20);
        let mut total = 0;
        add_to_grid(&mut total, &grid, 0, 0, 1);
        assert_eq!(total, 1);
    }
    #[test]
    fn day11_check_grid_addition1() {
        let grid = vec!(vec!(1;20);20);
        let mut total = 1;
        add_to_grid(&mut total, &grid, 0, 0, 2);
        assert_eq!(total, 4);
    }
    #[test]
    fn day11_check_grid_addition2() {
        let grid = vec!(vec!(1;20);20);
        let mut total = 4;
        add_to_grid(&mut total, &grid, 0, 0, 3);
        assert_eq!(total, 9);
    }

    #[test]
    fn day11_check_max_for_coord1() {
        let mut grid = vec!(vec!(1;300);300);
        grid[5][5] = 10;
        grid[6][6] = -1000000;

        let mut power = 4;
        let mut n = 0;
        find_max_power_with_size(&mut power, &mut n, &grid, 0, 0);
        assert_eq!(power, 34);
        assert_eq!(n, 5)
    }

    #[test]
    fn day11_check_max_for_coord2() {
        let mut grid = vec!(vec!(1;300);300);
        grid[299][299] = -10000000;

        let mut power = 4;
        let mut n = 0;
        find_max_power_with_size(&mut power, &mut n, &grid, 200, 150);
        assert_eq!(power, 10000 - 199);
        assert_eq!(n, 99)
    }

    #[test]
    fn day11_check_power_level1() {
        let pl = power_level(122,79,57);
        assert_eq!(pl, -5);
    }
    #[test]
    fn day11_check_power_level2() {
        let pl = power_level(217,196,39);
        assert_eq!(pl, 0);
    }
    #[test]
    fn day11_check_power_level3() {
        let pl = power_level(101,153,71);
        assert_eq!(pl, 4);
    }

    #[test]
    fn day11_check_hundreds_digit_char1() {
        let digit = hundreds_digit(1234567890);
        assert_eq!(digit,8);
    }
    #[test]
    fn day11_check_hundreds_digit_char2() {
        let digit = hundreds_digit(2020);
        assert_eq!(digit,0);
    }
    #[test]
    fn day11_check_hundreds_digit_char3() {
        let digit = hundreds_digit(10);
        assert_eq!(digit,0);
    }

    #[test]
    fn day11_check_hundreds_digit_char4() {
        let digit = hundreds_digit(4444444);
        assert_eq!(digit,4);
    }
    #[test]
    fn day11_check_power_level4() {
        let pl = power_level(90+16,269+16,18);
        assert_eq!(pl, -5);// should this really be -5?
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day11(&input_lines, "a"),
            (part1_result.to_string(), part2_result.to_string())
        );
    }


}
