
use std::collections::HashMap;
use std::fs::read_to_string;
use std::isize;

use utils::{print_map, wait_for_input, DEBUG, TEST};
    
    // What is the sum of the scores of all trailheads on your topographic map?
    // Find hiking trails.
    // Numbers (0=9) represent height
    // Start at 0. End at 9. Increment 1. No diagonals.
    // A trailhead is the starting position of 1 or more trails. 
    // A trailhead score is determined how many 9's are reachable
    

    // Step 1: Find all 9s. 
    // Step 2. For each 9, step by step investigate if any 0 is reachable. 
    

    // Method. Add all acceptable paths to a HashSet. If it doesn't grow, try another way. 
    // Dead-end if all 4 directions are invalid. 
    

    // Method: Go from 9 to 0. 


    mod utils {
        use std::io;

        pub const DEBUG: bool = false;
        pub const TEST: bool = false;

        pub fn print_map(columns: &Vec<Vec<u32>>, current_pos: (usize,usize), override_positions: &Vec<(usize,usize)>) {
            if !DEBUG {
                return
            }
            let map_height = columns.len();
            let map_width = columns[0].len();
    
            for y_i in 0..map_height {
                let mut line:Vec<String> = vec![];
                for x_i in 0..map_width {
                    if match (x_i, y_i) {
                        (_,0) => true,
                        (0,_) => true,
                        (x,y) => if x == (map_width-1) || y == (map_height-1) { true} else {false},
                    } {
                        // continue;
                    }
    
                    
                    let mut val = columns[x_i][y_i].to_string();
    
                    if (x_i, y_i) == current_pos || override_positions.contains(&(x_i, y_i)) {
                        val = "\x1b[31m".to_string() + &val + "\x1b[39m ";
                    }
    
                        let char = if x_i == 0 {
                            format!("{:^3}", val)
                        } else {
                             format!("{:^2}", val)
                        };
                        
                        line.push(char);
                }
                println!("{}", line.join(""));
            }
        }

        pub fn get_diff(start:(isize, isize), columns: &Vec<Vec<u32>>)-> Vec<((usize,usize), isize)> {
            let mut arr = vec![];
            let x_y = vec![(0,1),(0,-1),(1,0),(-1,0)];
            for index in 0..x_y.len() {
    
                let (x, y) = x_y[index];
                    let x = (start.0 as isize + x) as usize;
                    let y= (start.1 as isize + y) as usize;
    
                    // println!(" Start value: {}, xy-value: {},", columns[start.0 as usize][start.1 as usize], columns[x][y]);
                    let start_value =columns[start.0 as usize][start.1 as usize];
                    let xy_value =  columns[x][y];
                    let diff = xy_value as isize - start_value as isize;
                    // println!("{xy_value} - {start_value}");
                    // println!("diff: {diff}");
                    // println!("xy value: {xy_value}");
    
                    if diff == 1 {
                        arr.push(((x,y),diff));
                    }
    
            }
            return arr
        }

        pub fn wait_for_input(show_instruction: bool) {
            if !DEBUG {return} 
            let mut input = String::new();
            if show_instruction {
                println!("Press Enter to continue...");
            }
            io::stdin().read_line(&mut input).expect("Failed to read input");
        }

        pub fn walk_path<'a>(path_array: Vec<(usize,usize)>, start_pos: (isize,isize), columns: &'a Vec<Vec<u32>>, final_array: &'a mut Vec<Vec<(usize, usize)>>)-> &'a mut Vec<Vec<(usize, usize)>> {
            let start_value =columns[start_pos.0 as usize][start_pos.1 as usize];
            if start_value == 9 {                
                // Add entire path.
                let path_array = path_array.clone();
                //////////////////////////////////////////////////////
                //  PART 1.
                // Only add path if it's not a duplicate. 
                // let mut duplicate_found = false;
                // for i in 0..final_array.len() {
                //     let first_last = (final_array[i].first().unwrap(), final_array[i].last().unwrap());
                //     let current_first_last = (path_array.first().unwrap(), path_array.last().unwrap());
                //     if first_last == current_first_last {
                //         duplicate_found = true;
                //     }
                // }
                // if !duplicate_found {
                //     final_array.push(path_array);
                // }
                //////////////////////////////////////////////////////
                final_array.push(path_array);
                return final_array;
            }
    
            let diff = get_diff(start_pos, &columns);
             if diff.len() > 0 {
                for i in 0..diff.len(){
                    let mut path_array = path_array.clone();
                    let new_start_pos = (diff[i].0.0 ,diff[i].0.1).clone();
                    path_array.push(new_start_pos);
                    let new_start_pos = diff[i].0.clone();
                     walk_path(path_array,(new_start_pos.0 as isize, new_start_pos.1 as isize),&columns, final_array);
                }
                
                return final_array;
    
    
            } else {
                if DEBUG {
                    println!("No path forward at {},{}",start_pos.0,start_pos.1 );
                }
                
                return final_array
            }
    
        }

    }

