use grid::*;
use rand::prelude::*;
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::{self, BufReader, Read},
    ops::Index,
    thread,
    time::{self, Instant},
};
#[derive(Clone, Debug)]
struct Cell {
    x: usize,
    y: usize,
    is_path: bool,
    valid_neighbors: Vec<(usize, usize)>,
}
pub fn sleep(count: u64) {
    thread::sleep(time::Duration::from_millis(count));
}
fn safe_access<T>(grid: &Grid<T>, pos: (isize, isize)) -> Option<&T> {
    if pos.0 >= 0 && pos.1 >= 0 && (pos.0 as usize) < grid.rows() && (pos.1 as usize) < grid.cols()
    {
        Some(&grid[(pos.0 as usize, pos.1 as usize)])
    } else {
        None
    }
}

fn move_in_grid<T>(grid: &Grid<T>, pos: (isize, isize), delta: (isize, isize)) -> Option<&T> {
    let new_row = pos.0 + delta.0;
    let new_col = pos.1 + delta.1;
    safe_access(grid, (new_row, new_col))
}

fn is_valid_move(grid: &Grid<char>, pos: (isize, isize), delta: (isize, isize)) -> bool {
    let mark = move_in_grid(grid, pos, delta);
    match mark {
        Some('.') => true,
        Some('E') => true,
        Some('S') => true,
        _ => false,
    }
}
fn wait_for_input(show_instruction: bool) {
    let mut input = String::new();
    if show_instruction {
        println!("Press Enter to continue...");
    }
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}
fn add_color(char: char, color: &str) -> String {
    match color {
        "red" => "\x1b[31m".to_string() + &'o'.to_string() + "\x1b[39m",
        "magenta" => "\x1b[35m".to_string() + &'o'.to_string() + "\x1b[39m",
        "green" => "\x1b[32m".to_string() + &'o'.to_string() + "\x1b[39m",
        "yellow" => "\x1b[33m".to_string() + &'o'.to_string() + "\x1b[39m",
        "blue" => "\x1b[34m".to_string() + &'o'.to_string() + "\x1b[39m",
        _ => char.to_string(),
    }
}

fn print_map(grid: &Grid<char>, overrides: &Vec<((usize, usize), usize)>) {
    for y in 0..grid.rows() {
        let mut line: Vec<String> = vec![];
        for x in 0..grid.cols() {
            let val_to_push = match overrides.iter().find(|pos| pos.0 .0 == x && pos.0 .1 == y) {
                Some(_) => add_color(grid[(x, y)], "red"),
                None => grid[(x, y)].to_string(),
            };
            line.push(val_to_push.to_string());
        }
        println!("{}", line.join(""));
    }
    // wait_for_input(false);
}

pub fn print_map_animate(
    grid: &Grid<char>,
    starting_pos: (usize, usize),
    red_overrides: &Vec<((usize, usize), usize)>,
    blue_overrides: &Vec<((usize, usize), usize)>,
) {
    let map_width = grid.cols();
    let map_height = grid.rows();
    const VIEWPORT_HEIGHT: usize = 30;
    const VIEWPORT_WIDTH: usize = 50;
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

    let mut content: Vec<String> = vec![];
    for y in start_y..end_y {
        for x in start_x..end_x {
            if (x, y) == starting_pos {
                content.push(add_color(grid[(x, y)], "magenta").to_string());
                continue;
            }

            let val_to_push = match blue_overrides
                .iter()
                .find(|pos| pos.0 .0 == x && pos.0 .1 == y)
            {
                Some(_) => add_color(grid[(x, y)], "green"),
                None => match red_overrides
                    .iter()
                    .find(|pos| pos.0 .0 == x && pos.0 .1 == y)
                {
                    Some(_) => add_color(grid[(x, y)], "red"),
                    None => grid[(x, y)].to_string(),
                },
            };
            content.push(val_to_push.to_string());
        }

        content.push("\n".to_string());
    }
    println!("{}", content.join(""));
}

