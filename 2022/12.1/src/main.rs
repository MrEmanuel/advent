use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new() -> Self {
        Self {
            cells: vec![vec![]],
        }
    }
    fn push(&mut self, cell: Cell) {
        let row_index: usize = cell.row.try_into().unwrap();
        if let Some(_) = self.cells.get(row_index) {
            self.cells[row_index].push(cell);
        } else {
            self.cells.push(vec![]);
            self.push(cell);
        }
    }
    fn get(&'static self, row: i32, col: i32) -> Option<&'static Cell> {
        if let Some::<&Vec<Cell>>(row) = self.cells.get::<usize>(row.try_into().unwrap()) {
            if let Some(cell) = row.get::<usize>(col.try_into().unwrap()) {
                Some(cell)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn populate_grid(&mut self, file: File) {
        for (row, _line) in BufReader::new(file).lines().enumerate() {
            let line = _line.unwrap();
            for (col, _height) in line.bytes().enumerate() {
                self.push(Cell::new(
                    row.try_into().unwrap(),
                    col.try_into().unwrap(),
                    _height.try_into().unwrap(),
                ));
            }
        }
    }

    // fn is_valid_step()

    fn get_cell_height(&'static self, row: i32, col: i32) -> i32 {
        self.get(row, col).unwrap().height
    }
    fn is_valid_step(&'static self, from: (i32, i32), to: (i32, i32)) -> bool {
        let from_height = self.get_cell_height(from.0, from.1);
        let to_height = self.get_cell_height(to.0, to.1);

        to_height - from_height < 2
    }

    fn get_valid_delta_steps(&'static self, row: i32, col: i32) -> Vec<(usize, (i32, i32))> {
        let valid_moves: &Vec<i32> = &self.get(row, col).unwrap().valid_moves;
        valid_moves
            .iter()
            .enumerate()
            .filter_map(|(i, val)| match val {
                0 => Some((i, (1, 0))),  // Up
                1 => Some((i, (0, 1))),  // Right
                2 => Some((i, (-1, 0))), // Down
                3 => Some((i, (0, -1))), // Left
                _ => None,
            })
            .collect()
    }

    fn collapse_cell_and_neighbors(&'static self, cell_row: i32, cell_col: i32) {
        let steps_delta = self.get_valid_delta_steps(cell_row, cell_col);
        // TODO: If steps_delta.len() == 0, break.. and backtrack? Or something..
        for (index, (row_delta, col_delta)) in steps_delta {
            let neighbor_cell_row = cell_row + row_delta;
            let neighbor_cell_col = cell_col + col_delta;
            let is_valid_step =
                self.is_valid_step((cell_row, cell_col), (neighbor_cell_row, neighbor_cell_col));

            if !is_valid_step {
                // collapse cell and neighbor, i.e remove the step and it's inversion from cell and neighbor

                self.get(cell_row, cell_col)
                    .unwrap()
                    .to_owned()
                    .drop_valid_move(index.try_into().unwrap());

                let mut neighbor_cell = self
                    .get(neighbor_cell_row, neighbor_cell_row)
                    .unwrap()
                    .to_owned();
                let neighbor_valid_move = match index {
                    0 => 2, // Up => Down
                    1 => 3, // Right => Left
                    2 => 0, // Down => Up
                    3 => 1, // Left => Right
                    _ => {
                        println!("Error getting neighbor valid move!");
                        process::exit(1)
                    }
                };

                neighbor_cell.drop_valid_move(neighbor_valid_move);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Cell {
    row: i32,
    col: i32,
    height: i32,
    valid_moves: Vec<i32>,
}

impl Cell {
    fn new(row: i32, col: i32, height: i32) -> Self {
        Self {
            row,
            col,
            height,
            valid_moves: vec![0, 1, 2, 3],
        }
    }

    fn drop_valid_move(&mut self, num: i32) {
        self.valid_moves = self
            .valid_moves
            .iter()
            .filter_map(|val| match *val == num {
                true => Some(*val),
                _ => None,
            })
            .collect::<Vec<i32>>();
    }

    fn get_valid_moves(self) -> Vec<i32> {
        // Assume cell and all neighbor cells are collapsed.
        self.valid_moves
    }
}

fn main() {
    let response = run("./input test.txt");

    match response {
        Ok(res) => println!("Result:{res}",),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1)
        }
    }
}

fn run(path: &str) -> Result<i32, Box<dyn Error>> {
    let file = File::open(path)?;

    // let mut starting_cell = Cell::new(0, 0, 96);
    let (active_row, active_col) = (0, 0);
    let mut grid = Grid::new();
    grid.populate_grid(file);

    // grid.get(active_row, active_col).and_then(|cell| );
    // grid.collapse_cell_and_neighbors(active_row, active_col);

    // let cell = grid.get(active_row, active_col);

    return Ok(43);
}

// Wave function collapse algorithm.
// 1. Initiate starting Cell
// 2. Look around, initiating each surrounding cell with corresponding entropy if it doesn't exist.
// Each neighboring cell is an entropy of n or n-1 depending on if moving there is valid or not
// where n is it's previous state. Starting state is 4.
// 3. Return all cells that are valid to step to.
// 4. Among those with lowest entropy, pick a random cell.
// 5. Repeat steps 2 to 5.
