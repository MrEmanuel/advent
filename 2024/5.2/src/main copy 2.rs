use std::collections::HashMap;
use std::{fs::read_to_string, ops::Index};



fn main() {
    let test = false;    
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

println!("Checking array: {:?}",arr);

    while check_again  {
        println!("Checking again.. {:?}", arr);
        // check_again = false;
        let mut rules: Vec<(u32,u32)> = vec![];
        let mut a_map: HashMap<u32, Vec<usize>> = HashMap::new();
        let mut a_position_map:HashMap<u32, usize> = HashMap::new();
        // For each intruction A|B, C|D, E|F
        for  instructions in  instruction_lines.lines() {
            // println!("Instructions: {instructions}" );
            let values: Vec<&str> = instructions.split("|").collect();
            let first_val = values.index(0).parse::<u32>().unwrap();
            let second_val = values.index(1).parse::<u32>().unwrap();

            let first_val_index = arr.iter().position(|val| val == &first_val);
            let second_val_index = arr.iter().position(|val| val == &second_val);

            if first_val_index.is_some() && second_val_index.is_some() {
                // If A and B is in the array, add the rule to the HashMap.
                rules.push((first_val,second_val));
                // Create a new entry with k,v first_val and second_val_index, or add value if key already exists.
                a_map.entry(first_val).or_insert_with(Vec::new).push(second_val_index.unwrap());
                // Add position of A value too, used to compare positions later.
                a_position_map.insert(first_val, first_val_index.unwrap());
            }
        }
            // println!("{:?}", arr);
            // println!("{:?}", a_map);

        // See if rules are fulfilled. 
        // let mut rules_fulfilled = 0;
        let mut all_rules_fulfilled = true;
        let mut arr_changed = false;
        a_position_map.iter().for_each(|(a_val,a_index)| {
            if !arr_changed {
                
            
                // let a_rule_count = a_map.get(a_val).unwrap().len();
                // println!("a_rule_count: {a_rule_count}");
                
                // let mut a_rules_fulfilled = 0;
                let smallest_b_index = a_map.get(a_val).unwrap().iter().min().unwrap();
                if a_index < smallest_b_index {
                    // a_rules_fulfilled +=1;
                    // println!("{a_val} fulfills the rules!")
                } else {
                    // println!("{a_val} has index {a_index}");
                    // println!("{a_val} doesn't fullfill the rules!");
                    // println!("Array before: {:?}", arr);
                    // Move a_val in the arr. 
                    arr.remove(*a_index);
                    arr.insert(*smallest_b_index,*a_val);

                    // TODO: Update position map! 
                    arr_changed = true;


                    all_rules_fulfilled = false;
                    println!("Array after: {:?}", arr);
                    
                }
            }

                // if a_rules_fulfilled == a_rule_count {
                //     println!("All fullfilled for A: {a_val}");
                //     rules_fulfilled += 1;

                // }
            });



            // if rules_fulfilled != a_position_map.len() {
            //         check_again = true;
            // } else {
            //     println!(">>>>  All fullfilled for array: {:?}", arr);
            //     check_again = false;
            // }
            if !all_rules_fulfilled {
                check_again = true;
            } else {
                println!(">>>>  All fullfilled for array: {:?}", arr);
                sorted_arrays.push(arr.clone());
                check_again = false;
            }


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








}
