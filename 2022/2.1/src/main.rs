use std::{error::Error, fs, process};

fn main() {
    let res = run("./input.txt");

    match res {
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

    let mut enemy_score = 0;
    let mut _enemy_total_shape_score = 0;
    let mut my_score = 0;
    for row in rows {
        let round = row.trim().split(" ").collect::<Vec<&str>>();
        let enemy_shape_score = get_shape_score(round[0]);
        let my_shape_score = get_shape_score(round[1]);

        let (enemy_round_score, my_round_score) =
            get_round_score(enemy_shape_score, my_shape_score);

        _enemy_total_shape_score += enemy_shape_score;

        enemy_score += enemy_round_score + enemy_shape_score;
        my_score += my_round_score + my_shape_score;
    }

    // A: Rock
    // B: paper
    // C: Scissors

    // X: Rock
    // Y: Paper
    // Z: Scissors

    // Points
    // Rock: 1
    // Paper: 2
    // Scissor: 3
    // Outcome:  Lose: 0 , Win 6, Draw 3

    // 14488 too high
    println!("Enemy score: {}", enemy_score);
    println!("my score: {}", my_score);

    return Ok(my_score);
}

fn get_shape_score(shape: &str) -> i32 {
    match shape {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => 0,
    }
}

fn get_round_score(enemy_shape: i32, my_shape: i32) -> (i32, i32) {
    match (enemy_shape, my_shape) {
        (1, 1) | (2, 2) | (3, 3) => (3, 3), // Draw
        (1, 2) | (2, 3) | (3, 1) => (0, 6), // I win
        (_, _) => (6, 0),                   // Enemy win
    }
}