fn remove_neighbor_from_dead_end(
    pos: (usize, usize),
    cell_grid: &mut Grid<Cell>,
    dead_ends: &mut HashMap<(usize, usize), ()>,
) {
    let cell = &mut cell_grid[(pos.0, pos.1)];

    println!("Valid neightbors before {:?}", cell.valid_neighbors);
    let valid_neighbors: Vec<&(usize, usize)> = cell
        .valid_neighbors
        .iter()
        .filter(|n_pos| !dead_ends.contains_key(n_pos))
        .collect();

    println!("Valid neighbors after: {:?}", valid_neighbors);
    if valid_neighbors.len() == 1 {
        println!(
            "Adding additional dead_end {:?}",
            cell.valid_neighbors.first().unwrap()
        );
        // dead_ends.insert(*cell.valid_neighbors.first().unwrap(), ());
        dead_ends.insert(pos, ());
        let check_pos = cell.valid_neighbors.swap_remove(0);
        println!("Check pos: {:?}", check_pos);
        remove_neighbor_from_dead_end(check_pos, cell_grid, dead_ends);
    } else {
        // println!(
        //     "Recursive quit on cell {:?}. Removing neighbor: {:?}",
        //     cell, pos
        // );
        // // Remove cell from neighbors
        // // let new_valid_neighbor
        // cell.valid_neighbors
        //     .retain(|neighbor_pos| neighbor_pos != &pos);
        // println!("Cell after remove neighbor: {:?}", cell)
    }
}

fn get_cheapest_neighbor(
    pos: (usize, usize),
    current_direction: (isize, isize),
    cell_grid: &Grid<Cell>,
    dead_ends: &HashMap<(usize, usize), ()>,
) -> Option<((usize, usize), usize)> {
    let current_cell = &cell_grid[(pos.0, pos.1)];
    let mut cheapest: Option<((usize, usize), usize)> = None;

    // if DEBUG {
    //     println!("Cell valid neighbors: {:?}", current_cell.valid_neighbors);
    // }
    for index in 0..current_cell.valid_neighbors.len() {
        let neighbor_pos = current_cell.valid_neighbors[index];
        if dead_ends.contains_key(&neighbor_pos) {
            // Skip dead ends.
            continue;
        }
        // let len = cell_grid[pos].valid_neighbors.len();
        let neighbor_direction = (
            neighbor_pos.0 as isize - pos.0 as isize,
            neighbor_pos.1 as isize - pos.1 as isize,
        );
        let is_turn = neighbor_direction != current_direction;
        let neighbor_cost: usize = if is_turn { 1001 } else { 1 };

        // if DEBUG {
        //     println!(
        //         " current_directionL {:?}, neighbor_direction: {:?}",
        //         current_direction, neighbor_direction
        //     );
        //     println!("Neighbor {:? } Is turn: {}", neighbor_pos, is_turn);
        // }

        match cheapest {
            Some(cheap_candidate) => {
                // println!("Match cheapest: {:?}");
                if neighbor_cost < cheap_candidate.1 {
                    // if DEBUG {
                    //     println!(
                    //         "Setting cheapest pos and cost {:?}",
                    //         (neighbor_pos, neighbor_cost)
                    //     );
                    // }
                    cheapest = Some((neighbor_pos, neighbor_cost));
                } else if neighbor_cost == cheap_candidate.1 {
                    // Pick one at random here?
                    // println!("Neighbors are of the same cost..");

                    let random_index = rand::thread_rng().gen_range(0..2);
                    // println!("Random integer: {}", random_index);
                    match random_index {
                        0 => {
                            // if random_index == 0, set the new pos as cheapest
                            // println!("Setting {:?} as cheapest", (pos, cost));
                            cheapest = Some((neighbor_pos, neighbor_cost));
                        }

                        _ => {
                            // println!("Keeping {:?} as cheapest", cheapest);

                            // If random_index == 1 Choose the current pos
                        }
                    }
                }
            }
            None => {
                cheapest = {
                    // If no value is set, set current value.
                    Some((neighbor_pos, neighbor_cost))
                }
            }
        };
    }
    // if DEBUG {
    //     println!("Cheapest neighbor is: {:?}", cheapest);
    // }
    match cheapest {
        Some(data) => Some(data),
        None => {
            // if DEBUG {
            //     println!(
            //         "cheapest neighbor not found for position {:?}",
            //         cell_grid[(current_cell.x, current_cell.y)]
            //     );
            //     println!("Dead ends; {:?}", dead_ends);
            // }
            None
        }
    }
}

