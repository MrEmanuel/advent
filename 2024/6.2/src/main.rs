use std::{collections::HashSet, fs::read_to_string, ops::Index};
use std::{io, thread, time};

fn main() {
    let debug = false;
    let test = false;
    let pause_on_each_frame = false;
    let ticker_speed = 10;
    let viewport_width = if debug {50} else {150}; // Width of the visible grid
    let viewport_height = if debug {30} else {130}; // Height of the visible grid
    fn get_direction(string: &char) -> (i32,i32) {
        return match string {
            'v' => (0,1),
            '^' => (0,-1),
            '<' => (-1,0),
            '>' => (1,0),
            _ => unreachable!("Unexpected value: {}", string),  
        };

    }
    fn change_direction(string: &char) -> char {
        return match string {
            'v' => '<',
            '^' => '>',
            '<' => '^',
            '>' => 'v',
            _ => unreachable!("Unexpected value: {}", string),  
        };

    }

    fn next_is_outside_map(starting_pos: (usize,usize), starting_string: char,map_last_x_index: usize ,map_last_y_index: usize ) ->  bool {
        let match_val = match (starting_pos, starting_string) {
                ((_, 0), '^') => true,
                ((_,index), 'v') if index + 1 >= map_last_y_index =>true,
                ((index,_), '>') if index + 1 >= map_last_x_index =>true,
                ((0, _), '<') => true,
                _ => false,
            };

            // println!("Match val: {match_val}");
            return match_val
    }

    fn pos_is_obstacle(columns: &Vec<Vec<char>>, next_position: (usize, usize))-> bool {

        let next_char = *columns.index(next_position.0).index(next_position.1);
        return  next_char == '#'
    }

    
    let print_map = |columns: &Vec<Vec<char>>, map_height: usize, map_width: usize, override_pos: Option<(usize,usize)>, override_string: Option<char>| {
        if !debug {
            return
        }
        for y_i in 0..map_height {
            let mut line:Vec<String> = vec![];
            for x_i in 0..map_width {
                let val = columns.index(x_i).index(y_i).to_string();
                if override_pos.is_some() && override_string.is_some() && override_pos.unwrap() == (x_i, y_i) {
                    line.push(override_string.unwrap().to_string());

                } else {
                    line.push(val);
                }
            }
            println!("{}", line.join(""));
        }
    };

    let wait_for_input = |show_instruction: bool| {
        if !debug {return} 
        let mut input = String::new();
        if show_instruction {
            println!("Press Enter to continue...");
        }
        io::stdin().read_line(&mut input).expect("Failed to read input");
    };

    let print_map_animate =|
        columns: &Vec<Vec<char>>,
        map_height: usize,
        map_width: usize,
        starting_pos: Option<(usize, usize)>,
        starting_string: Option<char>,
        extra_obstacle_pos: Option<Vec<(usize, usize)>>
     | {
        if let Some((sx, sy)) = starting_pos {
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
                    if let Some(s_char) = starting_string {
                        if (x_i, y_i) == (sx, sy) {
                            let start_char = format!("{}{}{}","\x1b[31m", s_char.to_string(), "\x1b[39m");
                            line.push(start_char);
                            continue;
                        }
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
            
        }
        if pause_on_each_frame {
            wait_for_input(false);
        }
    };


    
    // Guard position is indicated by either "v","^",">","<"
    // Obstructions are indicated by #
    // If something is in front of the guard, he will turn right 90 degrees.
    // If nothing is in front, take one step forward. (in the new direction)
    //

    // "v" means to move (0, 1)
    // "^" means to move (0, -1)
    // "<" means to move (-1, 0)
    // ">" means to move (1, 0)

    // Count the unique positions the guard will visit before leaving the map.

    // Second part. 
    // 1. For each new position after start, evalulate the position and simulate a turn
    //      1.1 Check if any upcoming position matches the avaluated direction and position.
    //      1.2 If map is out of bounds, simulation failed
    //      1.3 If upcoming position matches position and direction, increment count.
    let file_path = if test {"./test_input.txt"} else {"./input.txt"};
    println!("In file {file_path}");
    let mut columns: Vec<Vec<char>> = Vec::new();
    let mut map_width = 0;
    let mut original_starting_pos = (0,0);
    let mut starting_pos: (usize, usize) = (0,0);
    for (row_index, line) in read_to_string(file_path).unwrap().lines().enumerate() {
        // Collect the entire map in to multiple columns.
        
        if row_index == 0 {
            map_width = line.len();
            // Create 1 vec per column, i.e 1 per line_len
            for _ in 0..line.len() {
                columns.push(vec![]);
            }
            println!("Map is {} wide and {} high.", columns.len(), columns.first().unwrap().len())
        }
        for column_index in 0..map_width {
            let character = line.chars().nth(column_index).unwrap();
            
            let is_starting_pos = match character {
                '<' => true,
                '>' => true,
                'v' => true,
                '^' => true,
                _ => false,  
            };
            if is_starting_pos {
                println!("Found starting pos: {column_index},{row_index}");
                starting_pos = (column_index,row_index);
                original_starting_pos = (column_index, row_index)
            }
            columns[column_index].push(character);
        }
    }
    let map_height = columns.first().unwrap().len();
    print_map(&columns,map_height,map_width, None, None);

    // We have the starting position and the map now. 
    // Step forward based on the direction until hitting obstacle.
    let mut starting_string: char = *columns.index(starting_pos.0).index(starting_pos.1);
    let original_starting_string = starting_string.clone();
    // println!("Starting string: {starting_string}");



    let map_last_x_index = map_width;
    let map_last_y_index = map_height;
    // println!("Bounds: {map_last_x_index}, {map_last_y_index}");

    let mut unique_positions_visited = HashSet::new();
    let mut positions_visited: Vec<((usize,usize), char)> = vec![];
    let mut positions_visited_twice = HashSet::new();
    let mut while_count = 0;
    let mut prev_moved = false;
    loop {
        let direction = get_direction(&starting_string);
        while_count +=1;
        if unique_positions_visited.contains(&starting_pos) && prev_moved {
            // Positions visited twice.
            positions_visited_twice.insert(starting_pos);
        }
        let last_visited_pos = positions_visited.last();
        if last_visited_pos.is_none() {
            positions_visited.push((starting_pos.clone(),starting_string.clone()));
        } else { // TODO: This check is not needed when we skip starting pos in next loop?
            if last_visited_pos.unwrap().0 != starting_pos {
                positions_visited.push((starting_pos.clone(),starting_string.clone()));
            }
        }
        
        unique_positions_visited.insert(starting_pos.clone()); // Add starting position

        if next_is_outside_map(starting_pos, starting_string, map_last_x_index, map_last_y_index) {
            // println!("Last position before leaving map is: {:?}", starting_pos);
            break
        } 

        // Check for obstacles. 
        let next_position = (
            (starting_pos.0 as i32 + direction.0).max(0) as usize,
            (starting_pos.1 as i32 + direction.1).max(0) as usize,
        );
        if pos_is_obstacle(&columns, next_position) {
            // println!("Next is obstacle is ahead current pos: {:?} and direction: {starting_string}", starting_pos);
            let new_direction = change_direction(&starting_string);
            // println!("New direction should be: {}", new_direction);
            starting_string = new_direction;
            prev_moved = false;
        } else {
            // Next is free. Update starting position.
            // println!("Step {starting_string} to {:?}", next_position);
            prev_moved = true;
            starting_pos = next_position;
        }


        if while_count > 10000 {
            println!("While break");
            break
        }
    }

    println!("Count: {}", unique_positions_visited.len());
    // println!("Positions visited twice: {:?}", positions_visited_twice);

    println!("original starting pos: {:?}",original_starting_pos);
    println!("=========================");
    println!("=========================");

    // For each visited position, simulate what happens if you turn right.

    let columns = columns;
    let mut loop_found_in_simulation_no = vec![];
    let mut obstacle_positions = HashSet::new();
    for  (index,(turn_here, turn_here_direction)) in positions_visited.iter().enumerate() {

            if turn_here == &original_starting_pos {
                continue
            }

            let mut starting_pos = original_starting_pos.clone();
            let mut starting_string = original_starting_string;
            let mut turn_right_positions = HashSet::new();
            let mut prev_pos = (999,999);
            let extra_obstacle_pos = (
                (turn_here.0 as i32).max(0) as usize,
                (turn_here.1 as i32).max(0) as usize,
            );

        let iterations = 10000;
        for i in 0..=iterations {
            let direction = get_direction(&starting_string);
            // println!("direction: {:?}", direction);
            if debug {
                if index < 16 {
                    continue
                }
                print_map_animate(&columns, map_height, map_width, Some(starting_pos), Some(starting_string), Some(vec![extra_obstacle_pos]));
                thread::sleep(time::Duration::from_millis(ticker_speed));
            }
            if next_is_outside_map(starting_pos, starting_string, map_last_x_index, map_last_y_index) {
                if debug {
                println!("\nLeaving map");
                wait_for_input(true);
            }
                break
            }
            
     
            let next_position = (
                (starting_pos.0 as i32 + direction.0).max(0) as usize,
                (starting_pos.1 as i32 + direction.1).max(0) as usize,
            );

            let should_turn_right= {
                let res = pos_is_obstacle(&columns, next_position);
                // println!("{:?} Is obstacle: {res}", {next_position});
                if res || next_position == *turn_here {
                    true
                } else {
                    false
                }
            };

            if should_turn_right {
                if debug {
                    println!("\nShould turn right..");
                    wait_for_input(true);
                }

                let first_right_turn_count = turn_right_positions.len();
                turn_right_positions.insert((starting_pos, starting_string));
                let second_right_turn_count = turn_right_positions.len();

                if debug {
                    println!("{:?}",{turn_right_positions.clone()});
                }
                wait_for_input(true);

                // Skip if last iteration was a turn.
                let loop_found = first_right_turn_count > 0 && first_right_turn_count == second_right_turn_count && starting_pos != prev_pos;
                prev_pos = starting_pos;
                
                if loop_found {
                    obstacle_positions.insert(extra_obstacle_pos);
                    loop_found_in_simulation_no.push(index);
                    if debug { 
                        // Print added obstacle
                        print_map_animate(&columns, map_height, map_width, Some(starting_pos), Some(starting_string), Some(vec![extra_obstacle_pos]));
                        println!("Loop found! {} in total. simulation index {index}", loop_found_in_simulation_no.len());
                        wait_for_input(true);
                    }
                    
                    wait_for_input(true);
                    break

                }


                // println!("Next is obstacle is ahead current pos: {:?} and direction: {starting_string}", starting_pos);
                let new_direction = change_direction(&starting_string);
                // println!("New direction should be: {}", new_direction);
                starting_string = new_direction;
                // prev_moved = false;
            } else {
                // Next is free. Update starting position.
                // println!("Step {starting_string} to {:?}", next_position);
                // prev_moved = true;
                starting_pos = next_position;
            }
            

            // println!("======= Ending pos: {:?}", starting_pos);

            if i == iterations {

                if debug {
                    println!("/////// Breaking on {i} iterations! for turn_here: {:?}", turn_here)
                }
                break
            }
        }


    } 

    print_map_animate(&columns, map_height, map_width, Some(original_starting_pos), Some(original_starting_string), Some(obstacle_positions.iter().map(|val| *val).collect()));
    if debug {
        
        println!("Loops found in indexes: {:?}",loop_found_in_simulation_no);
        println!("{:?}", obstacle_positions);
    }
    println!("Obstacles count: {}", obstacle_positions.len());
    

    // 35 wrong.
    // 1649 too low
    // 4819 too high
    // 1575 is someone elses input??
    // 379 is wrong 
    // 1796

}
