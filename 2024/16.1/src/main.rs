use grid::*;
use rand::prelude::*;
use std::{
    fs::{read_to_string, File},
    io::{self, BufReader, Read},
    ops::Index,
};
#[derive(Clone, Debug)]
struct Cell {
    row: usize,
    col: usize,
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

fn print_map(grid: &Grid<char>, overrides: &Vec<(usize, usize)>) {
    for x in 0..grid.rows() {
        let mut line: Vec<String> = vec![];
        for y in 0..grid.cols() {
            let val_to_push = match overrides.iter().find(|pos| pos.0 == x && pos.1 == y) {
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
    let char_grid = Grid::from_vec(content_vec, row_length);

    println!("Row Length: {}", row_length);
    let mut cells: Vec<Cell> = vec![];
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);
    // Make cells and put in cell_grid.
    for row_index in 0..char_grid.rows() {
        for col_index in 0..char_grid.cols() {
            let mark = char_grid[(row_index, col_index)];
            match mark {
                'S' => {
                    // found start
                    start_position = (row_index, col_index);
                }
                'E' => {
                    //found end
                    end_position = (row_index, col_index);
                }
                _ => {}
            }
            // Make cell.
            let mut cell = Cell {
                col: col_index,
                row: row_index,
                is_path: mark == '.',
                valid_neighbors: vec![],
            };
            for delta in DIRECTIONS {
                if (cell.is_path || mark == 'S')
                    && is_valid_move(&char_grid, (row_index as isize, col_index as isize), delta)
                {
                    let neighbor_pos = (row_index as isize + delta.0, col_index as isize + delta.1);
                    cell.valid_neighbors
                        .push((neighbor_pos.0 as usize, neighbor_pos.1 as usize));
                }
            }

            let mut overrides = vec![(cell.row, cell.col)];
            overrides.extend(cell.valid_neighbors.iter());

            println!("Adding {} valid neighbors", cell.valid_neighbors.len());
            print_map(&char_grid, &overrides);

            cells.push(cell);
        }
    }

    let mut cell_grid = Grid::from_vec(cells, row_length);
    println!(
        "Start pos: {:?},{:?}",
        start_position, cell_grid[start_position]
    );
    println!("end pos: {:?},{:?}", end_position, cell_grid[end_position]);

    // Start implementing Wave Function Collapse Algorithm!

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
                    (cell.row, cell.col)
                );
                None
            }
        }
    };
    // 1. For start position, choose the neighbor with least entropy, i.e least number of valid neighbors.
    let mut path: Vec<(usize, usize)> = vec![];
    let mut grid_copy = cell_grid.clone(); //
    let mut next_pos = start_position;
    let mut iteration_count = 0;
    loop {
        // let next_cell = &mut grid_copy[next_pos];

        if DEBUG {
            println!("Next cell: {:?}", grid_copy[next_pos]);
        }

        // TODO: Check if position is E
        if !grid_copy[next_pos].is_path
            && char_grid[(grid_copy[next_pos].row, grid_copy[next_pos].col)] == 'E'
        {
            println!(
                "Found E at {:?}",
                (grid_copy[next_pos].row, grid_copy[next_pos].col)
            );

            break;
        }

        println!("Next pos is: {:?}", next_pos);
        // Pick a path at random
        let smallest_neighbor = get_smallest_neighbor(next_pos, &grid_copy);
        match smallest_neighbor {
            Some(smallest_neighbor) => {
                // Remove next_pos from neighbords valid_neighbors array

                // TODO: Check if we're using grid_copy and cell_grid correctly..
                let neighbors_to_update: Vec<(usize, usize)> = grid_copy[next_pos]
                    .valid_neighbors
                    .iter()
                    .cloned()
                    .collect();

                for neighbor in neighbors_to_update {
                    if let Some(neighbor_cell) = grid_copy.get_mut(neighbor.0, neighbor.1) {
                        neighbor_cell.valid_neighbors.retain(|pos| pos != &next_pos);
                    }
                }

                // grid_copy[next_pos].valid_neighbors =
                grid_copy[next_pos]
                    .valid_neighbors
                    .retain(|pos| pos != &smallest_neighbor);

                next_pos = smallest_neighbor;
                println!("Smallest neighbor is: {:?}", smallest_neighbor);
                println!("=====Next cell will be: {:?}", grid_copy[next_pos]);
                path.push(next_pos);
            }

            None => {
                println!("backtracking..");

                for _ in 0..path.len() {
                    let prev_path = path.pop().unwrap();
                    let cell = &grid_copy[prev_path];
                    if !cell.valid_neighbors.is_empty() {
                        println!("Backtracking to cell: {:?}", cell);
                        next_pos = prev_path;
                        path.push(next_pos);
                    }
                }

                println!("path: {:?}", path);
                iteration_count += 1;
            }
        }

        print_map(&char_grid, &path);
        println!("Path length: {:?}", path);
        if path.len() > 2 && path.index(path.len() - 2) == &next_pos {
            println!("No movement made.. breaking loop.");
            break;
        }
        wait_for_input(false);
    }

    println!("Found E in {} iterations!", iteration_count);
    return Ok(());
}
