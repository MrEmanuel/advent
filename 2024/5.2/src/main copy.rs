use std::collections::HashMap;
use std::{fs::read_to_string, ops::Index};
use rand::thread_rng;
use rand::seq::SliceRandom;



fn main() {
    let test = false;    
    // Step 1: Create an arr 1 - 100 
    // Step 2: Sort the array according to the rules.
        // Step 2.1 For each rule, check that the arr obeys the rules.
        // Step 2.2 If a rule is not fulfilled, move the number in the array and check again.
        // Step 2.3 Repeat until arr is fully sorted.
    // Step 3: Find the indexes for the update row from the array of ordered numbers 
    // Step 4; Place them according to their index. 


    // New method. 
    // Find all rules involving A as lead argument. 
    // Use the smallest position of all tail arguments and put A before that. 
    // Repeat.


    let input_path = match test {
         true => "test_input.txt",
         false => "input.txt"
    };
    let mut moved_count: HashMap<u32, u32> = HashMap::new();  // Keep track of how many times a value has been move.
    
    let updates = read_to_string(input_path).unwrap();
    let mut rules: Vec<(u32,u32)> = vec![];
    let instructions_path = match test {
        true => "./test_instructions.txt",
        false => "./instructions.txt",
    }; //"./instructions.txt";
    let _lines = read_to_string(instructions_path).unwrap();
    for (index, instructions) in  _lines.lines().enumerate() {
        // println!("Instructions: {instructions}" );
        let values: Vec<&str> = instructions.split("|").collect();
        let first_val = values.index(0).parse::<u32>().unwrap();
        moved_count.insert(first_val, 0); // Initialize all rules moved count to 0
        rules.push((first_val,values.index(1).parse::<u32>().unwrap()));

        if index == 1 {
            // DEBUG. Only use a few rules.
            break
        }
        
    }
    // Create array with 100 values. 0 - 100.
    let mut arr: Vec<u32> = (1..=100).collect();

    let mut count = 0;
    let mut iter_count = 0;
    let mut rules_followed = 0;
    let rules_len = rules.len(); 
    
    

    // while !(rules_followed >= rules_len || count >= 10) {
    //     println!("rules followed: {rules_followed}/{rules_len}, count: {count}");
    // rules.shuffle(&mut thread_rng());
        
        // println!("Break while loop? {} or {} equals: {}",rules_followed < rules_len, count < 50, rules_followed < rules_len || count < 50);
    for rule in &rules {
        iter_count += 1;
        // For each rule.. 
        // Get index of each rule number. 
        let first_pos_val = (arr.iter().position(|val| *val == rule.0).unwrap(), rule.0);
        let second_pos_val = (arr.iter().position(|val| *val == rule.1).unwrap(), rule.1);

        

    }

        // if first_pos < second_pos {
        //     // Ok!

        //     // println!("{rules_followed}, count: {count}");
        //     rules_followed += 1;

        //     continue;
        // } else {
            // move first_pos just before second_pos
            // Re-order the array.
            // We know first_pos comes after second pos, so it won't effect it's index.
            
    //         println!("Rule: {}|{}", rule.0, rule.1);
    //         println!("Moving {}, because its after {} from {},{} to ", rule.0, rule.1, first_pos, second_pos-1);
    //         let moved_first_value = arr.remove(first_pos);
    //         // Update moved count for the first number that we moved.
    //         let rule_count = moved_count.get_mut(&rule.0).unwrap();
    //         let new_rule_count = *rule_count + 1;
    //         *rule_count =  new_rule_count;
    //         let mut new_index = second_pos - 1;



    //         arr.insert(new_index,moved_first_value);
           
       

    //         // println!("Rule {}|{} not followed! Starting over at index {iter_count}", rule.0, rule.1);
    //         println!("New arr: {:?}", arr);
    //         println!(">>>>>>>>>> Breaking with {rules_followed} rules followed, {count} <<<<<<<<<<<");
    //         rules_followed = 0;
    //         iter_count = 0;
            
    //         break
    //     }
    // }    
//     count +=1;

// }

println!("Stopped with {rules_followed}/{rules_len} rules followed");    
println!("===========================");
// println!("Arr: {:?}", arr);


// Filter out the values in the updates (input lines) from the sorted array. 
let mut sorted_lines = vec![];


for line in  updates.lines() {
    let arr_clone = arr.clone();
    // println!("Instructions: {instructions}" );
    let values: Vec<u32> = line.split(",").map(|val| val.parse::<u32>().unwrap()).collect();
    
     let sorted_line: Vec<u32> = arr_clone.into_iter().filter(|val| values.contains(val)).collect();
    //  println!("Sorted line: {:?}", sorted_line);
     sorted_lines.push(sorted_line);   
}

let mut sum = 0;

for line in sorted_lines {
    let middle_index = (line.len()-1)/2;
    let add = line.index(middle_index);
    sum += add;
}

println!("Sum: {sum}" );

// println!("Sorted lines: {:?}", sorted_lines);






}
