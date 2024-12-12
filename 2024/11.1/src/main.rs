use std::{fs::read_to_string, ops::Index};

use utils::{DEBUG, ITERATION_COUNT};

mod utils {
    use std::io;
    pub const DEBUG: bool = false;
    pub const TEST: bool = false;
    pub const ITERATION_COUNT: u128 = 25;
    pub fn print_columns(arr: Vec<Vec<u128>>) {
        // println!("arr.len {}", arr.len());
        // println!("arr[0].len {}", arr[0].len());
        for x in 0..arr.len() {
            let mut line = "".to_string();
            for y in 0..arr[x].len() {
                let col = &arr[x];

                let val = if y >= col.len() {
                    " "
                } else {
                    &col[y].to_string()
                };
                line += &(val.to_string() + " ");
            }

            println!("{:?}", line);
        }
    }

    pub fn next_line(in_array: &Vec<u128>) -> Vec<u128> {
        let mut new_array: Vec<u128> = vec![];
        for val_index in 0..in_array.len() {
            let val = in_array[val_index];
            let val_string = val.to_string();
            let res: Option<u128> = match val {
                0 => {
                    // Is zero
                    Some(1u128)
                }
                _ if (val_string.len() % 2) == 0 => {
                    // Is even. Split in the middle
                    let (first, second) = val_string.split_at(val_string.len() / 2);
                    new_array.push(first.parse::<u128>().unwrap());
                    new_array.push(second.parse::<u128>().unwrap());
                    None
                }
                val => Some(val * 2024),
            };
            match res {
                Some(val) => new_array.push(val),
                None => {}
            }
        }
        return new_array;
    }

    pub fn wait_for_input(show_instruction: bool) {
        if !DEBUG {
            return;
        }
        let mut input = String::new();
        if show_instruction {
            println!("Press Enter to continue...");
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    }
}
fn main() {
    let file_path = if utils::TEST {
        "./test_input2.txt"
    } else {
        "./input.txt"
    };
    println!("In file {file_path}");

    let content = read_to_string(file_path).unwrap();
    let values: Vec<&str> = content.split_whitespace().collect();

    println!("values; {:?}", values);
    let mut arr: Vec<Vec<u128>> = vec![];
    let mut start_values: Vec<u128> = values
        .iter()
        .map(|val| -> u128 { val.parse::<u128>().unwrap() })
        .collect();

    for _blink_index in 0u128..ITERATION_COUNT {
        // Input
        let new_line = utils::next_line(&start_values);
        start_values = new_line;
    }
    arr.push(start_values.clone());

    if DEBUG {
        println!("Arr: {:?}", arr);
    }
    println!("Arr len: {}", arr.last().unwrap().len());

    // utils::print_columns(arr);
}

// 2021976/999 = 2024, i.e 999 * 2024 = 2021976

/*
0   1   10  99 999
1 2024  1   0   9  9 2021976

1. Blink once.
The first stone, 0, becomes a stone marked 1.
The second stone, 1, is multiplied by 2024 to become 2024.
The third stone, 10, is split into a stone marked 1 followed by a stone marked 0.
The fourth stone, 99, is split into two stones marked 9.
The fifth stone, 999, is replaced by a stone marked 2021976.  999 * 2024 = 2021976
1 2024 1 0 9 9 2021976


Blink twice


 */
