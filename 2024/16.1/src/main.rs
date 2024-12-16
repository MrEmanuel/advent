use grid::*;
use std::{
    fs::{self, read_to_string, File},
    io::{self, BufRead, BufReader, Read},
};
#[derive(Clone, Debug)]
struct Cell {
    row: usize,
    col: usize,
    height: i32,
    valid_neighbors: Vec<(usize, usize)>,
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
        _ => false,
    }
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

    let char_grid = Grid::from_vec(content_vec, row_length);
    println!("Row Length: {}", row_length);

    let mut cells: Vec<Cell> = vec![];
    for row_index in 0..char_grid.rows() {
        for col_index in 0..char_grid.cols() {
            // Make cell.
            let mut cell = Cell {
                col: col_index,
                row: row_index,
                height: 0,
                valid_neighbors: vec![],
            };
            for delta in DIRECTIONS {
                if is_valid_move(&char_grid, (row_index as isize, col_index as isize), delta) {
                    let neighbor_pos = (row_index as isize + delta.0, col_index as isize + delta.1);
                    cell.valid_neighbors
                        .push((neighbor_pos.0 as usize, neighbor_pos.1 as usize));
                }
            }
            cells.push(cell);
        }
    }

    let cell_grid = Grid::from_vec(cells, row_length);

    return Ok(());
    // Get width:

    // let grid = Grid::from_vec(read_to_string(file_path).unwrap(), 10);
    // for char in line.chars() {}
}
