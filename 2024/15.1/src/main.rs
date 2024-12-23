use std::{fs::read_to_string, io};
use utils::{DEBUG, TEST};

mod utils {
    pub const TEST: bool = false;
    pub const DEBUG: bool = false;
}

struct OverridePos {
    pos: (usize, usize),
    mark: Option<char>,
    color: Option<String>,
}

fn main() {
    fn get_direction(string: &str) -> (isize, isize) {
        return match string {
            "v" => (0, 1),
            "^" => (0, -1),
            "<" => (-1, 0),
            ">" => (1, 0),
            _ => unreachable!("Unexpected value: {}", string),
        };
    }

    fn add_color(char: char, color: &str) -> String {
        match color {
            "red" => "\x1b[31m".to_string() + &char.to_string() + "\x1b[39m",
            _ => char.to_string(),
        }
    }

    pub fn wait_for_input(show_instruction: bool) {
        if !DEBUG {
            return;
        }
        let mut input = String::new();
        if show_instruction {
            println!("Press Enter to continue...");
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    }

    let print_map = |columns: &Vec<Vec<char>>,
                     map_height: usize,
                     map_width: usize,
                     override_positions: Option<&Vec<OverridePos>>| {
        if !DEBUG {
            return;
        }
        for y_i in 0..map_height {
            let mut line: Vec<String> = vec![];
            for x_i in 0..map_width {
                // let val = columns[x_i][y_i].to_string();

                let val_to_push = match override_positions {
                    Some(override_positions) => {
                        match override_positions.iter().find(|pos| pos.pos == (x_i, y_i)) {
                            Some(override_pos) => {
                                // override_pos.color, override_pos.mark,
                                let mark = match override_pos.mark {
                                    Some(mark) => mark,
                                    None => columns[x_i][y_i],
                                };

                                let res = match &override_pos.color {
                                    Some(color) => add_color(mark, &color),
                                    None => mark.to_string(),
                                };
                                res
                            }
                            None => columns[x_i][y_i].to_string(),
                        }
                    }
                    None => columns[x_i][y_i].to_string(),
                };
                line.push(val_to_push);
            }
            if DEBUG {
                println!("{}", line.join(""));
            }
        }
        if DEBUG {
            wait_for_input(true);
        }
    };

    let wait_for_input = |show_instruction: bool| {
        if !DEBUG {
            return;
        }
        let mut input = String::new();
        if show_instruction {
            println!("Press Enter to continue...");
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    };

    let instructions_file = if TEST {
        "./test_instructions.txt"
    } else {
        "./instructions.txt"
    };
    let map_file = if TEST { "./test_map.txt" } else { "./map.txt" };
    let _instructions = read_to_string(instructions_file).unwrap().replace("\n", "");
    let instructions: Vec<&str> = _instructions.split("").collect();

    let mut columns: Vec<Vec<char>> = Vec::new();
    let mut map_width = 0;
    let mut starting_pos: (usize, usize) = (0, 0);

    for (row_index, line) in read_to_string(map_file).unwrap().lines().enumerate() {
        // Collect the entire map in to multiple columns.

        if row_index == 0 {
            map_width = line.len();
            // Create 1 vec per column, i.e 1 per line_len
            for _ in 0..line.len() {
                columns.push(vec![]);
            }
            println!(
                "Map is {} wide and {} high.",
                columns.len(),
                columns.first().unwrap().len()
            )
        }
        for column_index in 0..map_width {
            let character = line.chars().nth(column_index).unwrap();

            if character == '@' {
                println!("Found starting pos: {column_index},{row_index}");
                starting_pos = (column_index, row_index);
            }
            columns[column_index].push(character);
        }
    }
    let map_height = columns.first().unwrap().len();
    let mut robot_pos = starting_pos;
    'instructions: for instruction in instructions {
        if instruction == "" {
            continue;
        }
        let direction = get_direction(instruction);
        // Check if move is valid.
        let mut free_space_pos: Option<(isize, isize)> = None;
        let mut found_crates = 0;
        let mut next_step: (isize, isize) = (0, 0);
        'increment: for increment in 1..(map_width as isize) {
            //Move and find an empty spot, or wall.

            let vector = (direction.0 * increment, direction.1 * increment);
            let new_pos = (
                robot_pos.0 as isize + vector.0,
                robot_pos.1 as isize + vector.1,
            );
            if increment == 1 {
                next_step = new_pos
            }

            // TODO: Check if position is inside map.
            match columns[new_pos.0 as usize][new_pos.1 as usize] {
                '#' => {
                    // Can't move
                    if DEBUG {
                        println!("Found wall at {:?}", new_pos);
                        print_map(
                            &columns,
                            map_height,
                            map_width,
                            Some(&vec![OverridePos {
                                mark: Some(columns[new_pos.0 as usize][new_pos.1 as usize]),
                                color: Some("red".to_string()),
                                pos: (new_pos.0 as usize, new_pos.1 as usize),
                            }]),
                        );
                    }
                    continue 'instructions; // Go to next instruction
                }
                'O' => {
                    if DEBUG {
                        println!("Found crate at {:?}", new_pos);
                        print_map(
                            &columns,
                            map_height,
                            map_width,
                            Some(&vec![OverridePos {
                                mark: Some(columns[new_pos.0 as usize][new_pos.1 as usize]),
                                color: Some("red".to_string()),
                                pos: (new_pos.0 as usize, new_pos.1 as usize),
                            }]),
                        );
                    }
                    found_crates += 1;
                }
                '.' => {
                    if DEBUG {
                        println!("Found free space at {:?}", new_pos);
                        print_map(
                            &columns,
                            map_height,
                            map_width,
                            Some(&vec![OverridePos {
                                mark: Some(columns[new_pos.0 as usize][new_pos.1 as usize]),
                                color: Some("red".to_string()),
                                pos: (new_pos.0 as usize, new_pos.1 as usize),
                            }]),
                        );
                    }
                    free_space_pos = Some(new_pos);
                    break 'increment; // Make a move forward
                }

                _ => {}
            }
        }

        match free_space_pos {
            Some(free_pos) => {
                // println!("Found free space at: {:?}", free_pos);
                // if we found a crate, update the empty position with it.
                if found_crates > 0 {
                    columns[free_pos.0 as usize][free_pos.1 as usize] = 'O';
                }
                // Update the position of the robot.
                columns[next_step.0 as usize][next_step.1 as usize] = '@';
                // Replace robot_pos with an empty space.
                columns[robot_pos.0][robot_pos.1] = '.';
                // Update robot_pos with the new one.
                robot_pos = (next_step.0 as usize, next_step.1 as usize);

                // print_map(&columns, map_height, map_width, None)
            }
            None => {
                println!("Do nothing..")
            }
        }
    }

    let mut total = 0;
    for x in 0..columns.len() {
        for y in 0..columns[x].len() {
            let mark = columns[x][y];
            if mark == 'O' {
                total += (100 * y) + x;
            }
        }
    }
    println!("Total: {}", total);
}
