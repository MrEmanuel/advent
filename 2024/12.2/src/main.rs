use std::{collections::HashMap, fs::read_to_string, ops::Index};
use utils::{
    get_neighbors, is_within_bounds, print_map_animate, wait_for_input, DEBUG, DIRECTIONS, TEST,
};

mod utils {
    use std::{io, isize, thread, time};
    pub const TICKER_SPEED: u64 = 40;
    pub const DEBUG: bool = false;
    pub const TEST: bool = true;
    pub const VIEWPORT_HEIGHT: usize = 30;
    pub const VIEWPORT_WIDTH: usize = 30;
    pub const PAUSE_ON_EACH_FRAME: bool = false;
    pub const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    pub fn is_within_bounds(x: isize, y: isize, map_height: usize, map_width: usize) -> bool {
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
        show_starting_pos: bool,
        starting_string: char,
        colored_secondary_positions: &Vec<(usize, usize)>,
        colored_positions: &Vec<(isize, isize)>,
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
            // print!("\x1B[2J\x1B[1;1H");
        }
        let mut line_to_print = vec![];
        for y_i in start_y..end_y {
            let mut line: Vec<String> = vec![];
            for x_i in start_x..end_x {
                // If the position matches the cursor, display the override character
                if show_starting_pos && (x_i, y_i) == (sx, sy) {
                    line.push(format!(
                        "{}{}{}",
                        "\x1b[1;31m",
                        &starting_string.to_string(),
                        "\x1b[0m"
                    ));
                    continue;
                }
                if colored_positions.contains(&(x_i as isize, y_i as isize)) {
                    line.push("\x1b[31m".to_string() + &columns[x_i][y_i].to_string() + "\x1b[39m");
                    continue;
                }
                if colored_secondary_positions.contains(&(x_i, y_i)) {
                    line.push("\x1b[32m".to_string() + &columns[x_i][y_i].to_string() + "\x1b[39m");
                    continue;
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
        "./test_input.txt"
    } else {
        "./input.txt"
    };
    println!("In file {file_path}");
    let mut columns: Vec<Vec<char>> = Vec::new();

    for (row_index, line) in read_to_string(file_path).unwrap().lines().enumerate() {
        if row_index == 0 {
            for _ in 0..line.len() {
                columns.push(vec![]);
            }
        }
        for (column_index, char) in line.chars().enumerate() {
            columns[column_index].push(char);
        }
        // columns.push(row);
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

            //
            // print_map_animate(
            //     &columns,
            //     map_height,
            //     map_width,
            //     current_pos,
            //     current_mark,
            //     &processed_tiles,
            //     &regions
            //         .get(&current_start_position)
            //         .unwrap()
            //         .iter()
            //         .map(|(&key, _)| key)
            //         .collect(),
            // );
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

    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

    // 1. Disregard any position without neighbors.
    // 2. Sort the region tiles.
    // 3. Count the sides by going line by line, and column by column.
    // Only count one of the sides, moving forward. Always count first and last sides.
    // Up -> down, down-> up. Left-> right, right -> left

    println!("Using file: {file_path}");
    print_map_animate(
        &columns,
        map_height,
        map_width,
        (0, 0),
        false,
        columns[0][0],
        &vec![],
        &vec![],
    );

    let get_position_cost = |pos: &RegionPosData,
                             col_index: usize,
                             row_index: usize,
                             direction: (isize, isize),
                             sides: &mut Vec<(isize, isize)>,
                             sides_count: &mut usize,
                             region_tiles: &Vec<(usize, usize)>| {
        // if let Some(pos) = tiles_data.get(&(col_index, row_index)) {
        // If position is in the region..
        let mark = columns[col_index][row_index];
        // let neighbors = get_neighbors(
        //     columns[col_index][row_index],
        //     (col_index, row_index),
        //     &columns,
        // );
        // let tile_to_check = (col_index+ direction.0, col_index + direction.1);
        let tile_to_check = (
            col_index as isize + direction.0,
            row_index as isize + direction.1,
        );

        let prev_tile = sides.last();
        // println!(
        //     "Checking direction {}: {:?}.  Mark {}. tile_to_check: {:?} prev_tile: {:?}",
        //     DIRECTIONS.iter().position(|dir| dir == &direction).unwrap(),
        //     direction,
        //     pos.mark,
        //     tile_to_check,
        //     prev_tile,
        // );
        // wait_for_input(false);
        if is_within_bounds(
            tile_to_check.0,
            tile_to_check.1,
            columns.len(),
            columns[0].len(),
        ) {
            // If tile_to_check is not a neighbor (the same mark), a new edge is found.
            if mark != columns[tile_to_check.0 as usize][tile_to_check.1 as usize] {
                // Check sides.last() if it is on the same row, and if it is next to tile_to_check

                if let Some(prev_tile) = prev_tile {
                    // TODO: on_same_row and is_neighbor works different for vertical and horizontal checks.
                    // direction
                    let (on_same_row, is_neighbor) = match direction {
                        (0, _) => (
                            tile_to_check.1 == prev_tile.1,
                            (tile_to_check.0 - prev_tile.0 as isize).abs() == 1,
                        ),
                        (_, 0) => (
                            tile_to_check.0 == prev_tile.0,
                            (tile_to_check.1 - prev_tile.1 as isize).abs() == 1,
                        ),
                        _ => unreachable!("Direction mismatch!"),
                    };

                    // let on_same_row = tile_to_check.1 == prev_tile.1;
                    // let is_neighbor = (tile_to_check.0 - prev_tile.0).abs() == 1;

                    if DEBUG {
                        println!(
                            "direction {:?} prev tile: {:?}, tile_to_check: {:?}. same row: {}, is_neighbor: {}. Adding count: {}, side count: {}",direction,
                            prev_tile, tile_to_check, on_same_row, is_neighbor, (!on_same_row || !is_neighbor),sides_count
                        );
                    }
                    if !on_same_row || !is_neighbor {
                        *sides_count += 1;
                    }
                } else {
                    *sides_count += 1;
                }
                sides.push((tile_to_check.0 as isize, tile_to_check.1 as isize));

                if DEBUG {
                    println!("Sides count:{sides_count}");
                    print_map_animate(
                        &columns,
                        map_height,
                        map_width,
                        // (0, 0),
                        (tile_to_check.0 as usize, tile_to_check.1 as usize),
                        true,
                        columns[tile_to_check.0 as usize][tile_to_check.1 as usize],
                        &region_tiles,
                        &sides,
                    );
                }
            }
        } else {
            // Outside of bounds, means adding a boundry.
            // println!(
            //     "Outside boundry.. Checking direction {}: {:?}.  Mark {}. tile_to_check: {:?} prev_tile: {:?}",
            //     DIRECTIONS.iter().position(|dir| dir == &direction).unwrap(),
            //     direction,
            //     pos.mark,
            //     tile_to_check,
            //     prev_tile,
            // );
            // wait_for_input(false);
            // let prev_tile = sides.last();
            if let Some(prev_tile) = prev_tile {
                // TODO: on_same_row and is_neighbor works different for vertical and horizontal checks.
                // direction
                let (on_same_row, is_neighbor) = match direction {
                    (0, _) => (
                        tile_to_check.1 == prev_tile.1,
                        (tile_to_check.0 - prev_tile.0 as isize).abs() == 1,
                    ),
                    (_, 0) => (
                        tile_to_check.0 == prev_tile.0,
                        (tile_to_check.1 - prev_tile.1 as isize).abs() == 1,
                    ),
                    _ => unreachable!("Direction mismatch!"),
                };

                // println!(
                //     "{} == {} and {} - {} for direction: {:?}",
                //     tile_to_check.1, prev_tile.1, tile_to_check.0, prev_tile.0, direction
                // );
                println!(
                    "direction {:?} prev tile: {:?}, tile_to_check: {:?}. same row: {}, is_neighbor: {}. Adding count: {}, side count: {}",direction,
                    prev_tile, tile_to_check, on_same_row, is_neighbor, (!on_same_row || !is_neighbor),sides_count
                );
                print_map_animate(
                    &columns,
                    map_height,
                    map_width,
                    // (0, 0),
                    pos.pos,
                    true,
                    pos.mark,
                    &region_tiles,
                    &sides,
                );
                wait_for_input(false);
                if !on_same_row || !is_neighbor {
                    *sides_count += 1;
                }
            } else {
                *sides_count += 1;
            }
            sides.push((tile_to_check.0 as isize, tile_to_check.1 as isize));
            // *sides_count += 1;
        }
        // }
    };

    let mark_to_check = 'C';
    let mut regions_costs: HashMap<(usize, usize), (char, usize)> = HashMap::new();
    for (_index, (pos, tiles_data)) in regions.into_iter().enumerate() {
        if DEBUG {
            if mark_to_check != columns[pos.0][pos.1] {
                continue;
            }
        }

        let region_tiles: Vec<(usize, usize)> = tiles_data.keys().map(|pos| *pos).collect();
        // Get the highest x and y value for the region.
        let x_max = tiles_data.keys().max_by(|a, b| a.0.cmp(&b.0));
        let y_max = tiles_data.keys().max_by(|a, b| a.1.cmp(&b.1));

        let x_min = tiles_data.keys().min_by(|a, b| a.0.cmp(&b.0));
        let y_min = tiles_data.keys().min_by(|a, b| a.1.cmp(&b.1));

        if DEBUG {
            println!("============");
            println!(
                "For region {:?}, x min: {:?}, y min: {:?}, x max: {:?}, y max: {:?}",
                columns[pos.0][pos.1], x_min, y_min, x_max, y_max
            );
        }

        // for rotation in [
        //     (-DIRECTIONS[0].1, DIRECTIONS[0].0),
        //     ((-DIRECTIONS[1].1, DIRECTIONS[1].0)),
        //     ((-DIRECTIONS[2].1, DIRECTIONS[2].0)),
        //     ((-DIRECTIONS[3].1, DIRECTIONS[3].0)),
        // ] {

        ///////////////////////////////////
        // First direction.
        ///////////////////////////////////
        let direction = DIRECTIONS[3];
        let mut sides: Vec<(isize, isize)> = vec![];
        let mut sides_count: usize = 0;
        for col_index in 0..columns.len() {
            for row_index in 0..columns[col_index].len() {
                // Count the number of non-neighbors in a row. Increment count if there is a gap.
                if let Some(pos) = tiles_data.get(&(col_index, row_index)) {
                    get_position_cost(
                        pos,
                        col_index,
                        row_index,
                        direction,
                        &mut sides,
                        &mut sides_count,
                        &region_tiles,
                    );
                }
            }
        }
        if DEBUG {
            println!("First direction sides count: {}", sides_count);
        }

        // regions_costs
        //     .entry(pos)
        //     .and_modify(|map| {
        //         map.insert(pos, (columns[pos.0][pos.1], sides_count));
        //     })
        //     .or_insert_with(HashMap::new)
        //     .insert(pos, (columns[pos.0][pos.1], sides_count));

        regions_costs
            .entry(pos)
            .and_modify(|(_char, size)| {
                *size += sides_count;
            })
            .or_insert((columns[pos.0][pos.1], sides_count));
        // regions_costs.insert(pos, (columns[pos.0][pos.1], sides_count));
        ///////////////////////////////////
        // Second direction.
        ///////////////////////////////////
        let direction = DIRECTIONS[2];
        let mut sides: Vec<(isize, isize)> = vec![];
        let mut sides_count: usize = 0;
        for col_index in (0..columns.len()).rev() {
            for row_index in (0..columns[col_index].len()).rev() {
                // Count the number of non-neighbors in a row. Increment count if there is a gap.

                if let Some(pos) = tiles_data.get(&(col_index, row_index)) {
                    // println!("Second col_index {}, row_index:{}", col_index, row_index);
                    get_position_cost(
                        pos,
                        col_index,
                        row_index,
                        direction,
                        &mut sides,
                        &mut sides_count,
                        &region_tiles,
                    );
                }
            }
        }
        if DEBUG {
            println!("Second direction sides count: {}", sides_count);
        }
        regions_costs
            .entry(pos)
            .and_modify(|(_char, size)| {
                *size += sides_count;
            })
            .or_insert((columns[pos.0][pos.1], sides_count));

        ///////////////////////////////////
        // Third direction.
        ///////////////////////////////////
        let direction = DIRECTIONS[0];
        let mut sides: Vec<(isize, isize)> = vec![];
        let mut sides_count: usize = 0;
        for col_index in 0..columns.len() {
            for row_index in (0..columns[col_index].len()).rev() {
                // Count the number of non-neighbors in a row. Increment count if there is a gap.

                if let Some(pos) = tiles_data.get(&(col_index, row_index)) {
                    // println!("Third col_index {}, row_index:{}", col_index, row_index);
                    get_position_cost(
                        pos,
                        col_index,
                        row_index,
                        direction,
                        &mut sides,
                        &mut sides_count,
                        &region_tiles,
                    );
                }
            }
        }
        if DEBUG {
            println!("Third direction sides count: {}", sides_count);
        }
        regions_costs
            .entry(pos)
            .and_modify(|(_char, size)| {
                *size += sides_count;
            })
            .or_insert((columns[pos.0][pos.1], sides_count));

        ///////////////////////////////////
        // Forth direction.
        ///////////////////////////////////
        let direction = DIRECTIONS[1];
        let mut sides: Vec<(isize, isize)> = vec![];
        let mut sides_count: usize = 0;
        for col_index in (0..columns.len()).rev() {
            for row_index in 0..columns[col_index].len() {
                // Count the number of non-neighbors in a row. Increment count if there is a gap.

                if let Some(pos) = tiles_data.get(&(col_index, row_index)) {
                    // println!("Forth col_index {}, row_index:{}", col_index, row_index);
                    get_position_cost(
                        pos,
                        col_index,
                        row_index,
                        direction,
                        &mut sides,
                        &mut sides_count,
                        &region_tiles,
                    );
                }
            }
        }
        if DEBUG {
            println!("Forth direction sides count: {}", sides_count);
        }
        regions_costs
            .entry(pos)
            .and_modify(|(_char, size)| {
                *size += sides_count;
            })
            .or_insert((columns[pos.0][pos.1], sides_count));
    }

    let mut total = 0;
    for (mark, cost) in regions_costs.values() {
        println!("Mark {mark} cost: {cost}");
        total += cost;
    }

    println!("Total cost: {total}")

    // println!("\n\n\n\n\n\n\n\n");
    // print_map_animate(
    //     &columns,
    //     map_height,
    //     map_width,
    //     // *sides.index(map_width + 1),
    //     (0, 0),
    //     false,
    //     columns[pos.0][pos.1],
    //     &region_tiles,
    //     &sides,
    // );

    // Iterate over the grid, row by row. for each value
    // }
}
