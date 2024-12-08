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

fn calculate_antinode(antenna: Vec<(i32, i32)>, maxcol: usize, maxrow: usize) -> Vec<(i32, i32)> {

    println!("Calculating antinode for antenna: {:?}", antenna);
    let mut t = 0;
    let mut antinode: Vec<(i32, i32)> = Vec::new();

    // loop over index of antenna as i
    for i in 0..antenna.len() {
        // print i
        println!("i: {}", i);
        let a1x = antenna[i].0;
        let a1y = antenna[i].1;
        // loop over the remainder of antenna as j
        for j in i+1..antenna.len() {
            let a2x = antenna[j].0;
            let a2y = antenna[j].1;
            let dx = a2x - a1x;
            let dy = a2y - a1y;

            let mut anti1x: i32 = 0;
            let mut anti2x: i32 = 0;
            let mut anti1y: i32 = 0;
            let mut anti2y: i32 = 0;

            // calculate antinode of a1
            if a1x < a2x {
                anti1x = a1x - dx.abs();
                anti2x = a2x + dx.abs();
            } else {
                anti1x = a1x + dx.abs();
                anti2x = a2x - dx.abs();
            }

            if a1y < a2y {
                anti1y = a1y - dy.abs();
                anti2y = a2y + dy.abs();
            } else {
                anti1y = a1y + dy.abs();
                anti2y = a2y - dy.abs();
            } 

            // check if antinode is within the grid
            if anti1x >= 0 && anti1x < maxcol as i32 && anti1y >= 0 && anti1y < maxrow as i32 {
                println!("Antinode 1: {} {} ongrid", anti1x, anti1y);
                antinode.push((anti1x, anti1y));
                t = t + 1;
            } else {
                println!("Antinode 1: {} {} offgrid", anti1x, anti1y);
            }

            if anti2x >= 0 && anti2x < maxcol as i32 && anti2y >= 0 && anti2y < maxrow as i32 {
                println!("Antinode 2: {} {} ongrid", anti2x, anti2y);
                antinode.push((anti2x, anti2y));
                t = t + 1;
            } else {
                println!("Antinode 2: {} {} offgrid", anti2x, anti2y);
            }



        }

    }
    println!("Total antinodes: {}", t);
    return antinode;
}

fn main() {
    let grid: Vec<Vec<char>> = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-8-1/src/input1.txt");
    let maxcol = grid[0].len();
    let maxrow = grid.len();
    let mut total: i32 = 0;

    let mut totalantinode: Vec<(i32, i32)> = Vec::new();

    // vector of contents of each cell and the coordinates
    let mut cells: Vec<(char, i32, i32)> = Vec::new();

    //loop over all the cells and store the contents and coordinates
    for i in 0..maxrow {
        for j in 0..maxcol {
            if grid[i][j] == '.' {
                continue;
            }
            cells.push((grid[i][j], i as i32, j as i32));
        }
    }

    // sort cells
    cells.sort();

    // print the contents and coordinates
    //for (c, i, j) in cells.clone() {
    //    println!("{} {} {}", c, i, j);
    //}

    // Vector with all unique different characters in cells
    let mut unique: Vec<char> = Vec::new();
    for (c, _, _) in cells.clone() {
        if !unique.contains(&c) {
            unique.push(c);
        }
    }

    //println!("Unique characters: {:?}", unique);

    // All prepping done. Le'ts loop over the signals

    for signal in unique {
        println!("Signal: {}", signal);
        let mut antenna: Vec<(i32, i32)> = Vec::new();
        // loop over all cells where first value is signal, to find all antennas
        for (c, i, j) in cells.clone() {
            if c == signal {
                antenna.push((i, j));
                println!("coords found {} {}", i, j);
            }
        }

        let returnednodes = calculate_antinode(antenna, maxcol, maxrow);
        for node in returnednodes {
            if !totalantinode.contains(&node) {
                totalantinode.push(node);
            }
        }
    }
    println!("Total: {}", total);
    totalantinode.sort();
    totalantinode.dedup();
    println!("Total unqiue antinodes: {}", totalantinode.len());
}
