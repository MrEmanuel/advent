use std::fs::read_to_string;

fn main() {
    let test = true;
    
    // Pair up the smallest number in the left list 
    // with the smallest number in the right list
    // then the second-smallest left number with 
    // the second-smallest right number, and so on.

    // Step 1. Read the input in to two lists. 
    // Step 2. Sort the lists in order, smallest first. 
    // Step 3. Calculate absolute distance between numbers. 
    // Step 4. Add all distances. 
    fn print_map(columns: &Vec<Vec<String>>, map_height: usize, map_width: usize, override_pos: Option<(usize,usize)>, override_string: Option<char>) {
        for y_i in 0..map_height {
            let mut line:Vec<String> = vec![];
            for x_i in 0..map_width {
                let val = columns[x_i][y_i].to_string();
                if override_pos.is_some() && override_string.is_some() && override_pos.unwrap() == (x_i, y_i) {
                    line.push(override_string.unwrap().to_string());

                } else {
                    line.push(val);
                }
            }
            println!("{}", line.join(""));
        }
    }

    let file_path = if test {"./test_input.txt"} else {"./input.txt"};
    println!("In file {file_path}");

    let mut columns: Vec<Vec<String>> = vec![];
    let mut map_height =0;
    let mut map_width =0;

    for (line_index,line) in read_to_string(file_path).unwrap().lines().enumerate() {
        // let values: Vec<&str> = line.split_whitespace().collect();
        if line_index == 0 {
            map_width = line.len();
        }
        for column_index in 0..line.len(){
            if line_index == 0 {
                // Create a new vec and add to columns.
                columns.push(vec![]);
            }

            let character = line.chars().nth(column_index).unwrap();
            columns[column_index].push(character.to_string());
        }
        
    }
    map_height = columns[0].len();


    print_map(&columns, map_height, map_width, None, None);


    
    





}
