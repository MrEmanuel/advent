use std::fs::read_to_string;

// 2.1
fn main() {
    
    


    let file_path = "./input.txt";
    println!("In file {file_path}");

    // let mut first_array: Vec<String> = Vec::new();
    // let mut second_array: Vec<String> = Vec:: new();


     
let mut safe_count: i32 = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        // println!("line: {line}");


        
        // For each line, check: 
        // 1. Are the levels strictly increasing or decreasing? 
        // 2. Adjecent numbers differ by 1, 2 or 3. 
        let values: Vec<&str> = line.split_whitespace().collect();
        let mut is_unsafe = false;
        let mut previous_diff: i32 = 0;

        for (index, value) in values.iter().enumerate() {
            if index == 0 {
                continue
            }

            let num1 = value.parse::<i32>().unwrap();
            let num2 = values[index -1].parse::<i32>().unwrap();
            let diff: i32 = num1 - num2;

            if diff.abs() > 3 {
                // println!("Unsafe > 3, {num1}, {num2}");
                is_unsafe = true;
                break
            }

            if diff == 0 {
                // println!("Unsafe diff == 0, {num1}, {num2}");
                is_unsafe = true;
                break
            }
        
            if previous_diff != 0 { 

                if  previous_diff.signum() != diff.signum() {
                    // println!("Unsafe signum, {previous_diff}, {diff}");
                    is_unsafe = true;
                    break

                }
            } else {
                // println!("Skip diff {index}: {diff}");
            }

            previous_diff = diff




        }
        if is_unsafe == false {
            // println!("Safe!");
            safe_count += 1
        } 

        
        // for (index, num) in level_iter.enumerate() {
          
        //     if not_safe { break } 
        //     println!("num: {num}, index: {index}");
        //     let current = num.parse::<i32>().unwrap();
        //     if index == 0 {
        //         previous_num = current;
        //         continue;
        //     }
        //     if previous_num != -1 {

        //         let diff = previous_num - current;
        //         println!("Previous: {previous_num} - current {current} = diff {diff}");
        //         if diff == 0 {
        //             println!("Diff = 0. Not safe index {index}. Breaking");
        //             not_safe = true;
        //             break
        //         }
        //         let diff_abs = diff.abs();
        //         if diff_abs > 3 {
        //             println!("Diff > 3. {}, {} Not safe index {} Breaking",diff_abs, diff, index);
        //             not_safe = true;
        //             break
        //         }

        //         if previous_diff == 0 {
        //             println!("No previous diff. Setting {diff} and Continuing");
        //             previous_diff = diff;
        //             continue 
        //         }

                              

        //         if diff.signum() != previous_diff.signum() {
        //             println!("Different signs in diffs. not safe index {index} Breaking");
        //             not_safe = true;
        //             break
        //         } 


        //     } else {
        //         println!("Error!!!!")
        //     }
        // }

        // if not_safe == false {
        //     // increment not_safe_count
        //     safe_count += 1
        // }

    }


    println!("safe_count: {safe_count}")

    // println!("{:?}", first_array);
    // println!("{:?}", second_array);

    // let (first, _) = first_array.split_at(10);
    // let (second, _) = second_array.split_at(10);

    // println!("First part of array 1: {:?}", first);
    // println!("First part of array 2: {:?}", second);

    // println!("Array 1 length: {}", first_array.len());
    // println!("Array 2 length: {}", second_array.len());


    // for index in 0..(first_array.len()) {
    //     let distance = first_array[index].parse::<u32>().unwrap().abs_diff( second_array[index].parse::<u32>().unwrap());
    //     // println!("Distance: {distance}");
    //     sum = sum + distance

    // }

    
    





}
