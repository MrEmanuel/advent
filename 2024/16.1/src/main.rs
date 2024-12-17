use grid::*;
use rand::prelude::*;
use std::{
    fs::{read_to_string, File},
    io::{self, BufReader, Read},
    ops::Index,
};
#[derive(Clone, Debug)]
struct Cell {
    x: usize,
    y: usize,
    is_path: bool,
    valid_neighbors: Vec<(usize, usize)>,
}

struct OverridePos {
    pos: (usize, usize),
    // mark: char,
    // color: Option<String>,
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
        "red" => "\x1b[31m".to_string() + &char.to_string() + "\x1b[39m",
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

fn main() -> io::Result<()> {
    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    const TEST: bool = true;
    const DEBUG: bool = true;
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

    let content = read_to_string(file_path)?;
    let content_vec: Vec<char> = content.chars().filter(|&val| val != '\n').collect();
    drop(content);
    let char_grid = Grid::from_vec_with_order(content_vec, row_length, Order::ColumnMajor);

    println!("Row Length: {}", row_length);
    let mut cells: Vec<Cell> = vec![];
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);
    // Make cells and put in cell_grid.
    for y_row_index in 0..char_grid.cols() {
        for x_col_index in 0..char_grid.rows() {
            let mark = char_grid[(x_col_index, y_row_index)];
            match mark {
                'S' => {
                    // found start
                    start_position = (x_col_index, y_row_index);
                }
                'E' => {
                    //found end
                    end_position = (x_col_index, y_row_index);
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

            println!(
                "For cell {:?}, Adding {} valid neighbors",
                (cell.x, cell.y),
                cell.valid_neighbors.len()
            );
            // print_map(&char_grid, &overrides.iter().map(|pos| (*pos, 0)).collect());
            // wait_for_input(false);
            cells.push(cell);
        }
    }
    println!("\n\n\n");

    let cell_grid = Grid::from_vec_with_order(cells, row_length, Order::ColumnMajor);
    // println!(
    //     "Start pos: {:?},{:?}",
    //     start_position, cell_grid[start_position]
    // );
    // println!("end pos: {:?},{:?}", end_position, cell_grid[end_position]);

    // Start implementing Wave Function Collapse Algorithm!

    fn get_cheapest_neighbor(
        pos: (usize, usize),
        current_direction: (isize, isize),
        cell_grid: &Grid<Cell>,
    ) -> Option<((usize, usize), usize)> {
        let current_cell = &cell_grid[(pos.0, pos.1)];
        let mut cheapest: Option<((usize, usize), usize)> = None;

        println!("Cell valid neighbors: {:?}", current_cell.valid_neighbors);
        for index in 0..current_cell.valid_neighbors.len() {
            let neighbor_pos = current_cell.valid_neighbors[index];
            // let len = cell_grid[pos].valid_neighbors.len();
            let neighbor_direction = (
                neighbor_pos.0 as isize - pos.0 as isize,
                neighbor_pos.1 as isize - pos.1 as isize,
            );
            println!(
                " current_directionL {:?}, neighbor_direction: {:?}",
                current_direction, neighbor_direction
            );
            let is_turn = neighbor_direction != current_direction;
            let neighbor_cost: usize = if is_turn { 1001 } else { 1 };

            println!("Neighbor {:? } Is turn: {}", neighbor_pos, is_turn);

            match cheapest {
                Some(cheap_candidate) => {
                    // println!("Match cheapest: {:?}");
                    if neighbor_cost < cheap_candidate.1 {
                        println!(
                            "Setting cheapest pos and cost {:?}",
                            (neighbor_pos, neighbor_cost)
                        );
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

        println!("Cheapest neighbor is: {:?}", cheapest);
        match cheapest {
            Some(data) => Some(data),
            None => {
                println!(
                    "cheapest neighbor not found for position {:?}",
                    (current_cell.x, current_cell.y)
                );
                None
            }
        }
    }

    fn get_smallest_neighbor(
        pos: (usize, usize),
        cell_grid: &Grid<Cell>,
    ) -> Option<(usize, usize)> {
        let cell = &cell_grid[(pos.0, pos.1)];
        let mut smallest: Option<((usize, usize), usize)> = None;

        for index in 0..cell.valid_neighbors.len() {
            let pos = cell.valid_neighbors[index];
            let len = cell_grid[pos].valid_neighbors.len();

            match smallest {
                Some(data) => {
                    if len < data.1 {
                        smallest = Some((pos, len));
                    } else if len == data.1 {
                        // Pick one at random here?
                        println!("Neighbors are of the same size..");

                        let random_index = rand::thread_rng().gen_range(0..2);
                        println!("Random integer: {}", random_index);
                        match random_index {
                            0 => {
                                // if random_index == 0, set the new pos as smallest
                                println!("Setting {:?} as smallest", (pos, len));
                                smallest = Some((pos, len));
                            }

                            _ => {
                                println!("Keeping {:?} as smallest", smallest);

                                // If random_index == 1 Choose the current pos
                            }
                        }
                    }
                }
                None => smallest = Some((pos, len)),
            };
        }
        match smallest {
            Some(data) => Some(data.0),
            None => {
                println!(
                    "Smallest neighbor not found for position {:?}",
                    (cell.x, cell.y)
                );
                None
            }
        }
    };
    // 1. For start position, choose the neighbor with least entropy, i.e least number of valid neighbors.
    let mut path: Vec<((usize, usize), usize)> = vec![];
    let mut grid_copy = cell_grid.clone(); //
    let mut next_pos = start_position;
    let mut iteration_count = 0;
    let mut current_direction = DIRECTIONS[1];
    loop {
        // let next_cell = &mut grid_copy[next_pos];

        if DEBUG {
            println!("Next cell: {:?}", grid_copy[next_pos]);
        }

        // TODO: Check if position is E
        if !grid_copy[next_pos].is_path
            && char_grid[(grid_copy[next_pos].x, grid_copy[next_pos].y)] == 'E'
        {
            println!(
                "Found E at {:?}",
                (grid_copy[next_pos].x, grid_copy[next_pos].y)
            );

            break;
        }

        let cheapest_neighbor = get_cheapest_neighbor(next_pos, current_direction, &grid_copy);
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
                println!(
                    "Valid neighbors of current_pos updated to: {:?}, should reove {:?}",
                    grid_copy[next_pos].valid_neighbors, cheapest_neighbor.0
                );

                let new_direction = (
                    cheapest_neighbor.0 .0 as isize - next_pos.0 as isize,
                    cheapest_neighbor.0 .1 as isize - next_pos.1 as isize,
                );
                current_direction = new_direction;
                next_pos = cheapest_neighbor.0;
                path.push(cheapest_neighbor);
                println!("New direction: {:?}", new_direction);
                println!("=====Next cell will be: {:?}", grid_copy[next_pos]);
            }

            None => {
                println!("backtracking..");

                for _ in 0..path.len() {
                    let prev_path = path.pop().unwrap();
                    let cell = &grid_copy[prev_path.0];
                    println!("Backtracking to cell.. {:?}", cell);

                    if !cell.valid_neighbors.is_empty() {
                        println!("Resetting to cell: {:?}", cell);
                        next_pos = prev_path.0;
                        path.push(prev_path);
                    }
                }
                // If path is empty, reset to inital starting position and direction.
                if path.is_empty() {
                    next_pos = start_position;
                    current_direction = DIRECTIONS[1];
                    // path.push((start_position, 0))
                }

                println!("path: {:?}", path);
                iteration_count += 1;
            }
        }

        print_map(&char_grid, &path);
        println!("Path length: {:?}", path);
        if path.len() > 2 && path.index(path.len() - 2).0 == next_pos {
            println!("No movement made.. breaking loop.");
            break;
        }
        wait_for_input(false);
    }

    let mut cost = 0;
    for pos in &path {
        cost += pos.1;
    }
    println!("Finished! Found E in {} iterations!", iteration_count);
    println!("Path length: {:?}, path cost: {:?}", path.len(), cost);
    return Ok(());
}
