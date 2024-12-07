use std::{collections::HashSet, fs::read_to_string, ops::Index};

fn main() {
    let test = false;


    fn get_direction(string: &char) -> (i32,i32) {
        println!("Matching string to get direction: {string}");
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



    let file_path = if test {"./test_input.txt"} else {"./input.txt"};
    println!("In file {file_path}");

    let mut columns: Vec<Vec<char>> = Vec::new();
    

    let mut map_width = 0;
    
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
            }
            columns[column_index].push(character);
        }
    }
    let map_height = columns.first().unwrap().len() -1;

    // for column in columns.clone() {
    //     println!("{:?}", column);

    // }

    for y_i in 0..map_height {
        let mut line:Vec<String> = vec![];
        for x_i in 0..map_width {
            let val = columns.index(x_i).index(y_i).to_string();
            line.push(val);
        }
        println!("{}", line.join(""));
    }


    
    // We have the starting position and the map now. 
    // Step forward based on the direction until hitting obstacle.
    let mut starting_string: char = *columns.index(starting_pos.0).index(starting_pos.1);
    println!("Starting string: {starting_string}");
  
    

    let map_last_x_index = map_width;
    let map_last_y_index = map_height;
    println!("Bounds: {map_last_x_index}, {map_last_y_index}");

    let path_is_free = true;    
    let mut positions_visited = HashSet::new();
    let mut while_count = 0;
    while path_is_free {
        let direction = get_direction(&starting_string);
        while_count +=1;
        positions_visited.insert(starting_pos.clone()); // Add starting position


        let next_is_outside_map = match (starting_pos, starting_string) {
            ((_, 0), '^') => true,
            ((_,index), 'v') if index == map_last_y_index =>true,
            ((_,index), '>') if index == map_last_x_index =>true,
            ((0, _), '<') => true,
            _ => false,
        };
        if next_is_outside_map {
            println!("Last position before leaving map is: {:?}", starting_pos);
            break
        }

        println!("Adding direction: {:?}", direction);
        // Check for obstacles. 
        let next_position = (
            (starting_pos.0 as i32 + direction.0).max(0) as usize,
            (starting_pos.1 as i32 + direction.1).max(0) as usize,
        );


        

        let next_char = *columns.index(next_position.0).index(next_position.1);
        let next_is_obstacle = next_char == '#';
        if next_char == starting_string {
            println!("Error! Next char is starting string {starting_string} at position {:?}", next_position);
            break
        }


        if next_is_obstacle {
            println!("Next is obstacle is ahead current pos: {:?} and direction: {starting_string}", starting_pos);
            let new_direction = change_direction(&starting_string);
            println!("New direction should be: {}", new_direction);
            starting_string = new_direction;
        } else {
            // Next is free. Update starting position.
            // println!("Step {starting_string} to {:?}", next_position);
            starting_pos = next_position;
        }


        if while_count > 10000 {
            println!("While break");
            break
        }
    }

    println!("Count: {}", positions_visited.len());





}
