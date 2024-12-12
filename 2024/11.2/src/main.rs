use std::{collections::HashMap, env, fs::read_to_string, time::Instant};
use utils::{get_count, DEBUG, FINAL_DEPTH, ITERATION_COUNT, ONLY_RUN_VALUE_NO};

mod utils {
    use std::collections::HashMap;
    // pub const ONLY_RUN_FIRST: bool = false;
    pub const DEBUG: bool = false;
    pub const TEST: bool = false;
    pub const ITERATION_COUNT: usize = 40;
    pub const FINAL_DEPTH: usize = 75;
    // pub const MAP_SIZE: u128 = 500000;
    pub const ONLY_RUN_VALUE_NO: usize = 0; // 0 to 7
    pub fn get_count(
        val: u128,
        depth: usize,
        product_map: &mut HashMap<u128, u128>,
        split_map: &mut HashMap<u128, (u128, u128)>,
        iteration_count: usize,
    ) -> u128 {
        if depth == iteration_count {
            return 1;
        }
        let res = match val {
            0 => {
                // Is zero
                if depth == iteration_count - 1 {
                    return 1;
                }
                get_count(1u128, depth + 1, product_map, split_map, iteration_count)
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

                let count1 = get_count(first, depth + 1, product_map, split_map, iteration_count);
                let count2 = get_count(second, depth + 1, product_map, split_map, iteration_count);
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
                    iteration_count,
                )
            }
        };
        return res;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let start_value = args[1].parse::<usize>().unwrap_or(ONLY_RUN_VALUE_NO);
    let start = Instant::now();
    let file_path = if utils::TEST {
        "./test_input2.txt"
    } else {
        "./input.txt"
    };
    println!("In file {file_path}");

    let mut product_map: HashMap<u128, u128> = HashMap::new();
    let mut split_map: HashMap<u128, (u128, u128)> = HashMap::new();

    // for i in 1u128..MAP_SIZE {
    //     if (((i as f64).log10().floor() as usize + 1) % 2) == 0 {
    //         // For even numbers, populare the split_map
    //         if i > 9 {
    //             let val_string = i.to_string();
    //             let key = val_string.parse().unwrap();
    //             let (first, second) = val_string.split_at(val_string.len() / 2);
    //             let first = first.parse::<u128>().unwrap();
    //             let second = second.parse::<u128>().unwrap();
    //             split_map.insert(key, (first, second));
    //         }
    //     } else {
    //         // For uneven numbers, populate the product_map
    //         product_map.insert(i as u128, (i as u128) * 2024);
    //     }
    // }

    let content = read_to_string(file_path).unwrap();
    let values: Vec<&str> = content.split_whitespace().collect();

    println!("values; {:?}", values);

    // for i in 1..=ITERATION_COUNT {
    let final_depth = FINAL_DEPTH;
    let start_values: Vec<u128> = values
        .iter()
        .map(|val| -> u128 { val.parse::<u128>().unwrap() })
        .collect();

    let mut count = 0;
    for value_index in 0..values.len() {
        // if ONLY_RUN_VALUE_NO  {
        //     println!(
        //         "⚠️  Warning! Skipping starting val {}",
        //         start_values[value_index]
        //     );
        //     if value_index > 0 {
        //         continue;
        //     }
        // }

        if value_index != start_value {
            println!(
                "⚠️  Warning! Skipping starting val {}.",
                start_values[value_index]
            );
            continue;
        } else {
            println!(
                "Blinking {FINAL_DEPTH} times for starting val {}",
                start_values[value_index]
            )
        }

        // Iterate through and return count for each start value.
        count += get_count(
            start_values[value_index],
            0,
            &mut product_map,
            &mut split_map,
            final_depth,
        );
        if DEBUG {
            println!("Count for value {} was: {count}", start_values[value_index]);
        }
    }

    let duration = start.elapsed();
    let time_per_iteration = ((duration.as_secs_f64() / ITERATION_COUNT as f64) * 1000f64).floor();
    println!(
            "Time elapsed: {:?}s. {}ms per iteration for iteration count. {final_depth}. Count: {count}",
            duration.as_millis()/1000, time_per_iteration
        );
    // }
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
