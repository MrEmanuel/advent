use regex::Regex;
use std::fs::read_to_string;

fn main() {
    const TEST: bool = true;
    const DEBUG: bool = true;
    // Calculate area and perimiter.
    // Area = tile count.
    // Perimiter per block = 4 - neighbour count

    // Go row by row. Find a new starting position. Add it to a map.
    // From that starting position, go row by row.
    // For each matching tile, count the neighbours and collect if they have at least 1.
    // When you encounter a row without any neighbors, break and start over.

    let file_path = if TEST {
        "./test_input.txt"
    } else {
        "./input.txt"
    };
    println!("In file {file_path}");

    struct Data {
        a: (i32, i32),
        b: (i32, i32),
        price: (i32, i32),
    }

    let machines: Vec<Data> = vec![];

    let mut button = "";
    let mut x = 0;
    let mut y = 0;
    for line in read_to_string(file_path).unwrap().lines() {
        let re = match Regex::new(
            r"(?i)(?P<label>[A-Za-z0-9\s]+):\s*X[+=]\s*(?P<X>\d+),\s*Y[+=]\s*(?P<Y>\d+)",
        ) {
            Ok(regex) => regex,
            Err(e) => {
                eprintln!("Invalid regex pattern: {}", e);
                return;
            }
        };

        for cap in re.captures_iter(line) {
            let button = &cap[1];
            let X_str = &cap[2];
            let y_str = &cap[3];
            let x = X_str.parse::<i32>().ok().unwrap();
            let y = y_str.parse::<i32>().ok().unwrap();

            println!("Button: {:?}, x: {:?}, y: {}", button, x, y)
        }
    }
    // let mut chars = line.chars();
    // // let char = chars.nth(7);

    // let a_index = chars.position(|char| char == 'A');
    // let comma_index = chars.position(|char| char == ',');
    // let x_index = chars.position(|char| char == 'X');
    // let y_index = chars.position(|char| char == 'Y');
    // println!(
    //     "{:?},{:?},{:?}, {:?} ",
    //     comma_index, x_index, y_index, a_index
    // );

    //    for char match in line. {
    //         Some('A') => {
    //             println!("A");
    //             // X value is x_index +2 to comma_index
    //             // Y value is y_index +2 to end
    //             let comma = comma_index.unwrap();
    //             let x = x_index.unwrap();
    //             let y = y_index.unwrap();

    //             let mut x_val = "".to_string();
    //             for i in x..comma {
    //                 x_val += &chars.nth(i).unwrap().to_string();
    //             }
    //             println!("x_val: {x_val} ");
    //         }
    //         Some('B') => {
    //             println!("B")
    //             // X value is x_index +2 to comma_index
    //             // Y value is  y_index +2 to end
    //         }
    //         _ => {
    //             println!("X")
    //         }
    //     }
    // }
}
