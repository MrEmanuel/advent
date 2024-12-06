use std::fs::read_to_string;
use std::collections::HashMap;
fn main() {
    
    
    // Pair up the smallest number in the left list 
    // with the smallest number in the right list
    // then the second-smallest left number with 
    // the second-smallest right number, and so on.

    // Step 1. Read the input in to two lists. 
    // Step 2. Sort the lists in order, smallest first. 
    // Step 3. Calculate absolute distance between numbers. 
    // Step 4. Add all distances. 

    let file_path = "./input.txt";
    println!("In file {file_path}");

    let mut first_array: Vec<String> = Vec::new();
    let mut second_array: Vec<String> = Vec:: new();

    for line in read_to_string(file_path).unwrap().lines() {
        let values: Vec<&str> = line.split_whitespace().collect();
        
        first_array.push(values[0].to_string());
        second_array.push(values[1].to_string())
    }
    first_array.sort();
    second_array.sort();



    // let mut similarity_score: u32 = 0;
    let mut occurrences: HashMap<i32, i32> = HashMap::new();

    // println!("{:?}", first_array);
    // println!("{:?}", second_array);

    for index in 0..(first_array.len()) {
        // let distance = first_array[index].parse::<u32>().unwrap().abs_diff( second_array[index].parse::<u32>().unwrap());
        // println!("Distance: {distance}");
        let value1 = first_array[index].parse::<i32>().unwrap();
        // let mut splitAt = 0;


        for (i, value2) in second_array.iter().enumerate() {
            
            // println!("{value1},{value2}");

            let value2_num =value2.parse::<i32>().unwrap();
            if   value2_num == value1 {

                // println!("increase count of {value1} by 1");
                *occurrences.entry(value1).or_insert(0) += 1;

                if i == second_array.len() {
                    break
                }

                if second_array[i+1].parse::<i32>().unwrap() > value1 {
                    // println!("Split and break at: {}", i);
                    // splitAt = i;
                    break

                }
            }

            if value2_num > value1 {
                // println!("break at: {}", i);
                break
            }

            
        }

        // sum = sum + distance

    }

    // Sum values in the hash map: 
    let mut sum = 0;
    for (key,val) in occurrences {
        // println!("Key: {key}, val: {val}");
        sum += key*val;

    }
    // println!("occurrences: {:?}",occurrences.clone());
    println!("sum: {sum}");
    
    





}
