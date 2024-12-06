use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};

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

fn main() {

    let startime = std::time::Instant::now();

    let grid: Vec<Vec<char>> = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-6-1/src/input1.txt");
    let mut startposx: i32 = 0;
    let mut startposy: i32 = 0;
    let startdir  = "N";

    let xsize: i32 = grid[0].len() as i32;
    let ysize: i32 = grid.len() as i32;
    // find both indexes in grid where a character us "^"

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '^' {
                startposx = (j as i32);
                startposy = (i as i32);
            }
        }
    }

    //println!("Start position is x: {} and y: {}", startposx, startposy);

    let mut done: bool = false;

    let mut posx: i32 = startposx;
    let mut posy: i32 = startposy;
    let mut dir  = startdir;

    let mut total: i32 = 1;

    let mut storedsteps: Vec<(i32, i32)> = Vec::new();

    while !done {

        // store pox and posy in storedsteps as tuple  
        storedsteps.push((posx, posy));

        let mut newposx: i32 = 0;
        let mut newposy: i32 = 0;

        // calculate new position to check
        if dir == "N" {
            newposy = posy - 1;
            newposx = posx;
        } else if dir == "E" {
            newposx = posx + 1;
            newposy = posy;
        } else if dir == "S" {
            newposy = posy + 1;
            newposx = posx;
        } else if dir == "W" {
            newposx = posx - 1;
            newposy = posy
        }

        // check if new position is out of bounds
        if newposx < 0 || newposx >= xsize || newposy < 0 || newposy >= ysize {
            //println!("Out of bounds at x: {} and y: {}", newposx, newposy);
            // Perfect he left the room. So we are done.
            total = total + 1;
            done = true;
            continue;
        }

        // check if the new position is an # in the grid    
        if grid[newposy as usize][newposx as usize] == '#' {
            //println!("Found a wall at x: {} and y: {}", newposx, newposy);
            //Change direction.   
            if dir == "N" {
                dir = "E";
            } else if dir == "E" {
                dir = "S";
            } else if dir == "S" {
                dir = "W";
            } else if dir == "W" {
                dir = "N";
            }
            continue;
        }

        // So it is not a wall, and we are not changing direction. So we can move.
        posx = newposx;
        posy = newposy;

        // Step taken so add to total
        total = total + 1;

        // print step number, direction and position    
        //println!("Step: {}, Direction: {}, Position: x: {}, y: {}", total, dir, posx, posy);

    }
    storedsteps.sort();
    storedsteps.dedup();
    //println!("Total unique steps: {}", storedsteps.len());
    //println!("Total steps: {}", total);

    // print time running until now     
    println!("Time running until now: {:?}", startime.elapsed());

    //remove startpos from storedsteps this one needs to be excluded anyway
    storedsteps.retain(|&x| x != (startposx, startposy));

    //iterate overstoredsteps   
    let mut steps = 0;
    let mut loops = 0;
    for (i, &step) in storedsteps.iter().enumerate() {
        //println!("Step: {}, Position: x: {}, y: {}", i, step.0, step.1);
        //pull a copy from grid in a new variable   
        let mut newgrid = grid.clone();
        // replace the step in grid with a "#"  
        newgrid[step.1 as usize][step.0 as usize] = '#';

        let mut done: bool = false;

        let mut posx: i32 = startposx;
        let mut posy: i32 = startposy;
        let mut dir  = startdir;
    
        let mut total: i32 = 1;

        steps = 0;
    
    
        while !done {
    
    
            let mut newposx: i32 = 0;
            let mut newposy: i32 = 0;
    
            // calculate new position to check
            if dir == "N" {
                newposy = posy - 1;
                newposx = posx;
            } else if dir == "E" {
                newposx = posx + 1;
                newposy = posy;
            } else if dir == "S" {
                newposy = posy + 1;
                newposx = posx;
            } else if dir == "W" {
                newposx = posx - 1;
                newposy = posy
            }
    
            // check if new position is out of bounds
            if newposx < 0 || newposx >= xsize || newposy < 0 || newposy >= ysize {
                // Perfect he left the room. So we are done.
                total = total + 1;
                done = true;
                //println!("No loop found, solved in {} steps", total);
                continue;
            }
    
            // check if the new position is an # in the grid    
            if newgrid[newposy as usize][newposx as usize] == '#' {
                //Change direction.   
                if dir == "N" {
                    dir = "E";
                } else if dir == "E" {
                    dir = "S";
                } else if dir == "S" {
                    dir = "W";
                } else if dir == "W" {
                    dir = "N";
                }
                continue;
            }
    
            // So it is not a wall, and we are not changing direction. So we can move.
            posx = newposx;
            posy = newposy;
    
            // Step taken so add to total
            total = total + 1;
    
            if total > 10000 {
                //println!("Assuming loop");
                loops = loops + 1;
                done = true;
            }
            // print step number, direction and position    
            //println!("Step: {}, Direction: {}, Position: x: {}, y: {}", total, dir, posx, posy);
    
        }
    }
    println!("Total loops found: {}", loops);
    
    let endtime = startime.elapsed();
    // print total time taken   
    println!("Total time taken: {:?}", endtime);
}



