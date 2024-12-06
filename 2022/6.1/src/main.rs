use itertools::Itertools;
use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    process,
};

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let result = run("./input.txt");
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    match result {
        Ok(res) => println!("Result: {}", res),
        Err(e) => {
            println!("Error: {e}");
            process::exit(1);
        }
    }
}

fn run(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut window = VecDeque::new();

    let mut res: usize = 0;

    for (index, char) in reader.bytes().enumerate() {
        // println!("char: {:?}", &char);
        window.push_front(char.unwrap());
        if index > 3 {
            window.pop_back();
            if itertools::equal(window.clone().into_iter().unique(), window.clone()) {
                res = index + 1;
                break;
            }
        }
    }

    return Ok(res);
}

// 1286  your answer is too low
