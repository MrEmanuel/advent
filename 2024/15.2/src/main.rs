use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    io,
};
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

    fn set_crates_to_move(
        columns: &Vec<Vec<char>>,
        crates_to_move: &mut Vec<(isize, isize)>,
        start_pos: (isize, isize),
        direction: (isize, isize),
        // print_map: impl Fn(&Vec<Vec<char>>, usize, usize, Option<&Vec<OverridePos>>),
    ) -> bool {
        //Check next position. If

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

        let new_pos = (
            start_pos.0 as isize + direction.0,
            start_pos.1 as isize + direction.1,
        );
        let new_pos_mark = columns[new_pos.0 as usize][new_pos.1 as usize];

        if new_pos_mark == '.' {
            return true;
        } else if new_pos_mark == '#' {
            return false;
        }

        let other_half = match new_pos_mark {
            '[' => {
                // x+1
                (new_pos.0 + 1, new_pos.1)
            }
            ']' => {
                // x-1
                (new_pos.0 - 1, new_pos.1)
            }
            _ => unreachable!("Couldn't find [ or ], found {new_pos_mark} instead.."),
        };

        let other_half_mark = columns[other_half.0 as usize][other_half.1 as usize];

        if DEBUG {
            print_map(
                columns,
                columns[0].len(),
                columns.len(),
                Some(&vec![
                    OverridePos {
                        color: Some("red".to_string()),
                        mark: Some(new_pos_mark),
                        pos: (new_pos.0 as usize, new_pos.1 as usize),
                    },
                    OverridePos {
                        color: Some("red".to_string()),
                        mark: Some(other_half_mark),
                        pos: (other_half.0 as usize, other_half.1 as usize),
                    },
                ]),
            );
            println!(
                "Checking {:?} and {:?} in set crates to move..",
                new_pos, other_half
            );
            wait_for_input(false);
        }

        let is_ok1 = set_crates_to_move(&columns, crates_to_move, new_pos, direction);
        let is_ok2 = set_crates_to_move(&columns, crates_to_move, other_half, direction);
        if !is_ok1 || !is_ok2 {
            // Don't move if any of the paths led to an obstacle.
            return false;
        }

        if DEBUG {
            println!("Setting crates to move inside function.. ");
        }
        crates_to_move.push(new_pos);
        crates_to_move.push(other_half);

        return true;
    }

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

        let mut line_vec: VecDeque<char> = line.chars().collect();
        let mut new_line: Vec<char> = vec![];
        while !line_vec.is_empty() {
            let char = line_vec.pop_front().unwrap();
            match char {
                'O' => {
                    new_line.push('[');
                    new_line.push(']');
                }
                '#' => {
                    new_line.push('#');
                    new_line.push('#');
                }
                '.' => {
                    new_line.push('.');
                    new_line.push('.');
                }
                '@' => {
                    new_line.push('@');
                    new_line.push('.');
                }
                _ => {
                    unreachable!("This should be unreachable")
                }
            }
        }
        println!("{:?}", new_line);

        if row_index == 0 {
            map_width = new_line.len();
            // Create 1 vec per column, i.e 1 per new_line_len
            for _ in 0..new_line.len() {
                columns.push(vec![]);
            }
            println!(
                "Map is {} wide and {} high.",
                columns.len(),
                columns.first().unwrap().len()
            )
        }

        // for column_index in 0..new_line.len() {
        //     columns[column_index].push(new_line[row_index]);
        // }

        for column_index in 0..map_width {
            let character = new_line[column_index];

            if character == '@' {
                println!("Found starting pos: {column_index},{row_index}");
                starting_pos = (column_index, row_index);
            }
            columns[column_index].push(character);
        }
    }

    let map_height = columns.first().unwrap().len();
    print_map(&columns, map_height, map_width, None);

    if DEBUG {
        println!("Instructions: {:?}", instructions);
    }

    let mut robot_pos = starting_pos;
    'instructions: for instruction in instructions {
        if instruction == "" {
            continue;
        }
        let direction = get_direction(instruction);
        // Check if move is valid.
        let mut free_space_pos: Option<(isize, isize)> = None;
        let mut found_crates = 0;
        let mut crates_to_move: Vec<(isize, isize)> = vec![];
        let mut next_step: (isize, isize) = (0, 0);
        let mut increment = 0;

        if instruction == "<" || instruction == ">" {
            'increment: for _ in 1..(map_width as isize) {
                //Move and find an empty spot, or wall.
                increment += 1;

                let vector = (direction.0 * increment, direction.1 * increment);
                let new_pos = (
                    robot_pos.0 as isize + vector.0,
                    robot_pos.1 as isize + vector.1,
                );
                if increment == 1 {
                    next_step = new_pos
                }

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
                    '[' => {
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
                        crates_to_move.push(new_pos);
                        // increment += 1;
                        found_crates += 1;
                    }
                    ']' => {
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

                        crates_to_move.push(new_pos);
                        // increment += 1;
                        found_crates += 1;
                    }
                    '.' => {
                        if DEBUG {
                            println!("horiz Found free space at {:?}", new_pos);
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
                    // if we found a crate, update the empty position with it.
                    if found_crates > 0 {
                        let mut prev_pos = free_pos;
                        for crate_index in (0..crates_to_move.len()).rev() {
                            let crate_pos = crates_to_move[crate_index];
                            let crate_mark = columns[crate_pos.0 as usize][crate_pos.1 as usize];
                            // Move crate mark to prev pos.
                            columns[prev_pos.0 as usize][prev_pos.1 as usize] = crate_mark;
                            prev_pos = crate_pos;
                            // prev_mark = crate_mark;
                        }
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
        } else {
            // Vertical movement..

            // For each increment, add crates to check in a vector.
            // let mut crates_to_check: Vec<Vec<(usize, usize)>> = vec![];
            // let mut increment = 0;
            // 'increment: for _ in 1..(map_width as isize) {
            // break if first step is a wall or free space.
            // Go through the "pyramid"

            increment += 1;
            let vector = (direction.0 * increment, direction.1 * increment);
            let new_pos = (
                robot_pos.0 as isize + vector.0,
                robot_pos.1 as isize + vector.1,
            );
            // if increment == 1 {
            next_step = new_pos;
            let new_pos_mark = columns[new_pos.0 as usize][new_pos.1 as usize];
            match new_pos_mark {
                // Vertical movement..
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
                '.' => {
                    if DEBUG {
                        println!("vert Found free space at {:?}", new_pos);
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

                    if DEBUG {
                        println!("Found verical free space!")
                    }
                }
                // Vertical movement..
                _ => {
                    if new_pos_mark != '[' && new_pos_mark != ']' {
                        if DEBUG {
                            println!("Early return..");
                        }
                        return;
                    }

                    let other_half = match new_pos_mark {
                        '[' => {
                            // x+1
                            (new_pos.0 + 1, new_pos.1)
                        }
                        ']' => {
                            // x-1
                            (new_pos.0 - 1, new_pos.1)
                        }
                        _ => unreachable!("Couldn't find [ or ], found {new_pos_mark} instead.."),
                    };

                    // Walk the tree of crate-chars ] or [
                    let is_ok1 =
                        set_crates_to_move(&columns, &mut crates_to_move, new_pos, direction);
                    let is_ok2 =
                        set_crates_to_move(&columns, &mut crates_to_move, other_half, direction);
                    if !is_ok1 || !is_ok2 {
                        if DEBUG {
                            println!(
                                "Crates {:?} and {:?} are NOT ok to move!",
                                new_pos, other_half
                            );
                        }
                        crates_to_move = vec![];
                        continue 'instructions; // Go to next instruction

                    // Don't move if any of the paths led to an obstacle.
                    } else {
                        crates_to_move.push(new_pos);
                        crates_to_move.push(other_half);
                    }
                }
            }

            if crates_to_move.is_empty() {
                match free_space_pos {
                    Some(free_pos) => {
                        // println!("Found free space at: {:?}", free_pos);

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
            } else {
                // Perform the movements of the crates..
                let mut moved_crates = HashSet::new();

                if direction.1 < 0 {
                    crates_to_move.sort_by(|a, b| (b.1 as usize).cmp(&(a.1 as usize)));
                } else {
                    crates_to_move.sort_by(|a, b| (a.1 as usize).cmp(&(b.1 as usize)));
                }

                while !crates_to_move.is_empty() {
                    let crate1 = crates_to_move.pop().unwrap();
                    let size_before = moved_crates.len();
                    moved_crates.insert(crate1);

                    if moved_crates.len() == size_before {
                        continue;
                    }

                    let crate1_mark = columns[crate1.0 as usize][crate1.1 as usize];
                    let crate1_new_pos = (crate1.0 + direction.0, crate1.1 + direction.1);
                    columns[crate1.0 as usize][crate1.1 as usize] = '.';
                    columns[crate1_new_pos.0 as usize][crate1_new_pos.1 as usize] = crate1_mark;
                }

                // Move robot.
                // Update the position of the robot.
                columns[next_step.0 as usize][next_step.1 as usize] = '@';
                // Replace robot_pos with an empty space.
                columns[robot_pos.0][robot_pos.1] = '.';
                // Update robot_pos with the new one.
                robot_pos = (next_step.0 as usize, next_step.1 as usize);

                print_map(&columns, map_height, map_width, None);
            }

            // Vertical movement.. end.
        }
    }

    print_map(&columns, map_height, map_width, None);

    let mut total = 0;
    for x in 0..columns.len() {
        for y in 0..columns[x].len() {
            let mark = columns[x][y];
            if mark == '[' {
                total += (100 * y) + x;
            }
        }
    }
    println!("Total: {}", total);
}
