use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};


// Function that reads a file and returns a vector of vectors of chars
fn charcters_from_file(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

// A function that takes grid and 2 indexes and returns a vector of all connected nodes
fn connected_nodes(grid: Vec<Vec<char>>, i: usize, j: usize) -> (Vec<(usize, usize)>, usize) {
    let mut connected: Vec<(usize, usize)> = Vec::new();
    let mut notconnected: Vec<(i32, i32)> = Vec::new();
    connected.push((i, j));
    let chartofind = grid[i][j];
    // Check if i, j is a node
    let mut processed = 0;

    while connected.len() > processed {
        let (i,j) = connected[processed];
        let ii32 = i as i32;
        let ji32 = j as i32;
        println!("Processing {}, {} to find {}", i, j, chartofind);
        // Check if cells around i, j are connected
        if i > 0 && grid[i-1][j] == chartofind {
            // if i,j not yet in connected
            if !connected.contains(&(i-1, j)) {
                connected.push((i-1, j));
            }    
        } else {
            
            notconnected.push((ii32-1, ji32));
            
        }
        if i < grid.len() - 1 && grid[i+1][j] == chartofind {
            if !connected.contains(&(i+1, j)) {
                connected.push((i+1, j));
            }
           
        } else {
            
            notconnected.push((ii32+1, ji32));
            
        }
        if j > 0 && grid[i][j-1] == chartofind {
            if !connected.contains(&(i, j-1)) {
                connected.push((i, j-1));
            }
            
        } else {
            
            notconnected.push((ii32, ji32-1));
            
        }
        if j < grid[i].len() - 1 && grid[i][j+1] == chartofind {
            if !connected.contains(&(i, j+1)) {
                connected.push((i, j+1));
            }
        } else {
            
                notconnected.push((ii32, ji32+1));
            
        }
        
        processed += 1;
    }
    println!("{:?}", notconnected);
    return (connected, notconnected.len());
}

fn main() {
    // start timer
    let start = std::time::Instant::now();

    let grid: Vec<Vec<char>> = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-12-1/src/input1.txt");
    let mut processed: Vec<(usize, usize)> = Vec::new();
    
    let mut total: i64 = 0;

    println!("{:?}", grid);

    // Loop over grid
    'outer: for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // Check if i, j already in processed, if so skip
            if processed.contains(&(i, j)) {
                continue;
            }
            // Call function that takes grid, i, j and returns a vector of all connected nodes
            let (connected, circumfence) = connected_nodes(grid.clone(), i, j);
            println!("{:?}", connected);
            println!("{:?}", circumfence);
            
            // Add all connected nodes to the processed vector
            for (i, j) in &connected {
                processed.push((*i, *j));
            }

            // calculate area of connected nodes
            let area = connected.len();;
            // calculate cost of area
            let cost = area * circumfence;

            // add costs to total cost
            total = total + cost as i64;
        }
    }
    println!("Total cost: {}", total);
}