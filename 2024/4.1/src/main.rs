use std::{fs::read_to_string, ops::Index};

fn main()  {

    // 1. Create a vector of all rows. 
    // 2. Create a vector of all columnn.
    // 3. Create a vector of all diagonals. (x2)
    // Use the row vectors to create the diagonals.
    // 4. Search through each vector, backwards and forwards to match each word. 

    // (x,y) = (n,m)
    //                                       n,m - n+1,m+1 - n+2,m+2 
    // To create each diagnonal, go from pos 0,0 - 1,1 - 2,2 - 3,3 up to n = line_length
    //                        skip first(!X,!Y -) !n-1, !m-1 

    let file_path = "./input.txt";
    let mut rows:   Vec<Vec<String>> = vec![];
    let mut columns: Vec<Vec<String>> = vec![];
    let mut diagonals: Vec<Vec<String>> = vec![];

    

    let mut line_len = 0;
    for (line_index, line) in read_to_string(file_path).unwrap().lines().enumerate() {
        if line_len == 0 {
            line_len = line.len();
        }
            let mut line_vec= vec![];
            for column_index in 0..line.len(){
                if line_index == 0 {
                    // Create a new vec and add to columns.
                    columns.push(vec![]);
                }

                let character = line.chars().nth(column_index).unwrap();
                columns[column_index].push(character.to_string());
                line_vec.push(character.to_string());
            }

        rows.push(line_vec);        

    }

    // println!("Rows: {:?}", rows);
    // println!("First column: {:?}", columns.index(0));
    // println!("Last column: {:?}", columns.index(line_len-1));


    // let mut first_diagonals = vec![];
    // let mut second_diagonals = vec![];
    // let mut third_diagonals = vec![];
    // let mut forth_diagonals = vec![];
    for index in 0..line_len {
        let mut first_diagonal = vec![];
        let mut second_diagonal = vec![];
        let mut third_diagonal = vec![];
        let mut forth_diagonal = vec![];

        // First diagonal:
        let top_x_start = index;
        let top_y_start = 0;
        let bottom_x_start = index;
        let bottom_y_start = line_len -1;
        for increment in 0..line_len {
            // First diagonal
            let first_x = top_x_start - increment;
            let first_y = top_y_start + increment;
            let first_val = columns.index(first_x).index(first_y).to_string();
            first_diagonal.push(first_val);

            if first_x == 0 || first_y == (line_len -1) {
                break
            }
        } 


        for increment in 0..line_len {
            // Second diagonal
            let second_x = top_x_start + increment;
            let second_y = top_y_start + increment;
            let second_val = columns.index(second_x).index(second_y).to_string();
            second_diagonal.push(second_val);

             if second_x == (line_len -1 ) || second_y == (line_len - 1) {
                break
             }
        }


        for increment in 0..line_len {
            // Third diagonal
            if index == line_len -1 { 
                // Skip first index, because we already have the middle diagonal from first and second diagonals
                continue;
             }
            let third_x = bottom_x_start - increment;
            let third_y = bottom_y_start - increment;
            let third_val = columns.index(third_x).index(third_y).to_string();
            third_diagonal.push(third_val);

             if third_x == 0 || third_y == 0 {
                break
             }
        }

        for increment in 0..line_len {
            // Forth diagonal
            if index == 0 { 
                // Skip first index, because we already have the middle diagonal from first and second diagonals
                continue;
             }
            let forth_x = bottom_x_start + increment;
            let forth_y = bottom_y_start - increment;
            let forth_val = columns.index(forth_x).index(forth_y).to_string();
            forth_diagonal.push(forth_val);

             if forth_x == (line_len - 1) || forth_y == 0 {
                break
             }
        }

        // first_diagonals.push(first_diagonal);
        // second_diagonals.push(second_diagonal);
        // third_diagonals.push(third_diagonal);
        // forth_diagonals.push(forth_diagonal);
        diagonals.push(first_diagonal);
        diagonals.push(second_diagonal);
        diagonals.push(third_diagonal);
        diagonals.push(forth_diagonal);

        // println!("forth diagonal: {:?}", forth_diagonal);
    }
    // println!("Diagonals: {:?}",diagonals);
    // println!("columns: {:?}",columns);
    // println!("rows: {:?}",rows);


    // Search for XMAS & SAMX in rows, columns and diagonals
    let word1 = "XMAS".to_string();
    let word2 = "SAMX".to_string();
    let mut count = 0;


    let mut iter_vec = vec![];
    iter_vec.append(&mut rows);
    iter_vec.append(&mut columns);
    iter_vec.append(&mut diagonals);

    // Columns ok. 3
    // Rows ok. 5 
    // Diagonals.. 
    // First diagonal ok.
    // Second diagonal 
    

    for arr in  iter_vec {
        for (index, char) in arr.clone().into_iter().enumerate() {

            if char == "X" {
                let mut match_count = 0;
                for (char_index,  val) in word1.chars().enumerate() {
                    if arr.len() == index + char_index {
                        break
                    }
                    let match_val = arr.index(index+ char_index).to_string();
                    
                    if val.to_string() == match_val {
                        match_count += 1;
                    }
                }
                if match_count == word1.len() {
                    count +=1;
                    println!("Found {word1} in {:?}, index {index}", arr);
                }
            }

            if char == "S" {
                let mut match_count = 0;
                for (char_index,  val) in word2.chars().enumerate() {
                    if arr.len() == index + char_index {
                        break
                    }
                    let match_val = arr.index(index+ char_index).to_string();
                    let v = val.to_string();
                    if v == match_val {
                        match_count += 1;
                    }
                }
                if match_count == word2.len() {
                    count +=1;

                    println!("Found {word2} in {:?}, index {index}", arr);
                }
            }
        }
    }

    println!("Count: {count}"); // 2500 correct?
       
    }




   