fn main() -> io::Result<()> {
    let start_program = Instant::now();
    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    const TEST: bool = false;
    const DEBUG: bool = false;
    let file_path = if TEST {
        "./test_input.txt"
    } else {
        "./input.txt"
    };
    println!("In file {file_path}");

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut row_length = 0;
    let mut buffer: [u8; 1] = [0; 1]; // Single-byte buffer
    while reader.read(&mut buffer)? > 0 {
        if buffer[0] == b'\n' {
            break;
        }
        row_length += 1;
    }

    if DEBUG {
        println!("Row Length: {}", row_length);
    }

    let content = read_to_string(file_path)?;
    let content_vec: Vec<char> = content.chars().filter(|&val| val != '\n').collect();
    drop(content);
    let char_grid = Grid::from_vec_with_order(content_vec, row_length, Order::ColumnMajor);
    let mut cells: Vec<Cell> = vec![];
    let mut start_position = (0, 0);
    // Make cells and put in cell_grid.
    for y_row_index in 0..char_grid.cols() {
        for x_col_index in 0..char_grid.rows() {
            let mark = char_grid[(x_col_index, y_row_index)];
            match mark {
                'S' => {
                    // found start
                    start_position = (x_col_index, y_row_index);
                }

                _ => {}
            }
            // Make cell.
            let mut cell = Cell {
                y: y_row_index,
                x: x_col_index,
                is_path: mark == '.',
                valid_neighbors: vec![],
            };
            for delta in DIRECTIONS {
                if (cell.is_path || mark == 'S')
                    && is_valid_move(
                        &char_grid,
                        (x_col_index as isize, y_row_index as isize),
                        delta,
                    )
                {
                    let neighbor_pos = (
                        x_col_index as isize + delta.0,
                        y_row_index as isize + delta.1,
                    );
                    cell.valid_neighbors
                        .push((neighbor_pos.0 as usize, neighbor_pos.1 as usize));
                }
            }

            let mut overrides = vec![(cell.x, cell.y)];
            overrides.extend(cell.valid_neighbors.iter());

            if DEBUG {
                // println!(
                //     "For cell {:?}, Adding {} valid neighbors",
                //     (cell.x, cell.y),
                //     cell.valid_neighbors.len()
                // );
            }
            // let overr = &overrides.iter().map(|pos| (*pos, 0)).collect();
            // print_map_animate(&char_grid, (cell.x, cell.y), overr);
            // wait_for_input(false);
            cells.push(cell);
        }
    }

    // For each cell in grid, if it only has 1 valid neighbor, set it to dead_end.
    // (except for start and end position).
    // Go to all it's neigbors and remove their previous paths too.

    if DEBUG {
        println!("\n\n\n");
    }

    let mut dead_ends: HashMap<(usize, usize), ()> = HashMap::new();
    let mut cell_grid = Grid::from_vec_with_order(cells, row_length, Order::ColumnMajor);

    // Pre-scan grid for dead-ends.
    for _ in 0..20 {
        for y_row_index in 0..char_grid.cols() {
            if y_row_index == 0 || y_row_index == char_grid.size().1 {
                continue;
            }
            for x_col_index in 0..char_grid.rows() {
                if x_col_index == 0 || x_col_index == char_grid.size().0 {
                    continue;
                }

                if x_col_index < 4 && y_row_index > (char_grid.size().1) - 7 {
                    continue;
                }
                let cell = &cell_grid[(x_col_index, y_row_index)];
                println!("Checking cell: {:?}", cell);
                println!("grid size: {:?}", char_grid.size());
                let valid_neighbors: Vec<&(usize, usize)> = cell
                    .valid_neighbors
                    .iter()
                    .filter(|n_pos| !dead_ends.contains_key(n_pos))
                    .collect();
                if valid_neighbors.len() == 1 {
                    println!("adding first dead end: {:?}", (cell.x, cell.y));
                    dead_ends.insert((cell.x, cell.y), ());
                    remove_neighbor_from_dead_end(
                        (x_col_index, y_row_index),
                        &mut cell_grid,
                        &mut dead_ends,
                    );
                }
            }
        }
    }

    // let dead_ends_vec: &Vec<((usize, usize), usize)> =
    //     &dead_ends.iter().map(|(key, _)| (*key, 0)).collect();
    // print_map_animate(&char_grid, (50, 50), dead_ends_vec, dead_ends_vec);
    // println!("Dead ends vec len: {}", dead_ends_vec.len());
    // wait_for_input(true);

    let mut final_path: Vec<((usize, usize), usize)> = vec![];
    let mut final_path_cost = 999999999;
    let mut while_count = 0;

    const LOOP_COUNT: i32 = 10000;
    while while_count < LOOP_COUNT {
        let loop_start = Instant::now();
        // 1. For start position, choose the neighbor with least entropy, i.e least number of valid neighbors.
        let mut path: Vec<((usize, usize), usize)> = vec![];
        let mut grid_copy = cell_grid.clone();
        let mut next_pos = start_position;
        let mut current_direction = DIRECTIONS[1];
        let mut did_backtrack = false;
        let mut i_count: i32 = 0;
        loop {
            let dead_ends_vec = dead_ends.iter().map(|(key, _)| (*key, 0)).collect();
            i_count += 1;

            // println!("Dead ends: {:?}", dead_ends);
            print_map_animate(&char_grid, next_pos, &path, &dead_ends_vec);
            // print_map_animate(&char_grid, (100, 100), &path, &dead_ends_vec);
            // wait_for_input(true);
            sleep(20);
            // let next_cell = &mut grid_copy[next_pos];

            // if DEBUG {
            //     println!("Next cell: {:?}", grid_copy[next_pos]);
            // }

            // TODO: Check if position is E
            if !grid_copy[next_pos].is_path
                && char_grid[(grid_copy[next_pos].x, grid_copy[next_pos].y)] == 'E'
            {
                // if DEBUG {
                //     println!(
                //         "Found E at {:?}",
                //         (grid_copy[next_pos].x, grid_copy[next_pos].y)
                //     );
                // }

                break;
            }

            let cheapest_neighbor =
                get_cheapest_neighbor(next_pos, current_direction, &grid_copy, &dead_ends);
            match cheapest_neighbor {
                Some(cheapest_neighbor) => {
                    // Remove next_pos from neighbords valid_neighbors array

                    // TODO: Check if we're using grid_copy and cell_grid correctly..
                    let neighbors_to_update: Vec<(usize, usize)> = grid_copy[next_pos]
                        .valid_neighbors
                        .iter()
                        .cloned()
                        .collect();

                    for neighbor in neighbors_to_update {
                        // println!("Neighbor: {:?}", neighbor);
                        if let Some(neighbor_cell) = grid_copy.get_mut(neighbor.0, neighbor.1) {
                            neighbor_cell.valid_neighbors.retain(|pos| pos != &next_pos);
                        }
                    }

                    // grid_copy[next_pos].valid_neighbors =
                    grid_copy[next_pos]
                        .valid_neighbors
                        .retain(|pos| pos != &cheapest_neighbor.0);
                    if DEBUG {
                        println!(
                            "Valid neighbors of current_pos {:?} updated to: {:?}, should reove {:?}", next_pos,
                            grid_copy[next_pos].valid_neighbors, cheapest_neighbor.0
                        );
                    }

                    let new_direction = (
                        cheapest_neighbor.0 .0 as isize - next_pos.0 as isize,
                        cheapest_neighbor.0 .1 as isize - next_pos.1 as isize,
                    );
                    current_direction = new_direction;
                    next_pos = cheapest_neighbor.0;
                    path.push(cheapest_neighbor);
                    // if DEBUG {
                    //     println!("New direction: {:?}", new_direction);
                    //     println!("=====Next cell will be: {:?}", grid_copy[next_pos]);
                    // }
                }

                None => {
                    // Backtrack

                    println!("Backtracking.. Path.len: {:?}", path.len());
                    // Insert next_pos to dead_ends
                    did_backtrack = true;

                    // dead_ends.insert(next_pos, ());

                    if path.is_empty() {
                        // let v: Vec<_> = dead_ends.keys().map(|pos| (*pos, 1)).collect();
                        // print_map_animate(&char_grid, next_pos, &v);
                        // wait_for_input(true);

                        // println!("Reset to start position: {:?}", grid_copy[start_position]);
                        next_pos = start_position;
                        current_direction = DIRECTIONS[1];
                        // grid_copy[start_position] = cell_grid[start_position].clone();

                        // path.push((start_position, 0))
                    } else {
                        for _i in 0..path.len() {
                            // print!("backtrack {_i} ");
                            let prev_path = path.pop().unwrap();
                            let cell = &grid_copy[prev_path.0];
                            let original_cell = &cell_grid[prev_path.0];

                            // TODO: Check if it has no valid neighbors
                            // in the dead_ends array combined with the origian
                            // If original cell only has 1 valid neigbor,
                            if !cell.valid_neighbors.is_empty() {
                                next_pos = prev_path.0;
                                path.push(prev_path);
                            }
                        }
                    }
                    // println!("... Restarting.. ?");
                    // If path is empty, reset to inital starting position and direction.

                    if DEBUG {
                        println!(
                            "start pos after backtrack: {:?}, path.len {}",
                            next_pos,
                            path.len()
                        );
                        // println!("{:?}", cell_grid[next_pos]);
                        // print_map(&char_grid, &path);
                        // if i_count > 300 {
                        //     wait_for_input(true);
                        // }
                        // sleep(2000);
                    }

                    // println!("path: {:?}", path);
                }
            }

            if DEBUG {
                // print_map(&char_grid, &path);

                // print_map_animate(&char_grid, next_pos, &path, &dead_ends_vec);

                // if i_count > 300 {
                //     wait_for_input(false);
                // } else {
                //     // sleep(1);
                // }
            }

            if path.len() > 2 && path.index(path.len() - 2).0 == next_pos {
                break;
            }
        }

        while_count += 1;

        if while_count % 1000 == 0 {
            println!("Iteration {while_count}/{LOOP_COUNT}");
            println!("While loop time: {:?}ms", loop_start.elapsed().as_millis());
            // print_map(&char_grid, &path);
            // print_map_animate(&char_grid, next_pos, dead_ends_vec, dead_ends_vec);
        }

        if did_backtrack {
            continue;
        }

        // if DEBUG {
        print_map(&char_grid, &path);
        // wait_for_input(true);
        // }

        let mut cost = 0;
        for pos in &path {
            cost += pos.1;
        }

        if cost < final_path_cost {
            final_path = path;
            final_path_cost = cost;
        }
    }

    // let mut cost = 0;
    // for pos in &final_path {
    //     cost += pos.1;
    // }
    if final_path_cost != 999999999 {
        println!("Finished! Found E in {} iterations!", while_count);
        println!(
            "Path length: {:?}, path cost: {:?}",
            final_path.len(),
            final_path_cost
        );
    } else {
        println!("didnt find anything..");
    }

    let end_program = start_program.elapsed();
    let end_time = match end_program.as_millis() > 1000 {
        true => end_program.as_secs().to_string() + "s",
        false => end_program.as_millis().to_string() + "ms",
    };
    println!("Program duration: {:?}", end_time);
    return Ok(());
}
