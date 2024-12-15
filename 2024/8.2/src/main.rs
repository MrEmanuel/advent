use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    io, isize,
};

fn main() {
    let test = false;
    const DEBUG: bool = false;
    const EMPTY_CHAR: char = '.';

    // Antennas.
    // Map all antennas by their type (char).
    // For each antenna, find any other on their diagonal.
    //

    fn print_map(
        columns: &Vec<Vec<char>>,
        map_height: isize,
        map_width: isize,
        override_pos: Option<&Vec<(usize, usize)>>,
        override_string: Option<char>,
        // active_pos: Option<(usize, usize)>,
    ) {
        for y_i in 0..map_height as usize {
            let mut line: Vec<String> = vec![];
            for x_i in 0..map_width as usize {
                let val = columns[x_i][y_i].to_string();

                match override_pos {
                    Some(override_pos) => {
                        if override_pos.contains(&(x_i, y_i)) {
                            if val != EMPTY_CHAR.to_string() {
                                let val = "\x1b[31m".to_string() + &val + "\x1b[39m";

                                line.push(val);
                            } else {
                                let val = "\x1b[31m".to_string()
                                    + &override_string.unwrap().to_string()
                                    + "\x1b[39m";
                                line.push(val);
                            }
                        } else {
                            line.push(val);
                        }
                        continue;
                    }
                    None => {}
                }

                line.push(val);
            }
            println!("{}", line.join(""));
        }
    }

    let file_path = if test {
        "./test_input.txt"
    } else {
        "./input.txt"
    };
    println!("In file {file_path}");

    // const DIRECTIONS: [(isize, isize); 4] = [(-1, -1), (1, 1), (1, -1), (-1, 1)];
    let mut columns: Vec<Vec<char>> = vec![];
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut map_height: isize = 0;
    let mut map_width: isize = 0;

    fn wait_for_input(show_instruction: bool) {
        if !DEBUG {
            return;
        }
        let mut input = String::new();
        if show_instruction {
            println!("Press Enter to continue...");
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    }

    fn is_within_bounds(x: isize, y: isize, map_height: usize, map_width: usize) -> bool {
        let x_ok = x >= 0 && x < map_width as isize;
        let y_ok = y >= 0 && y < map_height as isize;
        return y_ok && x_ok;
    }

    // y: Line index
    // x: column index
    for (line_index, line) in read_to_string(file_path).unwrap().lines().enumerate() {
        // let values: Vec<&str> = line.split_whitespace().collect();
        if line_index == 0 {
            map_width = line.len() as isize;
        }
        for column_index in 0..line.len() {
            if line_index == 0 {
                // Create a new vec and add to columns.=-
                columns.push(vec![]);
            }

            let character = line.chars().nth(column_index).unwrap();
            // let str = character.to_string();
            columns[column_index].push(character);
            // Add to antennas map.
            if character != EMPTY_CHAR {
                antennas
                    .entry(character)
                    .and_modify(|vec| {
                        vec.push((column_index as isize, line_index as isize));
                    })
                    .or_insert(vec![(column_index as isize, line_index as isize)]);
            }
        }
    }
    map_height = columns[0].len() as isize;

    print_map(&columns, map_height, map_width, None, None);

    if DEBUG {
        println!("{:?}\n\n\n", antennas);
    }

    let mut nodes_to_print: Vec<(usize, usize)> = vec![];

    let mut nodes: HashSet<(usize, usize)> = HashSet::new();
    for (_mark, positions) in antennas {
        // For each position, get the vector to all the other positions.
        // Place nodes, and keep them if they are within the map.
        for pos1_index in 0..positions.len() as isize {
            let pos1 = positions[pos1_index as usize];
            for pos2_index in (0..positions.len() as isize).rev() {
                let pos2 = positions[pos2_index as usize];
                if pos1 != pos2 {
                    // Get vector.
                    let distance = ((pos1.0 - pos2.0), (pos1.1 - pos2.1));
                    let node2 = (pos1.0 + distance.0, pos1.1 + distance.1);
                    let node1 = (pos1.0 - distance.0, pos1.1 - distance.1);
                    println!(
                        "pos1: {:?}, pos2: {:?}, Distance: {:?}, New nodes: {:?}, {:?}",
                        pos1, pos2, distance, node2, node1
                    );
                    // println!("Distance between {:?} and {:?}: {:?}", pos1, pos2, distance);
                    // Add and subtract distances from respective point to get nodes.
                    // If a node is within the grid, add it.

                    // It is "distance" from pos1 to pos2.
                    // Get nodes by doing pos2+distance and pos1 - distance.

                    // if node1.0 >= 0 && node1.0 <= map_width && node1.1 >= 0 && node1.1 <= map_height
                    // {
                    //     // add node to vec.
                    //     columns[node1.0 as usize][node1.1 as usize];
                    //     nodes.insert((node1.0 as usize, node1.1 as usize));
                    //     //
                    //     nodes_to_print.push((node1.0 as usize, node1.1 as usize));
                    // }

                    for increment in 0..map_height {
                        let new_distance = (distance.0 * increment, distance.1 * increment);
                        println!("New distance: {:?}", new_distance);
                        let node2 = (pos1.0 + new_distance.0, pos1.1 + new_distance.1);
                        let node1 = (pos1.0 - new_distance.0, pos1.1 - new_distance.1);

                        for node in [node1, node2] {
                            let is_within_map = is_within_bounds(
                                node.0,
                                node.1,
                                map_height as usize,
                                map_width as usize,
                            );

                            if is_within_map {
                                // add node to vec.
                                columns[node.0 as usize][node.1 as usize];
                                println!("New node at: {:?}", node);
                                let size1 = nodes.len();
                                nodes.insert((node.0 as usize, node.1 as usize));
                                nodes_to_print.push((node.0 as usize, node.1 as usize));
                                let size2 = nodes.len();

                                if DEBUG && size1 != size2 {
                                    print_map(
                                        &columns,
                                        map_height,
                                        map_width,
                                        Some(&nodes_to_print),
                                        Some('*'),
                                    );
                                    wait_for_input(false);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut node_vec: Vec<(usize, usize)> = vec![];
    let count = nodes.len();
    for node in nodes {
        node_vec.push(node);
    }

    print_map(&columns, map_height, map_width, Some(&node_vec), Some('*'));
    println!("Antennas count: {}", count);
}
