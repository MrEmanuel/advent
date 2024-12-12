use std::{collections::HashMap, fs::read_to_string};
use utils::{print_map_animate, TEST};

mod utils {
    use std::{collections::HashMap, io, isize, thread, time};
    pub const TICKER_SPEED: u64 = 40;
    pub const DEBUG: bool = true;
    pub const TEST: bool = true;
    pub const VIEWPORT_HEIGHT: usize = 30;
    pub const VIEWPORT_WIDTH: usize = 30;
    pub const PAUSE_ON_EACH_FRAME: bool = true;
    pub fn count_neighbors(mark: &str, pos1: (isize, isize), pos2: (isize, isize)) {}
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
        columns: Vec<Vec<char>>,
        map_height: usize,
        map_width: usize,
        starting_pos: (usize, usize),
        starting_string: char,
        colored_positions: HashMap<(usize, usize), (char, usize)>,
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
        print!("\x1B[2J\x1B[1;1H");
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

        print!("{}", line_to_print.join("\n"));
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
        "./test_input.txt"
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
    let starting_pos = (0, 0);
    let starting_string = columns[starting_pos.0][starting_pos.1];
    let region: HashMap<(usize, usize), (char, usize)> = HashMap::new();
    // let starting_string =

    print_map_animate(
        columns,
        map_height,
        map_width,
        starting_pos,
        starting_string,
        region,
    );
}
