use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    let result = run("./input test.txt");

    match result {
        Ok(res) => println!("Result: {res}"),
        Err(e) => {
            println!("Error: {e}",);
            process::exit(1)
        }
    }
}

fn run(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let input = BufReader::new(file);

    let mut head_positions_visited = vec![(0, 0)];
    let mut tail_positions_visited = HashSet::new();
    tail_positions_visited.insert((0, 0));
    let mut head_latest_position = (0, 0);
    let mut tail_latest_position = (0, 0);
    for line in input.lines() {
        // println!("{:?}", line?);
        let _line = line?;
        let line_opt = _line.trim().split_once(" ");

        // Count all the unique positions that H visited, except the last one.
        match line_opt {
            // (row, column)
            Some(("R", steps)) => {
                println!("Move right {} steps", steps);
                for _ in 0..steps.parse::<i32>()? {
                    let new_head_position = (head_latest_position.0, head_latest_position.1 + 1);

                    head_positions_visited.push(new_head_position);

                    let distance = get_distance(tail_latest_position, new_head_position);

                    // If distance = 2,  move tail 1.
                    if distance > 1.5 {
                        tail_latest_position = head_latest_position;
                        tail_positions_visited.insert(tail_latest_position);
                    }
                    head_latest_position = new_head_position;
                }
            }
            Some(("L", steps)) => {
                println!("Move left {} steps", steps);
                for _ in 0..steps.parse::<i32>()? {
                    let new_head_position = (head_latest_position.0, head_latest_position.1 - 1);
                    head_positions_visited.push(new_head_position);

                    let distance = get_distance(tail_latest_position, new_head_position);

                    if distance > 1.5 {
                        tail_latest_position = head_latest_position;
                        tail_positions_visited.insert(tail_latest_position);
                    }
                    head_latest_position = new_head_position;
                }
            }
            Some(("U", steps)) => {
                println!("Move up {} steps", steps);
                for _ in 0..steps.parse::<i32>()? {
                    let new_head_position = (head_latest_position.0 + 1, head_latest_position.1);
                    head_positions_visited.push(new_head_position); // TODO: Only insert this, based on distance.

                    let distance = get_distance(tail_latest_position, new_head_position);

                    if distance > 1.5 {
                        tail_latest_position = head_latest_position;
                        tail_positions_visited.insert(tail_latest_position);
                    }
                    head_latest_position = new_head_position;
                }
            }
            Some(("D", steps)) => {
                println!("Move down {} steps", steps);
                for _ in 0..steps.parse::<i32>()? {
                    let new_head_position = (head_latest_position.0 - 1, head_latest_position.1);
                    head_positions_visited.push(new_head_position); // TODO: Only insert this, based on distance.

                    let distance = get_distance(tail_latest_position, new_head_position);

                    if distance > 1.5 {
                        tail_latest_position = head_latest_position;
                        tail_positions_visited.insert(tail_latest_position);
                    }
                    head_latest_position = new_head_position;
                }
            }

            _ => println!(">>>Error. No match"),
        }
    }

    println!("tail Positions visited: {:?}", tail_positions_visited);
    return Ok(tail_positions_visited.len());
}

fn get_distance(tails_latest_position: (i32, i32), head_new_position: (i32, i32)) -> f32 {
    let res = ((i32::pow(head_new_position.0 - tails_latest_position.0, 2)
        + i32::pow(head_new_position.1 - tails_latest_position.1, 2)) as f32)
        .sqrt();

    println!(
        "Tail Latest position: {:?}, head new position: {:?}, distance: {:?}",
        tails_latest_position, head_new_position, res
    );
    res
}

// 5735  correct!
