use std::{collections::HashMap, fs::read_to_string, time::Instant};
use utils::{get_count, DEBUG, FINAL_DEPTH};

mod utils {
    use std::collections::HashMap;

    pub const DEBUG: bool = false;
    pub const TEST: bool = false;
    pub const FINAL_DEPTH: usize = 75;
    pub fn get_count(
        val: u128,
        depth: usize,
        product_map: &mut HashMap<u128, u128>,
        split_map: &mut HashMap<u128, (u128, u128)>,
        depth_val_map: &mut HashMap<(usize, u128), u128>,
        iteration_count: usize,
    ) -> u128 {
        if depth == iteration_count {
            return 1;
        }

        if depth_val_map.contains_key(&(depth, val)) {
            return *depth_val_map.get(&(depth, val)).unwrap();
        }

        let res = match val {
            0 => {
                // Is zero
                if depth == iteration_count - 1 {
                    return 1;
                }
                get_count(
                    1u128,
                    depth + 1,
                    product_map,
                    split_map,
                    depth_val_map,
                    iteration_count,
                )
            }
            _ if (((val as f64).log10().floor() as usize + 1) % 2) == 0 => {
                // Is even length. Split in the middle
                if depth == iteration_count - 1 {
                    return 2;
                }

                let (first, second): (u128, u128) = if split_map.get(&val).is_some() {
                    *split_map.get(&val).unwrap()
                } else {
                    let val_string = val.to_string();
                    let (first, second) = val_string.split_at(val_string.len() / 2);
                    let first = first.parse::<u128>().unwrap();
                    let second = second.parse::<u128>().unwrap();

                    // Add to answers in split_map so we don't have to calculate it again.

                    // println!("Adding val {val} to split_array");
                    split_map.insert(val, (first, second));

                    (first, second)
                };

                let count1 = get_count(
                    first,
                    depth + 1,
                    product_map,
                    split_map,
                    depth_val_map,
                    iteration_count,
                );
                let count2 = get_count(
                    second,
                    depth + 1,
                    product_map,
                    split_map,
                    depth_val_map,
                    iteration_count,
                );
                return count1 + count2;
            }

            val => {
                if depth == iteration_count - 1 {
                    return 1;
                }
                let product_val = if product_map.get(&val).is_some() {
                    *product_map.get(&val).unwrap()
                } else {
                    // println!("Adding val {val} to product_array");
                    product_map.insert(val, val * 2024);
                    val * 2024
                };
                get_count(
                    product_val,
                    depth + 1,
                    product_map,
                    split_map,
                    depth_val_map,
                    iteration_count,
                )
            }
        };

        if !depth_val_map.contains_key(&(depth, val)) {
            depth_val_map.insert((depth, val), res);
        }
        return res;
    }
}
fn main() {
    let file_path = if utils::TEST {
        "./test_input2.txt"
    } else {
        "./input.txt"
    };
    println!("In file {file_path}");

    let mut product_map: HashMap<u128, u128> = HashMap::new();
    let mut split_map: HashMap<u128, (u128, u128)> = HashMap::new();
    let mut deoth_val_map: HashMap<(usize, u128), u128> = HashMap::new();
    let content = read_to_string(file_path).unwrap();
    let values: Vec<&str> = content.split_whitespace().collect();

    println!("",);
    println!("",);
    println!("",);

    let final_depth = FINAL_DEPTH;
    let start_values: Vec<u128> = values
        .iter()
        .map(|val| -> u128 { val.parse::<u128>().unwrap() })
        .collect();

    let mut count = 0;
    let start = Instant::now();
    for value_index in 0..values.len() {
        // Iterate through and return count for each start value.
        count += get_count(
            start_values[value_index],
            0,
            &mut product_map,
            &mut split_map,
            &mut deoth_val_map,
            final_depth,
        );
        if DEBUG {
            println!("Count for value {} was: {count}", start_values[value_index]);
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}ms. Count: {count}", duration.as_millis());
    println!("",);
    println!("",);
    println!("",);
    // }
}
