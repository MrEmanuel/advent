use std::{collections::HashMap, fs::read_to_string};
use utils::{get_neighbors, print_map_animate, DEBUG, TEST};

mod utils {
    use std::{io, isize, thread, time};
    pub const TICKER_SPEED: u64 = 40;
    pub const DEBUG: bool = false;
    pub const TEST: bool = false;
    pub const VIEWPORT_HEIGHT: usize = 30;
    pub const VIEWPORT_WIDTH: usize = 30;
    pub const PAUSE_ON_EACH_FRAME: bool = true;
    const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    fn is_within_bounds(x: isize, y: isize, map_height: usize, map_width: usize) -> bool {
        let x_ok = x >= 0 && x < map_width as isize;
        let y_ok = y >= 0 && y < map_height as isize;
        return y_ok && x_ok;
    }
    pub fn get_neighbors(
        mark: char,
        pos: (usize, usize),
        columns: &Vec<Vec<char>>,
    ) -> (Vec<(usize, usize)>, usize) {
        let mut out_of_bounds_count = 0;
        let mut neighbors = vec![];
        for (x_diff, y_diff) in DIRECTIONS {
            let (x, y) = (pos.0 as isize + x_diff, pos.1 as isize + y_diff);
            if is_within_bounds(x, y, columns.len(), columns[0].len()) {
                // Check the neighbor
                if mark == columns[x as usize][y as usize] {
                    neighbors.push((x as usize, y as usize));
                }
            } else {
                out_of_bounds_count += 1;
                // if DEBUG {
                //     println!("Tile {x},{y} to check out of bounds")
                // }
            }
        }
        return (neighbors, out_of_bounds_count);
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
        colored_secondary_positions: &Vec<(usize, usize)>,
        colored_positions: &Vec<(usize, usize)>,
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

        if !PAUSE_ON_EACH_FRAME {
            print!("\x1B[2J\x1B[1;1H");
        }
        let mut line_to_print = vec![];
        for y_i in start_y..end_y {
            let mut line: Vec<String> = vec![];
            for x_i in start_x..end_x {
                // If the position matches the cursor, display the override character
                if (y_i, x_i) == (sx, sy) {
                    line.push(format!(
                        "{}{}{}",
                        "\x1b[1;31m",
                        &starting_string.to_string(),
                        "\x1b[0m"
                    ));
                    continue;
                }
                if colored_positions.contains(&(y_i, x_i)) {
                    line.push("\x1b[31m".to_string() + &columns[y_i][x_i].to_string() + "\x1b[39m");
                    continue;
                }
                if colored_secondary_positions.contains(&(y_i, x_i)) {
                    line.push("\x1b[32m".to_string() + &columns[y_i][x_i].to_string() + "\x1b[39m");
                    continue;
                }
                // Default to the grid character
                line.push(columns[y_i][x_i].to_string());
            }
            // Print the new line
            line_to_print.push(line.join(""));
            // line_to_print.push("\n".to_string());
        }

        println!("{}", line_to_print.join("\n"));
        if PAUSE_ON_EACH_FRAME {
            wait_for_input(false);
        } else {
            sleep();
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

    // Method.
    // Keep an array for current region. Add all positions that belong to the region.
    // Add any neighbors they have and keep checking those too.

    // Method 2.
    // "Walk around" the perimiter of the region, saving each tile to an array.
    //

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

    struct RegionPosData {
        pos: (usize, usize),
        mark: char,
        _neigbor_count: usize,
        parimiter_count: usize, // 4 - neighbor_count
    }

    let mut regions: HashMap<(usize, usize), HashMap<(usize, usize), RegionPosData>> =
        HashMap::new();
    regions.insert((0, 0), HashMap::new());

    let mut processed_tiles: Vec<(usize, usize)> = vec![];
    let mut region_queue: Vec<(usize, usize)> = vec![(0, 0)];
    let mut current_start_position: (usize, usize) = (0, 0);

    while !region_queue.is_empty() {
        if DEBUG {
            println!(" ");
            println!(" ");
            println!(
                "1.Before pop. Current region, i.e tiles left to check : {:?}",
                region_queue
            );
        }
        let popped_pos = region_queue.pop();

        if DEBUG {
            println!("Checking {:?} in current region. ", popped_pos);
            println!(
                "2. After pop. Current region, i.e tiles left to check : {:?}",
                region_queue
            );
        }

        let (current_pos, current_mark) = match popped_pos {
            Some(pos) => (pos, columns[pos.0][pos.1]),
            None => unreachable!("Current pos undefined.."),
        };

        if DEBUG {
            println!("Current mark: {current_mark}");
            print_map_animate(
                &columns,
                map_height,
                map_width,
                current_pos,
                current_mark,
                &processed_tiles,
                &regions
                    .get(&current_start_position)
                    .unwrap()
                    .iter()
                    .map(|(&key, _)| key)
                    .collect(),
            );
        }
        processed_tiles.push(current_pos); // Add to processed_tiles so it doesn't get added as a starting point for a new region.
        let (neighbors, _out_of_bounds_count) = get_neighbors(current_mark, current_pos, &columns);
        // Insert into or create a map for current_pos and its data for the current region.
        if DEBUG {
            println!("{:?} has neighbors: {:?}", current_pos, neighbors);

            println!(
                "Adding {:?} with neighbors: {} and parameter count: {} ",
                current_pos,
                neighbors.len(),
                4 - neighbors.len()
            );
        }
        regions
            .entry(current_start_position)
            .and_modify(|map| {
                map.insert(
                    current_pos,
                    RegionPosData {
                        pos: current_pos,
                        mark: current_mark,
                        _neigbor_count: neighbors.len(),
                        parimiter_count: 4 - neighbors.len(),
                    },
                );
            })
            .or_insert_with(HashMap::new)
            .insert(
                current_pos,
                RegionPosData {
                    pos: current_pos,
                    mark: current_mark,
                    _neigbor_count: neighbors.len(),
                    parimiter_count: 4 - neighbors.len(),
                },
            );

        for i in 0..neighbors.len() {
            if !processed_tiles.contains(&neighbors[i]) && !region_queue.contains(&neighbors[i]) {
                if DEBUG {
                    println!(
                        "Current region {:?} doesn't contain {:?}",
                        region_queue, neighbors[i]
                    );
                    println!("...Adding {:?} to current region", neighbors[i]);
                }
                // If neighbor is not in current_neighbors, add so it will be processed and added to regions.
                region_queue.push(neighbors[i]);
                // existing_neighbors.insert(neighbor, RegionPosData {mark: current_mark,neigbor_count});
            }
        }

        if region_queue.is_empty() {
            // Find the next starting position
            // Start top left, and use the first found tile that isn't already in a region
            'outer: for y in 0..columns[0].len() {
                for x in 0..columns.len() {
                    if !processed_tiles.contains(&(x, y)) {
                        // New region found.
                        // Set as current_start_position and
                        // add map to current_region

                        current_start_position = (x, y);
                        region_queue.push((x, y));
                        //
                        regions.insert(current_start_position, HashMap::new());
                        break 'outer;
                    }
                }
            }
        }
    }

    let mut total_cost = 0;
    for region in regions.iter() {
        let (_pos, region) = region;

        // println!("pos: {:?}, region {:?}", pos, region);
        let mut area = 0;
        let mut char: char = '-';
        let mut perimiter = 0;
        region.iter().for_each(|(_, data)| {
            // For each tile, calculate it's addition to the circumference
            if DEBUG {
                println!(
                    "Tile {}, pos:{:?}, count: {}",
                    data.mark, data.pos, data.parimiter_count
                );
            }
            // let tile_data = tiles.get(tile).unwrap();
            area += 1;
            char = data.mark;
            perimiter += data.parimiter_count;
        });

        let cost = area * perimiter;
        total_cost += cost;
        if DEBUG {
            println!("Region {char} with price {area} * {perimiter} = {cost}")
        }
    }
    println!("Total cost: {total_cost}")
}
