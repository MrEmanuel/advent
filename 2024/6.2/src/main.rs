use std::{collections::HashSet, fs::read_to_string, ops::Index};
use std::{io, thread, time};

fn main() {
    let test = true;
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

    
    fn print_map(columns: &Vec<Vec<char>>, map_height: usize, map_width: usize, override_pos: Option<(usize,usize)>, override_string: Option<char>) {
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
    }

    fn pause_for_input() {
        let mut input = String::new();
        println!("Press Enter to continue...");
        io::stdin().read_line(&mut input).expect("Failed to read input");
    }

    fn print_map_animate(
        columns: &Vec<Vec<char>>,
        map_height: usize,
        map_width: usize,
        starting_pos: Option<(usize, usize)>,
        starting_string: Option<char>,
    ) {
        let viewport_width = 50; // Width of the visible grid
        let viewport_height = 50; // Height of the visible grid
    
        // Clear the screen and reset the cursor
        print!("\x1B[2J\x1B[1;1H");
    
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
    
            for y_i in start_y..end_y {
                let mut line: Vec<String> = vec![];
                for x_i in start_x..end_x {
                    // If the position matches the cursor, display the override character
                    if let Some(s_char) = starting_string {
                        if (x_i, y_i) == (sx, sy) {
                            line.push(s_char.to_string());
                            continue;
                        }
                    }
                    // Default to the grid character
                    line.push(columns[x_i][y_i].to_string());
                }
                // Print the line
                println!("{}", line.join(""));
            }
        }
    }


    
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
    let mut positions_visited: Vec<(usize,usize)> = vec![];
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
        // if last_visited_pos.is_some() != &starting_pos {
        //     positions_visited.push(starting_pos.clone());

        // }
        if last_visited_pos.is_none() {
            positions_visited.push(starting_pos.clone());
        } else {
            if last_visited_pos.unwrap() != &starting_pos {
                positions_visited.push(starting_pos.clone());
            }
        }
        
        unique_positions_visited.insert(starting_pos.clone()); // Add starting position

        // let next_is_outside_map = match (starting_pos, starting_string) {
        //     ((_, 0), '^') => true,
        //     ((_,index), 'v') if index == map_last_y_index =>true,
        //     ((_,index), '>') if index == map_last_x_index =>true,
        //     ((0, _), '<') => true,
        //     _ => false,
        // };


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
    let mut loop_count = 0;
    // let debug_pos: (usize, usize) = (67, 38); //(68, 38)  (103, 40)  (54, 82)  (80, 22) (75, 12)
    // let index_to_check = positions_visited.iter().position(|pos| *pos == debug_pos).unwrap();
    let mut obstacle_positions = HashSet::new();
    for  (index,turn_here) in positions_visited.iter().enumerate() {

        // if index != index_to_check { continue  } // Debug
        // if index == 0 {
            // Reset start values for a new "turn_here" value
            let mut starting_pos = original_starting_pos.clone();
            let mut starting_string = original_starting_string;
            let mut is_simulating = false;
            let mut turn_right_positions = vec![];
            // let mut turn_right_next = false;
            // TODO: Figure out what to do on first spot. Skip or not?
            // continue
        // }
        // println!("Simulating for turn_here position: {:?}", turn_here);
        // Redo the same thing as in loop above.
      
        let iterations = 10000;
        for i in 0..=iterations {
            let direction = get_direction(&starting_string);
            // println!("direction: {:?}", direction);
            print_map_animate(&columns, map_height, map_width, Some(starting_pos), Some(starting_string));
            thread::sleep(time::Duration::from_millis(1000));
            if next_is_outside_map(starting_pos, starting_string, map_last_x_index, map_last_y_index) {
                // println!("Last position before leaving map is: {:?}", starting_pos);
                // println!(" ");
                // println!(" ");
                break
            }
            
            // Check for obstacles. 
      
            // println!("next position: {:?}, {starting_string}", next_position);
            // Check if next_position matches turn_here .
            // If it does, check if is_simulating
            
     
            let next_position = (
                (starting_pos.0 as i32 + direction.0).max(0) as usize,
                (starting_pos.1 as i32 + direction.1).max(0) as usize,
            );

            let mut should_turn_right= {
                let res = pos_is_obstacle(&columns, next_position);
                // println!("{:?} Is obstacle: {res}", {next_position});
                if res {
                    true
                } else {
                    false
                }
            };
            // turn_right_next = false; // Reset saved state from last iteration on each iteration.
            
            if next_position == *turn_here {
                pause_for_input();
                // println!("Match {:?} == {:?}",next_position, turn_here);
                if is_simulating {
                    loop_count +=1;
                    obstacle_positions.insert(next_position);
                    break
                    // println!("Loop found at {:?}", turn_here);
                    // Loop found! 
                    // println!("Count +1, breaking at {i}");
                    // print_map(&columns, map_height, map_width, Some(*turn_here), Some(starting_string));
                    // println!("======= Loop found at ending pos: {:?}, {starting_string}", starting_pos);
                    
                } else {
                    // println!("Simulation on! Turn right, true!");
                    is_simulating = true;
                     should_turn_right = true;
                    // turn_right_next = true;
                    

                }
            }

            if  should_turn_right {
                turn_right_positions.push(starting_pos);

                // Check if the position was repeated the 3rd latest position in turn_right_positions.
                // println!("Last 1st turn pos : {:?}",turn_right_positions.iter().nth_back(1).unwrap_or(&(0,0)));
                // println!("Last 2nd turn pos : {:?}",turn_right_positions.iter().nth_back(2).unwrap_or(&(0,0)));
                // println!("Last 3rd turn pos : {:?}",turn_right_positions.iter().nth_back(3).unwrap_or(&(0,0)));
                // println!("Last 4th turn pos : {:?}",turn_right_positions.iter().nth_back(4).unwrap_or(&(0,0)));
                // println!("Current turn por: {:?}",&starting_pos.clone());

                
                if turn_right_positions.iter().nth_back(4).unwrap_or(&(0,0)) == &starting_pos.clone() {
                    loop_count +=1;
                    obstacle_positions.insert(next_position.clone());
                    println!("Loop found! ");

                    pause_for_input();
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

                println!("/////// Breaking on {i} iterations! for turn_here: {:?}", turn_here)
            }
        }


    } 

    // print_map(&columns, map_height, map_width, None, None);
    // println!("Positions visited: {:?}", positions_visited);
    println!("loop count: {loop_count}");
    println!("{:?}", obstacle_positions);
    println!("Obstacles count: {}", obstacle_positions.len());

    // 35 wrong.
    // 1649 too low
    // 4819 too high

}
