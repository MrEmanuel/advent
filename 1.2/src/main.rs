use ::std::fs;
use std::{error::Error, process};

fn main() {
    let res = run(String::from("./input.txt"));

    match res {
        Ok(v) => println!("The total caloried for the top 3 elfs are: {v:?}"),
        Err(e) => {
            println!("Error: {e}");
            process::exit(1)
        }
    }
}

fn run(path: String) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let mut all_elf_calories: Vec<i32> = [].to_vec();

    for line in content.trim().split("\n\n").collect::<Vec<&str>>() {
        println!("line: {line}");
        let mut sum = 0;
        for calories in String::from(line).trim().split("\n").collect::<Vec<&str>>() {
            sum += calories.parse::<i32>().unwrap();
        }
        println!("sum: {sum}");
        all_elf_calories.push(sum)
    }

    all_elf_calories.sort();

    let len = all_elf_calories.len();
    let top_three = all_elf_calories[len - 3..len].to_vec();

    Ok(top_three.iter().sum())
}
