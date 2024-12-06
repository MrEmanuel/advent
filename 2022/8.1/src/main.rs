use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

struct Tree {
    height: i32,
    row: i32,
    column: i32,
}

impl 

fn main() {
    let res = run("./input.txt");
}

fn run(path: &str) -> Result<&str, Box<dyn Error>> {
    let file = File::open(path)?;
    let input = BufReader::new(file);

    for line in input.lines() {
        println!("Line: {}", line.unwrap());

        // Each value is a height.
        //  (height, row, column)
    }
    return Ok("123");
}
