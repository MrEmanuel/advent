use std::fs::read_to_string;

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

    // println!("{:?}", first_array);
    // println!("{:?}", second_array);

    // let (first, _) = first_array.split_at(10);
    // let (second, _) = second_array.split_at(10);

    // println!("First part of array 1: {:?}", first);
    // println!("First part of array 2: {:?}", second);

    // println!("Array 1 length: {}", first_array.len());
    // println!("Array 2 length: {}", second_array.len());

    let mut sum: u32 = 0;

    for index in 0..(first_array.len()) {
        let distance = first_array[index].parse::<u32>().unwrap().abs_diff( second_array[index].parse::<u32>().unwrap());
        // println!("Distance: {distance}");
        sum = sum + distance

    }

    println!("Sum: {sum}")
    
    





}
