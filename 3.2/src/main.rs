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

    let mut three_ruggsacks: Vec<&str> = vec![];

    for (index, ruggsack) in input.trim().split("\n").enumerate() {
        three_ruggsacks.push(ruggsack);
        if (index + 1) % 3 == 0 {
            // 1. Find all duplicates in first and second ruggsacks and save in array.
            // 2. Find the duplicate that matches the first array in the thrid ruggsack
            let first_duplicates = find_duplicate(three_ruggsacks[0], three_ruggsacks[1]);
            let last_duplicates = find_duplicate(
                &first_duplicates.iter().collect::<String>(),
                three_ruggsacks[2],
            );

            // if last_duplicates.len() > 1 {
            //     println!("Too many: {:?}", last_duplicates);
            //     return Err(Box::from("Too many duplicates!"));
            // }

            let priority = char_range.find(last_duplicates[0]);
            match priority {
                Some(val) => {
                    total_priority += val + 1;
                }
                None => {}
            }

            three_ruggsacks = vec![]; // Reset to empty vector
        }
        // let (first, second) = ruggsack.split_at(ruggsack.len() / 2);
        // if first.len() != second.len() {
        //     return Err(Box::from("Ruggsack of uneven length!"));
        // }

        // let duplicates = find_duplicate(first, second);
        // if duplicate != String::from("") {
        //     // println!("Duplicate: {}", duplicate)
        // } else {
        //     return Err(Box::from("No duplicate found..."));
        // }
        // let my_char = duplicate
        //     .chars()
        //     .collect::<Vec<char>>()
        //     .first()
        //     .unwrap()
        //     .clone();
        // let priority = char_range.find(my_char);

        // match priority {
        //     Some(val) => {
        //         total_priority += val + 1;
        //     }
        //     None => {}
        // }

        // total_priority += priority;
    }

    return Ok(total_priority);
}

fn find_duplicate(first: &str, second: &str) -> Vec<char> {
    // let first_vec = !vec[first];
    // let second_vec = !vec[second];
    let mut duplicates = vec![];
    for first_val in first.chars() {
        let res = second.chars().find(|second_val| first_val == *second_val);
        match res {
            Some(val) => {
                duplicates.push(val);
            }
            None => {}
        }
    }

    return duplicates;
}
