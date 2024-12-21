mod utils;

use std::collections::HashSet;
use utils::{
    add_color, get_cheapest_neighbor, get_distance_to_node, get_neighbor_positions, print_map,
    wait_for_input, Grid, DIRECTIONS, START_DIST,
};

// Implement BFS, breath first algorithm.
fn main() {
    const DEBUG: bool = true;
    const TEST: bool = true;

    let file = if TEST {
        "./test_input.txt"
    } else {
        "input.txt"
    };

    // TODO: Djiekstras algorithm..
    // 0. Prepare all nodes.
    // 1. Create a set of all unvisited nodes.
    // 2. Assign infinity to all nodes except starting node.

    let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
    println!("Reading file {file}");
    let content = std::fs::read_to_string(file).unwrap();

    // Get width a height
    let mut map_height = 0;
    let mut map_width = 0;
    for (i, line) in content.lines().enumerate() {
        map_width += 1;
        for _ in 0..line.len() {
            if i == 0 {
                map_height += 1;
            } else {
                continue;
            }
        }
    }

    println!("Map height: {} and width: {}", map_height, map_width);

    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    let mut map = Grid::new(map_width, map_height);

    for (y_index, val) in content.lines().enumerate() {
        for (x_index, mark) in val.chars().enumerate() {
            // Add node to unvisted set.
            if mark == '#' {
                continue;
            }
            unvisited.insert((x_index, y_index));
            map.update_mark(x_index, y_index, mark);
            match mark {
                'S' => {
                    start_pos = (x_index, y_index);
                    map.update_dist(x_index, y_index, 0);
                }
                'E' => end_pos = (x_index, y_index),
                _ => {}
            }
        }
    }

    // Implement Djikstra
    // let current_pos = start_pos;
    // 0. If unvisited is empty, or current_node == end_pos, break

    if DEBUG {
        println!("Checking {} nodes.", unvisited.len());
    }

    while !unvisited.is_empty() {
        if DEBUG {
            println!("")
        }
        if unvisited
            .iter()
            .all(|pos| map.get_node(*pos).dist == START_DIST)
        {
            println!("All {} unvisited nodes unreachable.", unvisited.len());
            break;
        }

        // 1. Update all unvisited neighbor nodes' distances.
        let current_pos = unvisited
            .iter()
            .min_by_key(|pos| map.get_node(**pos).dist)
            .copied();

        if DEBUG {
            println!("\nCurrent pos is: {:?}", current_pos);
        }

        match current_pos {
            Some(current_pos) => {
                let current_node = map.get_node(current_pos);
                for dir in DIRECTIONS {
                    let neighbor_pos = (
                        current_pos.0 as isize + dir.0,
                        current_pos.1 as isize + dir.1,
                    );
                    if DEBUG {
                        println!("Checking neighbor {:?}", neighbor_pos);
                    }
                    if unvisited.contains(&(neighbor_pos.0 as usize, neighbor_pos.1 as usize)) {
                        let neighbor_node =
                            map.get_node((neighbor_pos.0 as usize, neighbor_pos.1 as usize));
                        let new_dist = current_node.dist + get_distance_to_node();
                        if new_dist < neighbor_node.dist {
                            // If new dist is shorter, update the neighbor node with it
                            // because going through current node is faster
                            if DEBUG {
                                println!(
                                    "Updating neighbor {:?} with new distance: {}",
                                    neighbor_pos, new_dist
                                );
                            }
                            // This is right!
                            map.update_dist(
                                neighbor_pos.0 as usize,
                                neighbor_pos.1 as usize,
                                new_dist,
                            );
                        }
                    }
                }
                unvisited.remove(&current_pos);
            }
            None => {
                println!("Current_node not found. Breaking while loop");
                break;
            }
        }
    }

    println!("map width and height: {}, {}", map_width, map_height);

    let mut current_pos = start_pos;
    let mut path = vec![start_pos];
    while current_pos != end_pos {
        print_map(&map, &path);
        wait_for_input(false);
        let next_pos = get_cheapest_neighbor(current_pos, &map);
        current_pos = next_pos;
        path.push(next_pos);
    }
}
