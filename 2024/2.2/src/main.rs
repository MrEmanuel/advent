use std::fs::read_to_string;

// 2.1
fn main() {
    
    


    let file_path = "./johan_input.txt";
    println!("In file {file_path}");

    // let mut first_array: Vec<String> = Vec::new();
    // let mut second_array: Vec<String> = Vec:: new();


     
let mut safe_count: i32 = 0;
let mut line_count = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        line_count += 1;
        println!("-----------------");
        // println!("line: {line}");


        
        // For each line, check: 
        // 1. Are the levels strictly increasing or decreasing? 
        // 2. Adjecent numbers differ by 1, 2 or 3. 
        let values: Vec<&str> = line.split_whitespace().collect();
        // println!("{:?}", values);

        let mut line_is_safe = false;
        for skip_index in 0..values.len()+1 {
            
            if line_is_safe {
                break
            }

        let mut new_arr = values.clone();

            if skip_index != 0 {
                new_arr.remove(skip_index-1);
            }

            // println!("Check array: {:?}", new_arr);

            let mut sign = 0;

            for (index, val) in new_arr.iter().enumerate() {
                // Check all variations of the array.

                if index == 0 {
                    // println!("Skip on value {val}");
                    continue
                }

                let prev = new_arr[index-1].parse::<i32>().unwrap();
                let curr = val.parse::<i32>().unwrap();
                let diff = curr - prev;

                if diff.abs() > 3 {
                    // println!("Diff between {prev} and {curr} too large ({diff}), breaking!");
                    break
                }


                
                if diff.signum() == 0 {
                    // println!("No diff between {prev} and {curr}, breaking iteration.");
                    break
                }

                if sign == 0 {
                    // If sign of this line is not set, set it.
                    // println!("Setting sign to: {}, and skipping on {val}",diff.signum());
                    sign = diff.signum();
                    continue
                }

           
                


                if sign != diff.signum(){
                    // println!("Wrong sign between {prev} and {curr}, breaking iteration.");
                    break
                }

           



                // println!("Checking val {val} and current diff {diff}");

                

                // When done, set prev_diff
            
                if index == new_arr.len()-1 {

                    line_is_safe = true;
                    safe_count += 1;
                    if new_arr.last().unwrap() != val {
                        println!(">>>>>>>> Error! This is not the last number! <<<<<<<<")

                    }
    
                    println!("Line {} {:?} is safe! Variation {:?}",line_count, line,new_arr);
                    break
                }
                



            }

            // If safe, break

           
        }
   

    }

    println!("safe_count: {safe_count}/{line_count}");

}
