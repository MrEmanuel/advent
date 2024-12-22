mod utils;

use std::collections::HashSet;
use utils::{
    get_cheapest_neighbor, get_distance_and_direction_to, print_map, wait_for_input, Grid,
    DIRECTIONS, START_DIST,
};

// Implement BFS, breath first algorithm.
fn main() {
    const DEBUG: bool = false;
    const TEST: bool = false;

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
                    map.update_dist(x_index, y_index, 0f64);
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
        println!("End position is: {:?}", end_pos);
    }

    // let mut path: Vec<(usize, usize)> = vec![];
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

        let current_pos = unvisited
            .iter()
            .min_by_key(|pos| map.get_node(**pos).dist as i32)
            .copied();

        if DEBUG {
            println!("\nCurrent pos is: {:?}", current_pos);
        }

        match current_pos {
            Some(current_pos) => {
                if current_pos == end_pos {
                    if DEBUG {
                        println!("\nEnd position found! Breaking!");
                    }
                    break;
                }

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

                        let (dist, new_direction) = get_distance_and_direction_to(
                            (current_pos.0 as isize, current_pos.1 as isize),
                            neighbor_pos,
                            current_node.direction,
                        );
                        let new_dist = current_node.dist + dist;
                        if new_dist < neighbor_node.dist {
                            // If new dist is shorter, update the neighbor node with it
                            // because going through current node is faster
                            if DEBUG {
                                println!(
                                    "From direction {:?}: Updating neighbor {:?} with new distance: {}",
                                    current_node.direction,neighbor_pos, new_dist
                                );
                                print_map(
                                    &map,
                                    &vec![
                                        (neighbor_pos.0 as usize, neighbor_pos.1 as usize),
                                        current_pos,
                                    ],
                                    false,
                                );
                                wait_for_input(false);
                            }
                            // This is right!
                            map.update_dist(
                                neighbor_pos.0 as usize,
                                neighbor_pos.1 as usize,
                                new_dist,
                            );
                            map.update_direction(
                                neighbor_pos.0 as usize,
                                neighbor_pos.1 as usize,
                                new_direction,
                            );
                        }
                    }
                }
                unvisited.remove(&current_pos);
                // path.push(current_pos);
            }
            None => {
                println!("Current_node not found. Breaking while loop");
                break;
            }
        }
    }

    let mut current_pos = end_pos;
    let mut path = vec![end_pos];
    while current_pos != start_pos {
        let next_pos = get_cheapest_neighbor(current_pos, &map);
        current_pos = next_pos;
        path.push(next_pos);
    }

    if DEBUG {
        for step in path.iter().rev() {
            print_map(&map, &vec![*step], false);
            println!("Position cost: {}", map.get_node(*step).dist);
            wait_for_input(false);
        }
    }

    println!(
        "Cost from start {:?} to reach the goal at {:?} is {}",
        start_pos,
        end_pos,
        map.get_node(end_pos).dist
    )
}
