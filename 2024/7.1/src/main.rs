use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    
    fn apply_operator(a: u64, b: u64, operator: u64) -> u64 {
        // println!("{operator} with {a}, {b}");
        match operator {
            0 => a + b,
            1 => a - b,
            2 => a * b,
            3 => a / b,
            4 => format!("{}{}", a, b).parse::<u64>().unwrap(),
            _ => unreachable!("5th operator not implemented!")
        }
    }
    


    let test = false;
    let file_path = if test {"./test_input.txt"} else {"./input.txt"};
    println!("In file {file_path}");


    fn generate_combinations(numbers: &Vec<u64>, length: usize) -> Vec<Vec<u64>>{
        // Base case: if the length is 0, return an empty combination
        if length == 0 {
            return vec![vec![]];
        }

    let mut combinations = Vec::new();

    // Recursive case: prepend each number to all combinations of the remaining length
    for &number in numbers {
        let sub_combinations = generate_combinations(numbers, length - 1);
        for mut combo in sub_combinations {
            combo.insert(0, number); // Prepend the current number
            combinations.push(combo);
        }
    }

    return combinations


    }


    let mut sum = 0;
    for line in read_to_string(file_path).unwrap().lines() {
        let mut string_values: VecDeque<&str> = line.split_whitespace().collect();
        // println!(" Values {:?}",string_values);
        let correct_answer: u64 = string_values.pop_front().unwrap().replace(":", "").parse().unwrap();
        let values: Vec<u64> = string_values.iter().map(|val | val.parse::<u64>().unwrap()).collect();
        // let val_len = values.len();
        // println!("values.len: {val_len}");
        // // let mut operator_combinations = vec![];
        // let mut operators = vec![];
        // for i in 0..val_len {
        //     operators.push(i);
        // }
        // let mut numbers = vec![];
        // for val in values {
        //     numbers.push(val.parse().unwrap());
        // }

        let operators: Vec<u64> = vec![0,2,4];
        
        let combos = generate_combinations(&operators, values.len()-1);
        // println!("Operator combinations: {:?}", combos);


        // for combo in combos {
        //     println!("Combo: {:?}, values: {:?}", combo, values);
        // }

        // let mut reduce_index = 0;
        // let answer = values.iter().reduce(|acc, val| {
        //     println!("Applying operator {} to {acc} and {val}", operators[reduce_index]);
        //     let answer = apply_operator(*acc, *val, operators[reduce_index]);
        //     reduce_index+=1;
        //     Box::leak(Box::new(answer))
        // }).unwrap();

        for (index, combo) in combos.iter().enumerate() {
            let mut acc = values[0];

            // For each operator in combo
            // Apply to accumulated value and next value
            for (i, operator) in combo.iter().enumerate() {
                acc = apply_operator(acc, values[i+1], *operator)
            }

            if acc == correct_answer {
                sum += acc;
                break
            }
        }
    }

    println!("Sum: {sum}");
    
    





}
