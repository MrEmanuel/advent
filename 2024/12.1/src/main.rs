use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};
use utils::{count_neighbors, print_map_animate, DEBUG, TEST};

mod utils {
    use std::{collections::HashMap, io, isize, thread, time};
    pub const TICKER_SPEED: u64 = 40;
    pub const DEBUG: bool = true;
    pub const TEST: bool = true;
    pub const VIEWPORT_HEIGHT: usize = 30;
    pub const VIEWPORT_WIDTH: usize = 30;
    pub const PAUSE_ON_EACH_FRAME: bool = true;
    const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    fn is_within_bounds(x: isize, y: isize, map_height: usize, map_width: usize) -> bool {
        let x_ok = x >= 0 && x < map_width as isize;
        let y_ok = y >= 0 && y < map_height as isize;
        return y_ok && x_ok;
    }
    pub fn count_neighbors(mark: char, pos: (usize, usize), columns: &Vec<Vec<char>>) -> usize {
        let mut neighbor_count = 0;
        for (x_diff, y_diff) in DIRECTIONS {
            let (x, y) = (pos.0 as isize + x_diff, pos.1 as isize + y_diff);
            if is_within_bounds(x, y, columns.len(), columns[0].len()) {
                // Check the neighbor
                if mark == columns[x as usize][y as usize] {
                    neighbor_count += 1;
                }
            } else {
                if DEBUG {
                    println!("Tile {x},{y} to check out of bounds")
                }
            }
        }
        return neighbor_count;
    }
    pub fn wait_for_input(show_instructions: bool) {
        if !DEBUG {
            return;
        }
        let mut input = String::new();
        if show_instructions {
            println!("Press Enter to continue...");
        }

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    }
    pub fn sleep() {
        thread::sleep(time::Duration::from_millis(TICKER_SPEED));
    }
    pub fn print_map_animate(
        columns: &Vec<Vec<char>>,
        map_height: usize,
        map_width: usize,
        starting_pos: (usize, usize),
        starting_string: char,
        colored_positions: &HashMap<(usize, usize), (char, usize)>,
    ) {
        let (sx, sy) = starting_pos;
        // Calculate the top-left corner of the viewport
        let start_x = if sx > VIEWPORT_WIDTH / 2 {
            sx - VIEWPORT_WIDTH / 2
        } else {
            0
        };
        let start_y = if sy > VIEWPORT_HEIGHT / 2 {
            sy - VIEWPORT_HEIGHT / 2
        } else {
            0
        };

        // Clip the viewport to the grid boundaries
        let end_x = (start_x + VIEWPORT_WIDTH).min(map_width);
        let end_y = (start_y + VIEWPORT_HEIGHT).min(map_height);

        // Clear the map
        // print!("\x1B[2J\x1B[1;1H");
        let mut line_to_print = vec![];
        for y_i in start_y..end_y {
            let mut line: Vec<String> = vec![];
            for x_i in start_x..end_x {
                // If the position matches the cursor, display the override character

                if (x_i, y_i) == (sx, sy) {
                    // let start_char = format!(
                    //     "{}{}{}",
                    //     "\x1b[31m",
                    //     &starting_string.to_string(),
                    //     "\x1b[39m"
                    // );
                    line.push(format!(
                        "{}{}{}",
                        "\x1b[31m",
                        &starting_string.to_string(),
                        "\x1b[39m"
                    ));
                    continue;
                }
                // if extra_obstacle_pos
                //     .as_ref()
                //     .is_some_and(|obst_pos| obst_pos.contains(&(x_i, y_i)))
                match colored_positions.get(&(x_i, y_i)) {
                    Some((pos_char, _)) => {
                        line.push("\x1b[31m".to_string() + &pos_char.to_string() + "\x1b[39m");
                        continue;
                    }
                    _ => {}
                }
                // Default to the grid character
                line.push(columns[x_i][y_i].to_string());
            }
            // Print the new line
            line_to_print.push(line.join(""));
            // line_to_print.push("\n".to_string());
        }

        println!("{}", line_to_print.join("\n"));
        if PAUSE_ON_EACH_FRAME {
            wait_for_input(false);
        }
    }
}

fn main() {
    // Calculate area and perimiter.
    // Area = tile count.
    // Perimiter per block = 4 - neighbour count

    // Go row by row. Find a new starting position. Add it to a map.
    // From that starting position, go row by row.
    // For each matching tile, count the neighbours and collect if they have at least 1.
    // When you encounter a row without any neighbors, break and start over.

    let file_path = if TEST {
        "./test_input2.txt"
    } else {
        "./input.txt"
    };
    println!("In file {file_path}");
    let mut columns: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        let mut row = vec![];
        for char in line.chars() {
            row.push(char);
        }
        columns.push(row);
    }
    let map_height = columns[0].len();
    let map_width = columns.len();
    let first_starting_pos = (0, 0);
    let mut tiles: HashMap<(usize, usize), (char, usize)> = HashMap::new();
    let mut regions: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    // Insert first position as starting point.
    regions.insert((0, 0), vec![]);

    let mut starting_positions = VecDeque::new();
    starting_positions.push_back(first_starting_pos);

    while !starting_positions.is_empty() {
        if DEBUG {
            println!("Starting positions: {:?}", starting_positions);
        }
        let mut next_start_pos_found = false;
        let starting_pos = starting_positions.pop_front().unwrap();
        let starting_string = columns[starting_pos.0][starting_pos.1];
        for y in 0..map_height {
            if y < starting_pos.1 {
                // Skip until starting pos.
                if DEBUG {
                    print!("y {y} < starting_pos.1, skipping.. ")
                }
                continue;
            }

            let mut row_has_neighbor = false;
            for x in 0..map_width {
                if y == starting_pos.1 && x < starting_pos.0 {
                    if DEBUG {
                        print!("x {x} < starting_pos.0, skipping.. ")
                    }
                    // Skip until starting pos.
                    continue;
                }
                // 1. Check if starting_string matches.
                let tile = columns[x][y];
                let same_region = starting_string == tile;

                if DEBUG {
                    println!("Same region: {same_region}");
                }

                if same_region {
                    let neighbors_count = count_neighbors(starting_string, (x, y), &columns);
                    if DEBUG {
                        println!("neighbors_count: {neighbors_count}");
                    }

                    if neighbors_count > 0 || starting_pos == (x, y) {
                        tiles.insert((x, y), (tile, neighbors_count));
                    }

                    if !row_has_neighbor && neighbors_count > 0 {
                        row_has_neighbor = true;
                    }
                } else if !next_start_pos_found && !tiles.contains_key(&(x, y)) {
                    if DEBUG {
                        println!(
                            "Setting next starting position: {:?}, {}",
                            (x, y),
                            columns[x][y]
                        )
                    }
                    next_start_pos_found = true;
                    starting_positions.push_back((x, y));
                    regions.insert((x, y), vec![]);
                }

                print_map_animate(
                    &columns,
                    map_height,
                    map_width,
                    (x, y),
                    starting_string,
                    &tiles,
                );
            }
            if !row_has_neighbor {
                print!("ROw has no neightbors..");
                // If row has no neighbor,
                // break and start with the next starting point.
                break;
            }
        }

        // starting_positions
    }

    // Animate map.

    // for y in 0..map_height {
    //     for x in 0..map_width {
    //         print_map_animate(
    //             &columns,
    //             map_height,
    //             map_width,
    //             (x, y),
    //             starting_string,
    //             &region,
    //         );
    //     }
    // }
}
