use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    const PAINT_BOARD: bool = false;
    let result = run("./input.txt", PAINT_BOARD);

    match result {
        Ok(res) => println!("Result: {res}"),
        Err(e) => {
            println!("Error: {e}",);
            process::exit(1)
        }
    }
}

fn run(path: &str, PAINT_BOARD: bool) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let mut tail_positions_visited = HashSet::new();
    tail_positions_visited.insert((0, 0));

    let mut knots = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    for line in input.lines() {
        let _line = line?;
        let line_opt = _line.trim().split_once(" ");

        let steps_to_add = match line_opt {
            Some(("R", steps)) => vec![(0, 1); steps.parse::<usize>().unwrap()],
            Some(("L", steps)) => vec![(0, -1); steps.parse::<usize>().unwrap()],
            Some(("U", steps)) => vec![(1, 0); steps.parse::<usize>().unwrap()],
            Some(("D", steps)) => vec![(-1, 0); steps.parse::<usize>().unwrap()],
            Some((&_, _)) => {
                println!(">>> Error. None found..");
                vec![(0, 0); 1]
            }

            None => {
                println!(">>> Error. None found..");
                vec![(0, 0); 1]
            }
        };

        for step_to_add in steps_to_add {
            let mut updates: Vec<(i32, i32)> = vec![];

            for (knot_index, knot) in knots.iter().enumerate() {
                if knot_index == 0 {
                    // Update head position
                    updates.push((knot.0 + step_to_add.0, knot.1 + step_to_add.1));
                } else {
                    // Update tail positions
                    let parent_knot = updates[knot_index - 1];
                    let new_child_value = get_child_new_position(knot, parent_knot);

                    match new_child_value {
                        Ok(val) => {
                            updates.push(val);
                        }
                        Err(e) => println!(">> Error! failed to get new position. {e}"),
                    }
                }
            }

            tail_positions_visited.insert(*updates.last().unwrap());
            knots = updates.clone();

            if PAINT_BOARD {
                for row in (0..6).rev() {
                    println!("");
                    for column in 0..6 {
                        let knot_index = updates.iter().position(|knot| *knot == (row, column));
                        match knot_index {
                            index if index.is_some() => print!("{}", index.unwrap()),
                            None => print!("."),
                            Some(_) => print!("."),
                        }
                    }
                }
                println!("");
            }
        }
    }

    return Ok(tail_positions_visited.len());
}

fn get_xy_distance(child_position: &(i32, i32), parent_position: (i32, i32)) -> f32 {
    ((i32::pow(parent_position.0 - child_position.0, 2)
        + i32::pow(parent_position.1 - child_position.1, 2)) as f32)
        .sqrt()
}

fn get_child_new_position(
    child_knot: &(i32, i32),
    parent_knot: (i32, i32),
) -> Result<(i32, i32), Box<dyn Error>> {
    let distance = get_xy_distance(child_knot, parent_knot);
    if distance <= 1.0 {
        return Ok(*child_knot);
    }

    //  if parent changes both row and column, do the same update, unless you're at pos (0,0)

    let same_row = parent_knot.0 == child_knot.0;
    let same_column = parent_knot.1 == child_knot.1;
    if same_column {
        return Ok(update_cell_row_value(parent_knot, child_knot));
    }

    if same_row {
        return Ok(update_cell_column_value(parent_knot, child_knot));
    }

    if distance > 1.5 {
        let (new_row_value, _) = update_cell_row_value(parent_knot, child_knot);
        let (_, new_column_value) = update_cell_column_value(parent_knot, child_knot);
        return Ok((new_row_value, new_column_value));
    } else {
        return Ok(*child_knot);
    }
}

fn update_cell_row_value(parent_knot: (i32, i32), child_knot: &(i32, i32)) -> (i32, i32) {
    let direction = (parent_knot.0 - child_knot.0).signum();
    (child_knot.0 + (1 * direction), parent_knot.1)
}

fn update_cell_column_value(parent_knot: (i32, i32), child_knot: &(i32, i32)) -> (i32, i32) {
    let direction = (parent_knot.1 - child_knot.1).signum();
    (child_knot.0, child_knot.1 + (1 * direction))
}
