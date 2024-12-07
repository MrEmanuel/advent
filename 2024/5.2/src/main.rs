use std::{fs::read_to_string, ops::Index};



fn main() {
    let test = false;    
    let re_order_pages = true; // False for 5.1 and true for 5.2
    // Step 1: Add all rules to an array. 
    // Step 2: For each rule A|B, get the smallest index of all B's associated with A.
    // Step 3: Move A to before the smallest index. 
    // Step 4: Repeat untill all rules are fulfilled.

    


    let input_path = match test {
         true => "test_input.txt",
         false => "input.txt"
    };
    let updates = read_to_string(input_path).unwrap();

    let mut arrays = vec![];
    for line in updates.lines() {
        let values: Vec<u32> = line.split(",").map(|val| val.parse::<u32>().unwrap()).collect();
        arrays.push(values);
    }
    
  
    let instructions_path = match test {
        true => "./test_instructions.txt",
        false => "./instructions.txt",
    }; //"./instructions.txt";
    let instruction_lines = read_to_string(instructions_path).unwrap();
    
    
    

let mut check_again = true;
let mut while_count = 0;

let mut sorted_arrays: Vec<Vec<u32>>= vec![];

for mut arr in arrays {
    while_count = 0;
    check_again= true;

// For each array.. 75,47,61,53,29

// println!("Checking array: {:?}",arr);

    while check_again  {
        if !re_order_pages {
            check_again = false;
        }
        // println!("Checking again.. {:?}", arr);
        // check_again = false;
        let mut rules: Vec<(u32,u32)> = vec![];
        // For each intruction A|B, C|D, E|F
        for  instructions in  instruction_lines.lines() {
            // println!("Instructions: {instructions}" );
            let values: Vec<&str> = instructions.split("|").collect();
            let first_val = values.index(0).parse::<u32>().unwrap();
            let second_val = values.index(1).parse::<u32>().unwrap();
            let first_val_index = arr.iter().position(|val| val == &first_val);
            let second_val_index = arr.iter().position(|val| val == &second_val);

            if first_val_index.is_some() && second_val_index.is_some() {
                // Get all relevant rules for the array
                // If A and B is in the array, add the rule to the HashMap.
                rules.push((first_val,second_val));
            }
        }


        let mut all_rules_fullfilled = true;
     
        // println!("Checking array: {:?}", arr);
        rules.iter().for_each(|(a_value,b_value)| {
            
                // println!("Checking rule: {a_value}|{b_value}");
                // For rule A|B, check if it is fullfilled by the array.
                let a_pos = arr.iter().position(|val| val == a_value).unwrap();
                let b_pos = arr.iter().position(|val| val == b_value).unwrap();

                if a_pos > b_pos {
                    // println!("Rule {a_value}|{b_value} not ok!");
                    all_rules_fullfilled = false;
                    

                    if re_order_pages {
                        let a_val = arr.remove(a_pos);
                        arr.insert(b_pos,a_val);
                        if arr.last().unwrap().to_string() != 0.to_string() {
                            arr.push(0); // Indicate that it was changed.

                        }
                    }

                } else {
                    // println!("Rule {a_value}|{b_value} ok!");

                }
            });
        
        if all_rules_fullfilled {
            // println!(">>>>  All fullfilled for array: {:?}", arr);
            let array_was_corrected = arr.last().unwrap().to_string() == 0.to_string();
            if re_order_pages && array_was_corrected {
                arr.pop(); // Remove the zero at the end.
                println!("Pushing array: {:?}", arr);
                sorted_arrays.push(arr.clone());

            } else if !re_order_pages {
                println!("This shouldnt print in 5.2");
                sorted_arrays.push(arr.clone());
            }
            check_again = false;
        } else {
            // println!(">>>>  Rules not fullfilled for array: {:?}", arr);
            
            if re_order_pages {
                check_again = true;
            }
        }



            // if rules_fulfilled != a_position_map.len() {
            //         check_again = true;
            // } else {
            //     println!(">>>>  All fullfilled for array: {:?}", arr);
            //     check_again = false;
            // }
            // if !all_rules_fulfilled {
            //     check_again = true;
            // } else {
            //     println!(">>>>  All fullfilled for array: {:?}", arr);
            //     sorted_arrays.push(arr.clone());
            //     check_again = false;
            // }


        while_count +=1;

        if while_count > 100 {
            println!("Break after 100 iterations..");
            break
        }

        }
    }


    // println!("Sorted arrays: {:?}", sorted_arrays);

    let mut sum = 0;

    for line in sorted_arrays {
        let middle_index = (line.len()-1)/2;
        let add = line.index(middle_index);
        sum += add;
    }


    println!("Sum: {sum}" );

    // 4257 too low





}
