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

        let fully_contains = check_contains(first, second);
        if fully_contains {
            count += 1;
        }
    }

    Ok(count)
}

fn check_contains(first: Vec<&str>, second: Vec<&str>) -> bool {
    let first_range =
        ((first[0].parse::<i32>().unwrap())..=(first[1].parse().unwrap())).collect::<Vec<i32>>();
    let second_range =
        (second[0].parse::<i32>().unwrap()..=second[1].parse().unwrap()).collect::<Vec<i32>>();
    let mut biggest = 0;
    let mut smallest = 999;

    // 1. construct the maximum range
    for _num in [first, second].concat() {
        let num = _num.parse::<i32>().unwrap();
        if biggest < num {
            biggest = num;
        }

        if smallest > num {
            smallest = num;
        }
    }
    let maximum_range = (smallest..=biggest).collect::<Vec<i32>>();
    // 2. Check if biggest range is exactly the same size as one of the input ranges.
    // If it is, one of the ranges fully contains the other.
    if maximum_range.len() == first_range.len() || maximum_range.len() == second_range.len() {
        true
    } else {
        false
    }
}
