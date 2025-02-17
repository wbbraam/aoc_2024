use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use std::io::{self, Write};
use regex::Regex;


// Function that reads file and returns a vector of strings
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// Function that reads a file and returns a vector of vectors of chars
fn charcters_from_file(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

// Function that prints a grid from a vector of vectors of chars
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn find_character_in_grid(grid: &Vec<Vec<char>>, character: char) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == character {
                return (i, j);
            }
        }
    }
    return (0, 0);
}

fn find_dead_end_character(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '.' {
                let mut closed_paths = 0;
                if grid[i-1][j] == '#' {
                    closed_paths += 1;
                }
                if grid[i+1][j] == '#' {
                    closed_paths += 1;
                }
                if grid[i][j-1] == '#' {
                    closed_paths += 1;
                }
                if grid[i][j+1] == '#' {
                    closed_paths += 1;
                }
                if closed_paths == 3 {
                    return (i, j);
                }

            }
        }
    }
    return (0, 0);
}

fn dijkstra_distances(nodes: &Vec<(usize, usize)>, edges: &Vec<((usize, usize), (usize, usize), usize)>, start: (usize, usize), end: (usize, usize)) -> Vec<i32> {
     // So Dijkstra... we need a vector of distances and a vector of visited nodes
     let mut distances: Vec<i32> = vec![i32::MAX; nodes.len()];
     let mut visited: Vec<bool> = vec![false; nodes.len()];
     let mut previous: Vec<usize> = vec![0; nodes.len()];
     let mut current: usize = 0;
     let mut current_distance: i32 = 0;
 
     //println!("Start position: {:?}", start)
     
 
     // find index of node with start position and direction
     for (i, node) in nodes.iter().enumerate() {
         if node.0 == start.0 && node.1 == start.1 {
             current = i;
             distances[i] = 0;
             visited[i] = true;
         }
     }
     //println!("Start node: {:?} : {}", nodes[current], current);
 
     // find all edges that start with current node  
     let mut current_edges: Vec<((usize, usize), (usize, usize), usize)> = Vec::new();
     for edge in edges.iter() {
         if edge.0 == nodes[current] {
             current_edges.push(*edge);
         }
     }
 
     //println!("Current edges: {:?}", current_edges);
     // Update distances
     for edge in current_edges.iter() {
         for (i, node) in nodes.iter().enumerate() {
             if node == &edge.1 {
                 if distances[i] > distances[current] + edge.2 as i32 {
                     distances[i] = distances[current] + edge.2 as i32;
                     previous[i] = current;
                 }
             }
         }
     }
 
     //println!("Distances: {:?}", distances);
 
     // find amount of visited that are false
     let mut unvisited: i32 = 0;
     for v in visited.iter() {
         if !v {
             unvisited += 1;
         }
     }
 
     while unvisited > 0 {
         // find the node with the smallest distance
         let mut smallest_distance: i32 = i32::MAX;
         let mut smallest_index: usize = 0;
         for (i, d) in distances.iter().enumerate() {
             if !visited[i] && *d < smallest_distance {
                 smallest_distance = *d;
                 smallest_index = i;
             }
         }
         current = smallest_index;
         visited[current] = true;
         //println!("Current node: {:?} : {}", nodes[current], current);
 
         // find all edges that start with current node and do not end at previous
         let mut current_edges: Vec<((usize, usize), (usize, usize), usize)> = Vec::new();
         for edge in edges.iter() {
             if edge.0 == nodes[current] && edge.1 != nodes[previous[current]] {
                 current_edges.push(*edge);
             }
         }
         //println!("Current edges: {:?}", current_edges);
 
         // Update distances
         for edge in current_edges.iter() {
             for (i, node) in nodes.iter().enumerate() {
                 if node == &edge.1 {
                     if distances[i] > distances[current] + edge.2 as i32 {
                         distances[i] = distances[current] + edge.2 as i32;
                         previous[i] = current;
                     }
                 }
             }
         }
 
         //println!("Distances: {:?}", distances);
 
         // find amount of visited that are false
         unvisited = 0;
         for v in visited.iter() {
             if !v {
                 unvisited += 1;
             }
         }
         
     }
    return distances;
}

fn get_nodes_and_walls(grid: &Vec<Vec<char>>) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    // loop over all positions in grid and add to nodes if char is "." or "S" or "E"
    let mut nodes: Vec<(usize, usize)> = Vec::new();
    let mut walls: Vec<(usize, usize)> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' {
                walls.push((i, j));
            }
        }
    }

    // Check if position has a "." or "S" or "E", and at least 2 positions next to it as well
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'.' {
                // if position left and above is not a wall
                if i > 0 && j > 0 {
                    if grid[i-1][j] != '#' && grid[i][j-1] != '#' {
                        nodes.push((i, j));
                    }
                }

                // if position right and above is not a wall
                if i > 0 && j < grid[0].len() - 1 {
                    if grid[i-1][j] != '#' && grid[i][j+1] != '#' {
                        nodes.push((i, j));
                    }
                }

                // if position left and below is not a wall
                if i < grid.len() - 1 && j > 0 {
                    if grid[i+1][j] != '#' && grid[i][j-1] != '#' {
                        nodes.push((i, j));
                    }
                }

                // if position right and below is not a wall
                if i < grid.len() - 1 && j < grid[0].len() - 1 {
                    if grid[i+1][j] != '#' && grid[i][j+1] != '#' {
                        nodes.push((i, j));
                    }
                }

            }
            if c == &'S' || c == &'E' {
                nodes.push((i, j));
            }
        }
    }


    return (nodes, walls);
}

