use ::std::fs;
use std::{error::Error, process};

fn main() {
    let res = run(String::from("./input.txt"));

    match res {
        Ok(v) => println!("The biggest elf load is: {v}"),
        Err(e) => {
            println!("Error : {e}");
            process::exit(1);
        }
    }
}

fn run(path: String) -> Result<u32, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let elf_loads = contents.trim().split("\n\n").collect::<Vec<&str>>();
    let mut biggest = 0;
    for load in elf_loads {
        let mut sum_load = 0;
        for num in String::from(load).split("\n").collect::<Vec<&str>>() {
            sum_load = sum_load + num.parse::<u32>().unwrap();
        }
        if sum_load > biggest {
            biggest = sum_load
        }
    }

    Ok(biggest)
}
