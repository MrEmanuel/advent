use std::{
    collections::{HashMap, VecDeque},
    io, isize,
};

// Implement BFS, breath first algorithm.
fn main() {
    const DEBUG: bool = true;
    const TEST: bool = true;
    type Grid = HashMap<(isize, isize), Pos>;
    #[derive(Eq, Hash, PartialEq, Debug)]
    pub struct Pos {
        x: isize,
        y: isize,
        seen: bool,
    }
    impl Pos {
        fn adjecent(&self, grid: &Grid) -> Vec<(isize, isize)> {
            let mut pos: Vec<(isize, isize)> = vec![];
            for dir in DIRECTIONS {
                let x = self.x + dir.0;
                let y = self.y + dir.1;
                match grid.get(&(x, y)) {
                    Some(p) => {
                        if !p.seen {
                            pos.push((x, y));
                        }
                    }
                    _ => {}
                }
            }

            return pos;
        }
    }

    type Parents = HashMap<(isize, isize), (isize, isize)>;

    fn bfs(start: (isize, isize), end: (isize, isize), grid: &mut Grid) -> Parents {
        let mut queue: VecDeque<(isize, isize)> = VecDeque::from(vec![start]);
        let mut parents: Parents = HashMap::new();
        if DEBUG {
            println!("Running bfs algorithm..");
        }
        while let Some(curr) = queue.pop_front() {
            let current_pos = grid.get(&curr);

            let (x_current, y_current, neighbors) = {
                let pos = match current_pos {
                    Some(pos) => pos,
                    None => continue,
                };
                if (pos.x, pos.x) == end {
                    continue;
                };
                let neighbors = pos.adjecent(&grid);
                (pos.x, pos.y, neighbors)
            };

            for i in 0..neighbors.len() {
                let adjecent_pos = grid.get_mut(&neighbors[i]);
                match adjecent_pos {
                    Some(next) => {
                        if !next.seen {
                            next.seen = true;
                            queue.push_back((next.x, next.y));
                            parents.insert((next.x, next.y), (x_current, y_current));
                        } else {
                        }
                    }
                    None => {}
                }
            }
        }
        return parents;
    }

    fn reconstruct_path(parents: &mut Parents, end: (isize, isize)) -> Vec<(isize, isize)> {
        let mut path = vec![];
        let mut curr = end;
        if DEBUG {
            println!("Reconstructing path..",);
        }
        while let Some(parent) = parents.get(&curr) {
            if DEBUG {
                println!("Pushing parent: {:?}", parent);
            }
            path.push(*parent);
            curr = *parent;
        }
        path.reverse();
        return path;
    }

    fn p1(
        start: Option<(isize, isize)>,
        end: Option<(isize, isize)>,
        positions: &mut Grid,
    ) -> Option<Vec<(isize, isize)>> {
        match (start, end) {
            (Some(start), Some(end)) => {
                if DEBUG {
                    println!("Start: {:?}, End: {:?}", start, end);
                }
                let mut parents = bfs(start, end, positions);
                if DEBUG {
                    println!("bfs produced parents:{:?}", parents);
                }
                let path = reconstruct_path(&mut parents, end);
                if path.len() > 0 {
                    Some(path)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn add_color(char: char, color: &str) -> String {
        match color {
            "red" => "\x1b[31m".to_string() + &'o'.to_string() + "\x1b[39m",
            "magenta" => "\x1b[35m".to_string() + &'o'.to_string() + "\x1b[39m",
            "green" => "\x1b[32m".to_string() + &'o'.to_string() + "\x1b[39m",
            "yellow" => "\x1b[33m".to_string() + &'o'.to_string() + "\x1b[39m",
            "blue" => "\x1b[34m".to_string() + &'o'.to_string() + "\x1b[39m",
            _ => char.to_string(),
        }
    }
    fn wait_for_input(show_instruction: bool) {
        let mut input = String::new();
        if show_instruction {
            println!("Press Enter to continue...");
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    }
    const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut start: Option<(isize, isize)> = None;
    let mut end: Option<(isize, isize)> = None;
    let mut positions: Grid = HashMap::new();
    let mut map_width = 0;
    let mut map_heigh = 0;

    // let start = (1, 1);
    // let end = (3, 12);

    let file = if TEST {
        "./test_input.txt"
    } else {
        "input.txt"
    };

    println!("Reading file {file}");
    let content = std::fs::read_to_string(file).unwrap();
    for (x_index, val) in content.lines().enumerate() {
        if x_index == 0 {
            map_heigh = val.len();
            map_width = val.len();
        }
        for (y_index, char) in val.chars().enumerate() {
            match char {
                'S' => {
                    positions.insert(
                        (x_index as isize, y_index as isize),
                        Pos {
                            x: x_index as isize,
                            y: y_index as isize,
                            seen: true,
                        },
                    );
                    start = Some((x_index as isize, y_index as isize));
                    // queue.push_back((x_index as isize, y_index as isize))
                }
                'E' => {
                    positions.insert(
                        (x_index as isize, y_index as isize),
                        Pos {
                            x: x_index as isize,
                            y: y_index as isize,
                            seen: false,
                        },
                    );
                    end = Some((x_index as isize, y_index as isize));
                }
                '.' => {
                    positions.insert(
                        (x_index as isize, y_index as isize),
                        Pos {
                            x: x_index as isize,
                            y: y_index as isize,
                            seen: false,
                        },
                    );
                }
                _ => {}
            }
        }
    }
    if DEBUG {
        println!("Created positions: {:?}", positions);
    }

    // Print the map.
    let path = p1(start, end, &mut positions).unwrap();

    let mut prev_path = vec![];
    for pos in &path {
        prev_path.push(pos);
        let mut print_me = vec![];
        for (x_index, val) in content.lines().enumerate() {
            for (y_index, char) in val.chars().enumerate() {
                // if pos.0 == (x_index as isize, y_index as isize) {
                if prev_path.contains(&&(x_index as isize, y_index as isize)) {
                    print_me.push(add_color('x', "red"));
                } else {
                    print_me.push(char.to_string());
                }
            }
            print_me.push('\n'.to_string());
        }
        println!("{}", print_me.join(""));
        wait_for_input(false);
    }

    println!("Path is {:?} long", path.len());
    println!("\n\n");
}
