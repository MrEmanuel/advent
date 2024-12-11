
use std::collections::HashSet;
use std::{fs::read_to_string, ops::Index};
use std::isize;

use utils::{print_map, DEBUG, TEST};
    
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
        pub const TEST: bool = true;

        pub fn print_map(columns: &Vec<Vec<u32>>, current_pos: (usize,usize)) {
            if !DEBUG {
                return
            }
            let map_height = columns.len();
            let map_width = columns[0].len();
    
            println!("{},{}", map_height, map_width);
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
    
                    if (x_i, y_i) == current_pos {
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
                    // println!("-----");
                    // println!("(x,y) = ({x},{y})");
                    // println!("(1,2) = ({},{})",start.0, start.1);
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
                println!("9 Found!");
                
                // Add entire path.
                let path_array = path_array.clone();
                final_array.push(path_array);
                return final_array;
            }
    
            let diff = get_diff(start_pos, &columns);
            println!("{}", diff.len());
                    
         
    
            if diff.len() == 1 {
                // let x = diff[0].0.0;
                // let y = diff[0].0.1;
                // 1. Add (x,y) to path. 
                // 2. If length increases, keep going.
                // println!("One path forward at {x},{y}");
                let new_start_pos = (diff[0].0.0 as isize, diff[0].0.1 as isize).clone(); 
                let mut path_array = path_array.clone();
                path_array.push((diff[0].0.0 ,diff[0].0.1));
                // println!("path array is: {:?}",path_array);
                return walk_path(path_array,new_start_pos,&columns, final_array)
    
                // path.insert((x,y));
            } else if diff.len() > 1 {
                println!("Multiple paths forward!");

                // let final_arr_multiple = vec![];
                
                for i in 0..diff.len(){
                    let mut path_array = path_array.clone();
                    let new_pos = (diff[i].0.0 ,diff[i].0.1).clone();
                    println!("Multi match new pos: ({},{})", new_pos.0, new_pos.1);
                    path_array.push(new_pos);
                    let new_start_pos = diff[i].0.clone();
                    walk_path(path_array,(new_start_pos.0 as isize, new_start_pos.1 as isize),&columns, final_array);

                    // TODO: Figure out how to match arrays.. 
                }
                
                return final_array;
    
    
            } else {
                println!("No path forward at {},{}",start_pos.0,start_pos.1 );
                
                return final_array
            }
    
        }

    }

fn main() {
   
    let pause_on_each_frame = false;
    let viewport_width = if DEBUG {20} else {150}; // Width of the visible grid
    let viewport_height = if DEBUG {10} else {130}; // Height of the visible grid
    

  

    let print_map_animate =|
        columns: &Vec<Vec<u32>>,
        starting_pos: (usize, usize),
        starting_string: char,
        extra_obstacle_pos: Option<Vec<(usize, usize)>>
     | {
        let map_height = columns.first().iter().len();
        let map_width = columns.first().iter().last().iter().len();
         let (sx, sy) = starting_pos;
            // Calculate the top-left corner of the viewport
            let start_x = if sx > viewport_width / 2 {
                sx - viewport_width / 2
            } else {
                0
            };
            let start_y = if sy > viewport_height / 2 {
                sy - viewport_height / 2
            } else {
                0
            };
    
            // Clip the viewport to the grid boundaries
            let end_x = (start_x + viewport_width).min(map_width);
            let end_y = (start_y + viewport_height).min(map_height);
    
            // Clear the map
            print!("\x1B[2J\x1B[1;1H");
            let mut line_to_print = vec![];
            for y_i in start_y..end_y {
                let mut line: Vec<String> = vec![];
                for x_i in start_x..end_x {
                    // If the position matches the cursor, display the override character
                     let s_char = starting_string;
                        if (x_i, y_i) == (sx, sy) {
                            let start_char = format!("{}{}{}","\x1b[31m", s_char.to_string(), "\x1b[39m");
                            line.push(start_char);
                            continue;
                        }
                    
                    if extra_obstacle_pos.as_ref().is_some_and(|obst_pos| obst_pos.contains(&(x_i,y_i)))  {
                        line.push("\x1b[31m#\x1b[39m".to_string());
                        continue;
                    }
                    // Default to the grid character
                    line.push(columns[x_i][y_i].to_string());
                }
                // Print the new line
                line_to_print.push( line.join(""));
                // line_to_print.push("\n".to_string());
                
            }
            
                print!("{}", line_to_print.join("\n"));
            
        
        if pause_on_each_frame {
            utils::wait_for_input(false);
        }
    };


    
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
    }


    let columns: Vec<Vec<u32>> = columns.clone();
    let mut final_paths :Vec<Vec<(usize, usize)>> = vec![];
    let mut start_pos_scores: Vec<((usize,usize), usize)>= vec![];
    for  start in starting_positions.iter() {


        if DEBUG {
            print_map(&columns, *start);
            utils::wait_for_input(true);
        }

        let path_array = vec![];
        let mut final_array: Vec<Vec<(usize, usize)>> = vec![];
        let arr = utils::walk_path(path_array, (start.0 as isize, start.1 as isize), &columns, &mut final_array);
        let mut set = HashSet::new();
        for i in 0..arr.len(){
            if arr[i].len() == 9 {
                set.insert(arr[i].clone());
            }

        }

        // Save final result for the start position
        start_pos_scores.push((start.clone(),set.len()));

        for arr in set.iter() {
            // println!("Arr #{} len: {:?}",i+1, arr.len());
            final_paths.push(arr.clone());
            
            if DEBUG {
            for pos_index in 0..arr.len(){
                // print_map_animate(&columns,arr[pos_index],'X',None);
                // Add arrays to final_paths. 
                print_map(&columns, (arr[pos_index].0,arr[pos_index].1));
                utils::wait_for_input(true);
            }
        }

        }


        println!("Start pos: {},{}", start.0, start.1);


        }

        println!("Final paths: {:?}", final_paths);

        let mut points = 0;
        for i in 0..start_pos_scores.len() {
            points += start_pos_scores[i].1;

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
