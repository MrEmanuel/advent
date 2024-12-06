use std::{error::Error, fs, process};

fn main() {
    let result = run("./input.txt");

    match result {
        Ok(res) => println!("Result: {}", res),
        Err(e) => {
            println!("Error: {e}");
            process::exit(1);
        }
    };
}

fn run(path: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    let (crates, instructions) = input.split_once("\n\n").unwrap();

    let instruction_rows = instructions.trim().split("\n").collect::<Vec<&str>>();

    let crate_rows = crates.split("\n").collect::<Vec<&str>>();

    let mut columns: Vec<Vec<char>> = vec![];
    for _ in (0..crate_rows[0].len()).step_by(4) {
        // Fill the columns vector with column vectors o_O
        columns.push(vec![]);
    }

    for row in crate_rows {
        for index in (0..row.len()).step_by(4) {
            // Grab each crate value
            let start_index = index;
            let end_index = start_index + 3;
            let mut val = row[start_index..end_index].chars();
            let crate_value = &val.nth(1).unwrap();
            if *crate_value != ' ' {
                columns[index / 4].push(crate_value.clone());
            }
        }
    }

    for instruction in instruction_rows {
        let cleaned_instructions = instruction.replace("move ", "");
        let (_crate_count, from_to) = cleaned_instructions.split_once(" from ").unwrap();
        let crate_count = _crate_count.to_string().parse::<usize>().unwrap();

        let (_from, _to) = from_to.split_once(" to ").unwrap();
        let from = _from.to_string().parse::<usize>().unwrap();
        let to = _to.to_string().parse::<usize>().unwrap();

        // 1. pick up crate.
        let from_column = &mut columns[from - 1];
        let _from_index = from_column.iter().position(|crate_val| *crate_val != ' ');
        let from_index = match _from_index {
            Some(val) => val,
            None => {
                println!("Error. No value");
                0
            }
        };

        // Use vec::drain to remove a continuous range of values from vec.
        let crates_to_move = from_column
            .drain(from_index..crate_count)
            .collect::<Vec<char>>();

        columns[to - 1] = [crates_to_move, columns[to - 1].clone()].concat();
    }

    let mut first_columns: Vec<char> = vec![];

    for column in columns {
        first_columns.push(column[0]);
    }

    let answer = first_columns.iter().cloned().collect::<String>();
    Ok(answer)
}
