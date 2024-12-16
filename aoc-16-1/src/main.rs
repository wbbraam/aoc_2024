use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use std::io::{self, Write};


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
fn print_grid(grid: Vec<Vec<char>>) {
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

fn main() {
    // start timer
    let start_timer = std::time::Instant::now();

    let mut grid = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-16-1/src/input1.txt");
    print_grid(grid.clone());

    let startpos: (usize, usize) = find_character_in_grid(&grid, 'S');
    let endpos: (usize, usize) = find_character_in_grid(&grid, 'E');
    let direction: char = 'E';

    let mut nodes: Vec<(usize, usize, char)> = Vec::new();
    let mut edges: Vec<((usize, usize, char), (usize, usize, char), usize)> = Vec::new();

    println!("Start position: {:?}", startpos);
    println!("End position: {:?}", endpos);
    
    // Let's start with closing off paths that lead nowhere

    let mut pos: (usize, usize) = find_dead_end_character(&grid);
    while pos != (0, 0) {
        grid[pos.0][pos.1] = '#';
        pos = find_dead_end_character(&grid);
    }
    print_grid(grid.clone());

    // build nodes... we assume every '.' combined with its direction is a node

    // loop over all characters in the grid
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if grid[i][j] != '#' {
                // check if the node already exists
                nodes.push((i, j, 'N'));
                nodes.push((i, j, 'E'));
                nodes.push((i, j, 'S'));
                nodes.push((i, j, 'W'));
            }
        }
    }
    for node in nodes.iter() {
        println!("{:?}", node);
    }

    // Build edges in 2 steps. First rotational edges, then straight edges if exist
    // loop over all nodes
    for node in nodes.iter() {
        if node.2 == 'N' {
            edges.push((*node, (node.0, node.1, 'E'), 1000));
            edges.push((*node, (node.0, node.1, 'W'), 1000));
        }
        if node.2 == 'E' {
            edges.push((*node, (node.0, node.1, 'S'), 1000));
            edges.push((*node, (node.0, node.1, 'N'), 1000));
        }
        if node.2 == 'S' {
            edges.push((*node, (node.0, node.1, 'W'), 1000));
            edges.push((*node, (node.0, node.1, 'E'), 1000));
        }
        if node.2 == 'W' {
            edges.push((*node, (node.0, node.1, 'N'), 1000));
            edges.push((*node, (node.0, node.1, 'S'), 1000));
        }
    }

    // Now the nodes that move a step
    for node in nodes.iter() {
        if node.2 == 'N' {
            if grid[node.0-1][node.1] != '#' {
                edges.push((*node, (node.0-1, node.1, 'N'), 1));
            }
        }
        if node.2 == 'E' {
            if grid[node.0][node.1+1] != '#' {
                edges.push((*node, (node.0, node.1+1, 'E'), 1));
            }
        }
        if node.2 == 'S' {
            if grid[node.0+1][node.1] != '#' {
                edges.push((*node, (node.0+1, node.1, 'S'), 1));
            }
        }
        if node.2 == 'W' {
            if grid[node.0][node.1-1] != '#' {
                edges.push((*node, (node.0, node.1-1, 'W'), 1));
            }
        }
    }


    for edge in edges.iter() {
        println!("{:?} -> {:?} = {}", edge.0, edge.1, edge.2);
    }
    println!("Nodes: {}", nodes.len());
    println!("Edges: {}", edges.len());
    return;

    // So Dijkstra... we need a vector of distances and a vector of visited nodes
    let mut distances: Vec<i32> = vec![i32::MAX; nodes.len()];
    let mut visited: Vec<bool> = vec![false; nodes.len()];
    let mut previous: Vec<usize> = vec![0; nodes.len()];
    let mut current: usize = 0;
    let mut current_distance: i32 = 0;

    // find index of node with start position and direction
    for (i, node) in nodes.iter().enumerate() {
        if node.0 == startpos.0 && node.1 == startpos.1 && node.2 == direction {
            current = i;
            distances[i] = 0;
            visited[i] = true;
        }
    }
    println!("Start node: {:?} : {}", nodes[current], current);
    // find all edges that start with current node  
    let mut current_edges: Vec<((usize, usize, char), (usize, usize, char), usize)> = Vec::new();
    for edge in edges.iter() {
        if edge.0 == nodes[current] {
            current_edges.push(*edge);
        }
    }
    println!("Current edges: {:?}", current_edges);
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
        let mut current_edges: Vec<((usize, usize, char), (usize, usize, char), usize)> = Vec::new();
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
       println!("Unvisited: {}", unvisited);
        
    }

    // find nodes that start with end_position
    let mut end_nodes: Vec<usize> = Vec::new();
    for (i, node) in nodes.iter().enumerate() {
        if node.0 == endpos.0 && node.1 == endpos.1 {
            println!("End node: {:?} : {}", node, distances[i]);
        }
    }

    // print all end_nodes from nodes with distances

    

}