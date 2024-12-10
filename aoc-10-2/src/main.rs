// YEAH I KNOW ITS UGLY AS H***.
// I'm not proud of this code, but it works. 
// I first thought it was needed to try and fit files from the right side to fill the gaps fully... 
// after struggling to understand why it was not the right number and figuring it out why... Duh learn to read !!
// I just whacked an ugly hack together to get it to work. 

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

fn main() {
    // start timer
    let start = std::time::Instant::now();

    let grid: Vec<Vec<char>> = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-10-1/src/input1.txt");
    println!("{:?}", grid);

    let mut total = 0;
    let mut totalpaths = 0;
    let mut processthis: Vec<(usize, usize, usize)> = Vec::new();

    // Loop over all characters with index i and j
    'outer: for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // If the character is a '#' add the index to the toProcess vector
            if grid[y][x] == '0' {
                //println!("Found a 0 at x: {} y: {}", x, y);
                processthis.push((grid[y][x].to_digit(10).unwrap() as usize, x, y));
                
            }
        }
    }

    println!("{:?}", processthis);
    // loop over all elements in toProcess, giving all 3 values of the tuple in a for loop


    for (value, x, y) in processthis {

        // loop over all elements in toProcess, giving all 3 values of the tuple
        let mut pointer:usize = 0;
        let mut positionsfound:Vec<(usize, usize)> = Vec::new();
        
        let mut toProcess:Vec<(usize, usize, usize)> = Vec::new();
        toProcess.push((value, x, y));

        while pointer < toProcess.len(){

            // loop over all elements in toProcess, giving all 3 values of the tuple
            // test if value in the grid above this one is value + 1

            let (value, x, y) = toProcess[pointer];
            //println!("Value: {} x: {} y: {}", value, x, y);
            if value == 9 {
                //println!("Found a 9");
                positionsfound.push((x, y));
                pointer = pointer + 1;
                totalpaths = totalpaths + 1;
                continue;
            }

            if y > 0 {
                if grid[y-1][x] == (value + 1).to_string().chars().next().unwrap() {
                    //println!("Found a value above");
                    toProcess.push((value + 1, x, y-1));
                }
            }
            if y < grid.len() - 1 {
                if grid[y+1][x] == (value + 1).to_string().chars().next().unwrap() {
                    //println!("Found a value below");
                    toProcess.push((value + 1, x, y+1));
                }
            }    
            if x > 0 {
                if grid[y][x-1] == (value + 1).to_string().chars().next().unwrap() {
                    //println!("Found a value left");
                    toProcess.push((value + 1, x-1, y));
                }
            }
            if x < grid[y].len() - 1 {
                if grid[y][x+1] == (value + 1).to_string().chars().next().unwrap() {
                    //println!("Found a value right");
                    toProcess.push((value + 1, x+1, y));
                }
            }

            pointer = pointer + 1;



        }
        positionsfound.sort();
        positionsfound.dedup();
        //println!("Total positions found for this start {:?}", positionsfound.len());
        total = total + positionsfound.len();
    }

    

    println!("Total: {}", total);
    println!("Total paths: {}", totalpaths);
    //end timer and print   
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}