fn get_edges(nodes: &Vec<(usize, usize)>, grid: &Vec<Vec<char>>) -> Vec<((usize, usize), (usize, usize), usize)> {
    let mut edges: Vec<((usize, usize), (usize, usize), usize)> = Vec::new();
    // build edges by checking connecting nodes
    for (i, a_node) in nodes.iter().enumerate() {
        for (b, b_node) in nodes.iter().enumerate() {
            if a_node != b_node {
                // check if nodes are connected
                if a_node.0 == b_node.0 {
                    let mut connected: bool = true;
                    if a_node.1 < b_node.1 {
                        for j in a_node.1..b_node.1 {
                            if grid[a_node.0][j] == '#' {
                                connected = false;
                            }
                        }
                    } else {
                        for j in b_node.1..a_node.1 {
                            if grid[a_node.0][j] == '#' {
                                connected = false;
                            }
                        }
                    }
                    if connected {
                        edges.push((*a_node, *b_node, (b_node.1 as i32 - a_node.1 as i32).abs() as usize));
                    }
                }
                if a_node.1 == b_node.1 {
                    let mut connected: bool = true;
                    if a_node.0 < b_node.0 {
                        for j in a_node.0..b_node.0 {
                            if grid[j][a_node.1] == '#' {
                                connected = false;
                            }
                        }
                    } else {
                        for j in b_node.0..a_node.0 {
                            if grid[j][a_node.1] == '#' {
                                connected = false;
                            }
                        }
                    }
                    if connected {
                        edges.push((*a_node, *b_node, (b_node.0 as i32 - a_node.0 as i32).abs() as usize));
                    }
                }
            }
        }
    }
    return edges;
} 

fn main() {
    // start timer
    let start_timer = std::time::Instant::now();

    let mut grid: Vec<Vec<(char)>> = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-20-1/src/input.txt");
    let mut walls: Vec<(usize, usize)> = Vec::new();

 //   print_grid(&grid);
   


    let startpos: (usize, usize) = find_character_in_grid(&grid, 'S');
    let endpos: (usize, usize)   = find_character_in_grid(&grid, 'E');


    let mut nodes: Vec<(usize, usize)> = Vec::new();
    let mut edges: Vec<((usize, usize), (usize, usize), usize)> = Vec::new();

    println!("Start position: {:?}", startpos);
    println!("End position: {:?}", endpos);
    
    (nodes, walls) = get_nodes_and_walls(&grid);

    edges = get_edges(&nodes, &grid);

    println!("Nodes: {}", nodes.len());
    println!("Edges: {}", edges.len());

    let distances = dijkstra_distances(&nodes, &edges, startpos, endpos);

    // find distance to end position
    let mut original_end_distance: i32 = 0;
    for (i, node) in nodes.iter().enumerate() {
        if node == &endpos {
            original_end_distance = distances[i];
        }
    }

    println!("Distance to end: {}", original_end_distance);
    
    let mut amount_ok_paths = 0;
    let MIN_SAVED = 99;

    // loop over all walls and check if wall has 2 ".", "S" or "E" next to it
    for wall in walls.iter() {
        //println!("Wall: {:?}", wall);
        let mut wall_ok = false;
        // wall_ok is true if wall has "." or "S" or "E" both above and below.
        if wall.0 > 0 && wall.0 < grid.len() - 1 {
            if grid[wall.0-1][wall.1] != '#' && grid[wall.0+1][wall.1] != '#' {
                wall_ok = true;
            }
        }
        // wall_ok is true if wall has "." or "S" or "E" both left and right.
        if wall.1 > 0 && wall.1 < grid[0].len() - 1 {
            if grid[wall.0][wall.1-1] != '#' && grid[wall.0][wall.1+1] != '#' {
                wall_ok = true;
            }
        }


        if wall_ok {
            println!("Wall ok: {:?}", wall);
            let mut tmpgrid = grid.clone();
            tmpgrid[wall.0][wall.1] = '.';
            let (nodes, walls) = get_nodes_and_walls(&tmpgrid);
            let edges = get_edges(&nodes, &tmpgrid);
            let distances = dijkstra_distances(&nodes, &edges, startpos, endpos);
            let mut new_end_distance: i32 = 0;
            for (i, node) in nodes.iter().enumerate() {
                if node == &endpos {
                    new_end_distance = distances[i];
                    println!("New end distance: {}", new_end_distance);
                }
            }
            if new_end_distance < original_end_distance - MIN_SAVED {
                amount_ok_paths += 1;
            }
        }
    }

    println!("Amount of ok paths: {}", amount_ok_paths);
            

}