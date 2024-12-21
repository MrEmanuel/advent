use std::io;

pub const START_DIST: i32 = 9999;
pub const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, Copy, Clone)]
pub struct Node {
    // x: usize,
    // y: usize,
    pub dist: i32,
    pub mark: char,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub grid: Vec<Vec<Node>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn get_node(&self, (x, y): (usize, usize)) -> Node {
        self.grid[x][y]
    }
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            grid: vec![
                vec![
                    Node {
                        dist: START_DIST,
                        mark: '#'
                    };
                    width
                ];
                height
            ],
            height,
            width,
        }
    }
    pub fn update_dist(&mut self, x: usize, y: usize, dist: i32) {
        self.grid[x][y].dist = dist;
    }
    pub fn update_mark(&mut self, x: usize, y: usize, mark: char) {
        self.grid[x][y].mark = mark;
    }
}

pub fn add_color(input_char: char, color: &str) -> String {
    let color_code = match color {
        "red" => "\x1b[31m",
        "magenta" => "\x1b[35m",
        "green" => "\x1b[32m",
        "yellow" => "\x1b[33m",
        "blue" => "\x1b[34m",
        _ => "",
    };
    let reset_code = "\x1b[39m";
    if color_code.is_empty() {
        input_char.to_string()
    } else {
        format!("{}{}{}", color_code, input_char.to_string(), reset_code)
    }
}
pub fn wait_for_input(show_instruction: bool) {
    let mut input = String::new();
    if show_instruction {
        println!("Press Enter to continue...");
    }
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}

pub fn print_map(grid: &Grid, override_positions: &Vec<(usize, usize)>) {
    let mut content = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            if override_positions.contains(&(x, y)) {
                content.push(add_color('x', "red"));
            } else {
                content.push(grid.grid[x][y].mark.to_string());
            }
        }
        content.push('\n'.to_string());
    }

    print!("{}", content.join(""));
}

pub fn get_neighbor_positions(pos: (usize, usize), grid: &Grid) -> [(usize, usize); 4] {
    DIRECTIONS.map(|dir| {
        (
            (pos.0 as isize + dir.0) as usize,
            (pos.1 as isize + dir.1) as usize,
        )
    })
}

pub fn get_cheapest_neighbor(pos: (usize, usize), map: &Grid) -> (usize, usize) {
    let neighbor_positions = get_neighbor_positions(pos, map);

    return *neighbor_positions
        .iter()
        .min_by_key(|pos| map.get_node(**pos).dist)
        .unwrap();
}

pub fn get_distance_to_node() -> i32 {
    1
}
