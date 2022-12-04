use std::{error::Error, fs};

fn main() {
    let result = run("./input.txt");

    match result {
        Ok(val) => println!("Result: {val}"),
        Err(e) => println!("Error: {e}"),
    }
}

fn run(path: &str) -> Result<usize, Box<dyn Error>> {
    let input = fs::read_to_string(path).unwrap();
    let mut total_priority = 0;
    let char_range = [
        ('a'..='z').collect::<Vec<_>>(),
        ('A'..='Z').collect::<Vec<_>>(),
    ]
    .concat()
    .iter()
    .collect::<String>();

    for ruggsack in input.trim().split("\n") {
        let (first, second) = ruggsack.split_at(ruggsack.len() / 2);
        if first.len() != second.len() {
            return Err(Box::from("Ruggsack of uneven length!"));
        }

        let duplicate = find_duplicate(first, second);
        if duplicate != String::from("") {
        } else {
            return Err(Box::from("No duplicate found..."));
        }
        let my_char = duplicate
            .chars()
            .collect::<Vec<char>>()
            .first()
            .unwrap()
            .clone();
        let priority = char_range.find(my_char);

        match priority {
            Some(val) => {
                total_priority += val + 1;
            }
            None => {}
        }

        // total_priority += priority;
    }

    return Ok(total_priority);
}

fn find_duplicate(first: &str, second: &str) -> String {
    // let first_vec = !vec[first];
    // let second_vec = !vec[second];
    let mut duplicate = String::from("");
    for first_val in first.chars() {
        let res = second.chars().find(|second_val| first_val == *second_val);
        match res {
            Some(val) => {
                duplicate = String::from(val);
                break;
            }
            None => {}
        }
    }

    return duplicate;
}
