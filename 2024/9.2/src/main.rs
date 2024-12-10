use std::collections::HashMap;
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
    let mut file_sizes = HashMap::new();
    for (i, byte) in  reader.bytes().enumerate() {
        
        // let index = i as i32;
        let char = char::from_u32(byte.unwrap() as u32).unwrap();
            if char.is_digit(10) {
                let num = char.to_digit(10).unwrap();
                let index = format!("{char_index}");
                file_sizes.insert(index, num); // Save size for each index.
                if i % 2 == 0 {
                    // Is even, is file.
                    for _ in 0..num {
                        
                        // println!("index: {index}");
                        arr.push(format!("{char_index}"));
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
    // let mut skip_count = 0;
    // println!("{:?}", String::from_iter(arr.clone()));
    
    // 'outer: for i in 0..arr.len() {
    //     if arr[i] == dot {
    let mut skip_indexes = HashMap::new();

    println!("Arr len before: {}", arr.len());
    println!("{:?}", arr.clone().iter().take(50).cloned().collect::<Vec<String>>().join(""));
    println!("Last 20: {:?}", arr.clone().iter().rev().take(20).cloned().collect::<Vec<String>>());
            for ri in (0..arr.len()).rev() {
                // if skip_count > 0 {
                //     skip_count -=1;
                //     continue;
                // }

                match skip_indexes.get(&ri) {
                     Some(true ) => {
                        continue;
                    }
                    _ => {}

                }
           
                if arr[ri] != dot {
                    let file_len = file_sizes.get(&arr[ri]).unwrap();
                    // println!("{} file len {file_len}", arr[ri]);
                    // Find first empty space the same length as the file.
                    let mut free_size: u32 = 0;
                    for i in 0..arr.len() {
                        if i >= ri {
                            break
                        }
                        // If value == dot, increase free_size
                        if arr[i] == dot {
                            free_size +=1;
                        } else {
                            free_size = 0;
                        }

                        if free_size == *file_len {

                            if skip_indexes.len() > 5 {


                            }
                            // If free_size == file_len, move the file.
                            for decrement in 0..free_size {
                                // Decrement i and ri and swap them.
                                let dec: usize = decrement as usize;
                                skip_indexes.insert(i-dec, true);
                                arr.swap(i-dec, ri-dec); 
                            }
                            break
                        }

                    }
                }
            }
    //     }
    // }


    println!("{:?}", arr.clone().iter().take(50).cloned().collect::<Vec<String>>().join(""));

    println!("Arr len after: {}", arr.len());

    let mut sum = 0;
    for i in 0..arr.len() {
        let index = i as i64;
        if arr[i] != dot {
            sum = sum + index * arr[i].parse::<i64>().unwrap();
        }
    }

    

    
    println!("sum: {sum}");

    // 6486125239162 too high
    // 6486125239162




}
