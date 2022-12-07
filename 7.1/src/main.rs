use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

#[derive(Debug)]
struct Dir {
    name: String,
    pub file_sizes: i32,
    pub child_dir_names: Vec<String>,
}

fn main() {
    let result = run("./input.txt");

    match result {
        Ok(res) => println!("Result: {res}"),
        Err(e) => {
            println!("Error: {e}");
            process::exit(1);
        }
    }
}

fn run(path: &str) -> Result<i32, Box<dyn Error>> {
    use std::time::Instant;
    let file = File::open(path)?;
    let input = BufReader::new(file);

    let now = Instant::now();

    let mut visited_dirs: Vec<Dir> = vec![];
    let mut looking_in_new_dir = true;
    let mut depth = 0;
    for _line in input.lines() {
        // startswith & = command.
        // cd X -> change directory to X
        // cd .. -> change directory to level n-1
        // cd /  -> change directory to level 0
        // ls -> print out files and directories
        // 123 abc  means size filename
        // dir e means the current directory contains a directory called e
        // Find the total size of each directory
        let line = _line?;
        // For each directory, sum up all it's file sizes, including files in child directories.
        // Save each directory name and it's size.

        let mut visited_dirs_length = visited_dirs.len();
        if visited_dirs_length < 1 {
            visited_dirs.push(Dir {
                child_dir_names: vec![],
                file_sizes: 0,
                name: String::from("/"),
            })
        }
        visited_dirs_length = visited_dirs.len();
        let latest_dir = &mut visited_dirs[visited_dirs_length - 1];

        match line {
            x if x.starts_with("$ cd ..") => {
                looking_in_new_dir = false;
                depth -= 1;
            }
            x if x.starts_with("$ cd ")
                && ('a'..='z').contains(&x.chars().nth_back(0).unwrap()) =>
            {
                depth += 1;
                looking_in_new_dir = true;

                let dir_name = get_dir_name(x, depth);

                visited_dirs.push(Dir {
                    child_dir_names: vec![],
                    file_sizes: 0,
                    name: dir_name,
                })
            }
            x if x.starts_with("$ ls") && looking_in_new_dir => {
                // let is_new_dir = visited_dirs.contains(current_dir);
            }
            _ if ('0'..='9').contains(&line.chars().nth(0).unwrap()) => {
                let (num, _) = line.split_once(" ").unwrap().clone();
                let size = num.parse::<i32>().unwrap();

                latest_dir.file_sizes = latest_dir.file_sizes + size
            }
            x if x.starts_with("dir ") && looking_in_new_dir => {
                // Add dirs that are included in current dir.
                latest_dir.child_dir_names.push(get_dir_name(x, depth + 1));
            }
            x => {
                // println!("Other: {}", x)
            }
        }
    }

    let mut new_dirs: Vec<Dir> = vec![];
    for dir in visited_dirs.as_slice() {
        let size = count_size_of_dir(dir, &visited_dirs);
        new_dirs.push(Dir {
            child_dir_names: dir.child_dir_names.clone(),
            name: dir.name.clone(),
            file_sizes: size,
        });
    }

    let sizes = new_dirs.iter().map(|dir| match dir.file_sizes {
        s if s <= 100000 => s,
        _ => 0,
    });

    let sum: i32 = sizes.sum();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(sum)
}

fn count_size_of_dir(dir: &Dir, visited_dirs: &Vec<Dir>) -> i32 {
    let mut file_sizes = dir.file_sizes;
    for child_dir_name in &dir.child_dir_names {
        let child_dir = visited_dirs
            .iter()
            .find(|child| child.name == *child_dir_name);

        match child_dir {
            Some(child_dir) => {
                file_sizes += count_size_of_dir(child_dir, visited_dirs);
            }
            None => {
                // println!("No child dir found for dir: {}, {:#?}", dir.name, dir)
            }
        }
    }
    return file_sizes;
}

fn get_dir_name(line: String, depth: i32) -> String {
    line.trim().split(" ").last().unwrap().trim().to_string() + "-" + &depth.to_string()
}
