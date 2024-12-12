use std::{collections::HashMap, fs::read_to_string, time::Instant};
use utils::{get_count, DEBUG, ITERATION_COUNT, MAP_SIZE, ONLY_RUN_FIRST};

mod utils {
    use std::collections::HashMap;

    pub const ONLY_RUN_FIRST: bool = false;
    pub const DEBUG: bool = false;
    pub const TEST: bool = false;
    pub const ITERATION_COUNT: usize = 40;
    pub const MAP_SIZE: u128 = 500000;
    pub fn get_count(
        val: u128,
        depth: usize,
        product_map: &HashMap<u128, u128>,
        split_map: &HashMap<u128, (u128, u128)>,
        iteration_count: usize,
    ) -> u128 {
        if DEBUG {
            println!("=--------=====");
        }
        // TODO: If it's the second last iteration, we don't need to calculate the next value.
        if depth == iteration_count {
            if DEBUG {
                println!("Final count 1 for depth {depth} for val {val}")
            }
            return 1;
        }

        if DEBUG {
            println!(
                "val: {val} len is {}",
                ((val as f64).log10().floor() as usize + 1)
            );
        }
        let res = match val {
            0 => {
                // Is zero
                // Some(1u128)
                if DEBUG {
                    println!("Returning 1 for {val}")
                }
                get_count(1u128, depth + 1, product_map, split_map, iteration_count)
            }
            _ if (((val as f64).log10().floor() as usize + 1) % 2) == 0 => {
                // Is even. Split in the middle
                if DEBUG {
                    println!("Splitting {val}")
                }

                let (first, second): (u128, u128) = if split_map.get(&val).is_some() {
                    *split_map.get(&val).unwrap()
                } else {
                    let val_string = val.to_string();
                    let (first, second) = val_string.split_at(val_string.len() / 2);
                    let first = first.parse::<u128>().unwrap();
                    let second = second.parse::<u128>().unwrap();
                    (first, second)
                };

                let count1 = get_count(first, depth + 1, product_map, split_map, iteration_count);
                let count2 = get_count(second, depth + 1, product_map, split_map, iteration_count);
                if DEBUG {
                    println!(
                        "{count1} + {count2} = {} from {first} and {second} from {}",
                        count1 + count2,
                        val
                    )
                }
                return count1 + count2;
            }

            val => {
                if DEBUG {
                    println!("Multiplying {val}")
                }
                let product_val = if product_map.get(&val).is_some() {
                    *product_map.get(&val).unwrap()
                } else {
                    if DEBUG && val < MAP_SIZE {
                        println!("Error. Multiplying value {val}")
                    }
                    val * 2024
                };

                get_count(
                    product_val,
                    // *product_map.get(&val).unwrap_or(&(val * 2024)),
                    depth + 1,
                    product_map,
                    split_map,
                    iteration_count,
                )
            }
        };

        if DEBUG {
            println!("Count from {val} is: {:?}", res);
        }
        return res; // TODO: This is correct each time, but doesn't add up between
    }
}
fn main() {
    let start = Instant::now();
    let file_path = if utils::TEST {
        "./test_input2.txt"
    } else {
        "./input.txt"
    };
    println!("In file {file_path}");

    let mut product_map: HashMap<u128, u128> = HashMap::new();
    let mut split_map: HashMap<u128, (u128, u128)> = HashMap::new();

    for i in 1u128..MAP_SIZE {
        if (((i as f64).log10().floor() as usize + 1) % 2) == 0 {
            // For even numbers, populare the split_map
            if i > 9 {
                let val_string = i.to_string();
                let key = val_string.parse().unwrap();
                let (first, second) = val_string.split_at(val_string.len() / 2);
                let first = first.parse::<u128>().unwrap();
                let second = second.parse::<u128>().unwrap();
                split_map.insert(key, (first, second));
            }
        } else {
            // For uneven numbers, populate the product_map
            product_map.insert(i as u128, (i as u128) * 2024);
        }
    }

    let content = read_to_string(file_path).unwrap();
    let values: Vec<&str> = content.split_whitespace().collect();

    println!("values; {:?}", values);

    for i in 1..=ITERATION_COUNT {
        let iteration_count = i;
        let start_values: Vec<u128> = values
            .iter()
            .map(|val| -> u128 { val.parse::<u128>().unwrap() })
            .collect();

        let mut count = 0;
        for value_index in 0..values.len() {
            if ONLY_RUN_FIRST {
                println!(
                    "⚠️  Warning! Skipping starting val {}",
                    start_values[value_index]
                );
                if value_index > 0 {
                    continue;
                }
            }
            // Iterate through and return count for each start value.
            count += get_count(
                start_values[value_index],
                0,
                &product_map,
                &split_map,
                iteration_count,
            );
            if DEBUG {
                println!("Count for value {} was: {count}", start_values[value_index]);
            }
        }

        let duration = start.elapsed();
        let time_per_iteration =
            ((duration.as_secs_f64() / ITERATION_COUNT as f64) * 1000f64).floor();
        println!(
            "Time elapsed: {:?}s. {}ms per iteration for iteration count. {iteration_count}. Count: {count}",
            duration.as_millis()/1000, time_per_iteration
        );
    }
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

// 197357 too low