fn main() {   
    let file_path = if TEST {"./test_input.txt"} else {"./input.txt"};
    println!("In file {file_path}");
    let mut columns: Vec<Vec<u32>> = Vec::new();
    let mut starting_positions = Vec::new();

    
    for (row_index, line) in read_to_string(file_path).unwrap().lines().enumerate() {
        let mut row = vec![];
        row.push(99); // Padding
        for (column_index, char) in line.chars().enumerate() {
            let digit = char.to_digit(10).unwrap();
            if digit == 0 {
                starting_positions.push((column_index+1,row_index+1));
            }
            row.push(digit);
        }
        row.push(99); // Padding
        if row_index == 0 {
            // Create 1 vec per column, i.e 1 per line_len
            for _ in 0..(row.len()) {
                columns.push(vec![]);
            }
        }


    
        for column_index in 0..row.len() {
            // Rows and columns are created, but empty. Add the numbers.
            columns[column_index].push(row[column_index])
        }
    }


    for i in 0..columns.len() {
        columns[i].push(99);
        columns[i].insert(0, 99);
    }


    if DEBUG {
        println!("Starting positons: {:?}", starting_positions);
        wait_for_input(true);
    }


    let columns: Vec<Vec<u32>> = columns.clone();
    let mut final_paths:Vec<Vec<(usize, usize)>> = vec![];
    let mut start_position_scores: HashMap<&(usize, usize), usize> = HashMap::new();
    for  start in starting_positions.iter() {

        let path_array = vec![*start];
        let mut final_array: Vec<Vec<(usize, usize)>> = vec![];
        let arr: &mut Vec<Vec<(usize, usize)>> = utils::walk_path(path_array, (start.0 as isize, start.1 as isize), &columns, &mut final_array);
        for i in 0..arr.len(){
            if arr[i].len() == 10 {
                if DEBUG {
                    println!("Adding to final_paths: {:?}", arr[i]);
                }

                let start_pos_score = start_position_scores.get(start);


                match start_pos_score {
                    Some(score) => {
                        start_position_scores.insert(start, score+1);
                    },
                    None => {
                        start_position_scores.insert(start, 1);
                    }
                }
                
                final_paths.push(arr[i].clone());
            }

        }

        if DEBUG {
            println!("final_paths len: {}",final_paths.len());
            println!("Start pos: {},{}", start.0, start.1);
        }
        // Save final result for the start position
        
        wait_for_input(true);
    }
        if DEBUG {
            println!("final_paths: {:?}",final_paths);
        }
        final_paths.sort_by(|a, b| {
            let first = a[0].0 + a[0].1;
            let second = b[0].0 + b[0].1;
            first.cmp(&second)
        });

        for arr in final_paths
        {
            // println!("Arr #{} len: {:?}",i+1, arr.len());
            // final_paths.push(arr.clone());

            
            
            if DEBUG {
                
                let mut override_positions = vec![];
            for pos_index in 0..arr.len(){

                // print_map_animate(&columns,arr[pos_index],'X',None);
                // Add arrays to final_paths. 
                println!("Start pos: ({},{}). Arr {}/{}",arr[0].0,arr[0].1, pos_index+1, arr.len());
                print_map(&columns, (arr[pos_index].0,arr[pos_index].1), &override_positions);
                utils::wait_for_input(true);
                override_positions.push((arr[pos_index].0,arr[pos_index].1))
            }
        }

        }


        let mut points = 0;
        println!("Start position scores: {:?}", start_position_scores );
        for (_, score ) in start_position_scores {
            points += score;

        }

        println!("Final points: {points}" )


        // If we get here, there was no match. Keep going from our new start positions. 
    }
    
    

    // 35 wrong.
    // 1649 too low
    // 4819 too high
    // 1575 is someone elses input??
    // 379 is wrong 
    // 1796
