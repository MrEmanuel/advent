use std::{error::Error, fs, process};

fn main() {
    let result = run("./input.txt");

    match result {
        Ok(val) => println!("Result: {val}"),
        Err(e) => {
            println!("Error: {e}");
            process::exit(1);
        }
    }
}
fn run(path: &str) -> Result<i32, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    let mut count = 0;
    for row in input.trim().split("\n") {
        let (first, second) = match row.split(",").collect::<Vec<&str>>()[..] {
            [first, second] => (
                first.split("-").collect::<Vec<&str>>(),
                second.split("-").collect::<Vec<&str>>(),
            ),
            _ => (vec![], vec![""]),
        };

        let overlaps = check_overlaps(first, second);
        if overlaps {
            count += 1;
        }
    }

    Ok(count)
}

fn check_overlaps(first: Vec<&str>, second: Vec<&str>) -> bool {
    let first_range =
        ((first[0].parse::<i32>().unwrap())..=(first[1].parse().unwrap())).collect::<Vec<i32>>();
    let second_range =
        (second[0].parse::<i32>().unwrap()..=second[1].parse().unwrap()).collect::<Vec<i32>>();

    let mut overlaps = false;
    for num in first_range {
        if second_range.iter().find(|val| **val == num).is_some() {
            overlaps = true
        }
    }

    return overlaps;
}
