use std::{error::Error, fs, process};

fn main() {
    let result = run("input.txt");

    match result {
        Ok(val) => println!("Result: {val}"),
        Err(e) => {
            println!("Error: {e}");
            process::exit(1)
        }
    }
}

fn run(path: &str) -> Result<i32, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;

    let rows = input.trim().split("\n").collect::<Vec<&str>>();
    let mut enemy_total_score = 0;
    let mut my_total_score = 0;
    for row in rows {
        let values = row.trim().split(" ").collect::<Vec<&str>>();

        let enemy_shape = values[0];
        let round_result = values[1];
        let enemy_shape_score = get_shape_score(enemy_shape);
        let my_shape_score = get_my_shape_score(enemy_shape_score, round_result);
        let (enemy_round_score, my_round_score) = get_round_score(enemy_shape_score, my_shape_score);

        my_total_score += my_round_score + my_shape_score;
        enemy_total_score += enemy_round_score + enemy_shape_score;
        
    }

    println!("Enemy total score: {enemy_total_score}");
    println!("My total score: {my_total_score}");

    Ok(my_total_score)
}

fn get_shape_score(shape: &str) -> i32 {
    match shape {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => 0,
    }
}

fn get_round_score(enemy_shape_score: i32, my_shape_score: i32) -> (i32, i32) {
    match (enemy_shape_score, my_shape_score) {
        (1, 1) | (2, 2) | (3, 3) => (3, 3), // Draw
        (1, 2) | (2, 3) | (3, 1) => (0, 6), // I win
        (_, _) => (6, 0),                   // Enemy win
    }
}

fn get_my_shape_score(enemy_shape_score: i32, round_result: &str) -> i32 {
    // Y = draw  3 points
    // X = lose  0 points 
    // Z = win   6 points

    match round_result {
        "X" /* lose */ => {
            
            let mut result = 0;
            for my_shape in [1,2,3]{
                if get_round_score(enemy_shape_score, my_shape) == (6,0){
                    result = my_shape;
                }
            };
            if result == 0{
                println!("Error in X!");
            }
            result
        },
        "Y" /* draw */ =>  {
            let mut result = 0;
            for my_shape in [1,2,3]{
                if get_round_score(enemy_shape_score, my_shape) == (3,3) {
                    result = my_shape;
                }
            };
            if result == 0{
                println!("Error in Y!")
            }
            result

        },
        "Z" /* win */ =>  {
            let mut result = 0;
            for my_shape in [1,2,3]{
                if get_round_score(enemy_shape_score, my_shape) == (0,6) {
                    result=  my_shape;
                }
            };

            
            if result == 0{
                println!("Error in Z!")
            }
            result
        },
        
        _ => 0,
    }
}
