use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
        
    // Free space and files
    // Alternating between file and free space
    // Each file sequence no is it's ID 
    // Move each right-most value to the first free space from the left.
    
    // Step 1. for each char, add the corresponding entry to an array. 
    // Step 2. Sort the array.


    let test = false;
    let file_path = if test {"./test_input.txt"} else {"./input.txt"};
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);


    let mut arr: Vec<String>= vec![];
    let mut char_index = 0;
    for (i, byte) in  reader.bytes().enumerate() {
        
        // let index = i as i32;
        let char = char::from_u32(byte.unwrap() as u32).unwrap();
            if char.is_digit(10) {
                let num = char.to_digit(10).unwrap();
                if i % 2 == 0 {
                    // Is even, is file.
                    for _ in 0..num {
                        let index = format!("{char_index}");
                        // println!("index: {index}");
                        arr.push(index);
                    }
                    char_index +=1;
                } else {
                    for _ in 0..num {
                    // is odd, is free space
                    arr.push(".".to_string());
                    }
                }
            } else {
                eprintln!("Error, char is not a digit!");
            }
    }

    let dot: String = ".".to_string();
    // println!("{:?}", String::from_iter(arr.clone()));
    'outer: for i in 0..arr.len() {
        if arr[i] == dot {
            for ri in (0..arr.len()).rev() {
                if i >= ri {
                    break 'outer;
                }
                if arr[ri] != dot {
                    
                    // println!("{:?} Swap {i} and {ri}", String::from_iter(arr.clone()));
                    arr.swap(i, ri); // Safe mutable access
                    break; // Exit the inner loop after the swap
                }
            }
        }
    }

    let mut sum = 0;

    
    for i in 0..arr.len() {
        let index = i as i64;
        if arr[i] != dot {
            sum = sum + index * arr[i].parse::<i64>().unwrap();
        }
    }
    
    println!("sum: {sum}");


}
