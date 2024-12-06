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

    let input_path = "./test_input.txt";
    let instructions_path = "./test_instructions.txt";
 
    let input = read_to_string(input_path).unwrap().lines();

    for (index,line) in read_to_string(instructions_path).unwrap().lines().enumerate() {
        println!("Instructions: {line}", )

    }
    





}
