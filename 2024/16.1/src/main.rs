use std::{
    collections::{HashMap, VecDeque},
    isize,
};

fn main() {
    const DEBUG: bool = false;
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
    ) -> Option<usize> {
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
                    Some(path.len())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut start: Option<(isize, isize)> = None;
    let mut end: Option<(isize, isize)> = None;
    let mut positions: Grid = HashMap::new();

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

    println!("Path is {:?} long", p1(start, end, &mut positions).unwrap())

    // Read all information into a Grid.
    // BFS algorithm.
    // Implicit graph by only storing coordinates and looking up state "on the fly"

    // pub fn adjecent(pos: Pos) -> impl Iterator<Item = Pos> {}

    // First in, first out queue. I.e VecDeque.
    // Seen set, i.e a HashMap.

    // 0. Add start to queue and seen.

    // 1. Pop the queue.
    // 2. If the vertice is the end, stop the loop.
    // 3. Add all its' unseen neighbors to the queue and seen set.

    // Repeat 1-2
}
