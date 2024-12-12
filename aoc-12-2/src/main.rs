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
fn connected_nodes(grid: Vec<Vec<char>>, i: usize, j: usize) -> (Vec<(usize, usize)>, Vec<(i32, i32)>) {
    let mut connected: Vec<(usize, usize)> = Vec::new();
    let mut notconnected: Vec<(i32, i32)> = Vec::new();
    connected.push((i, j));
    let chartofind = grid[i][j];
    println!("Region: {}", chartofind);
    // Check if i, j is a node
    let mut processed = 0;

    while connected.len() > processed {
        let (i,j) = connected[processed];
        let ii32 = i as i32;
        let ji32 = j as i32;
        //println!("Processing {}, {} to find {}", i, j, chartofind);
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
    //println!("Prerimiter{:?}", notconnected);
    return (connected, notconnected);
}

fn main() {
    // start timer
    let start = std::time::Instant::now();

    let grid: Vec<Vec<char>> = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-12-1/src/input1.txt");
    let mut processed: Vec<(usize, usize)> = Vec::new();
    
    let mut total: i64 = 0;

    println!("{:?}", grid);
    let mut total = 0;
    // Loop over grid
    'outer: for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // Check if i, j already in processed, if so skip
            if processed.contains(&(i, j)) {
                continue;
            }
            // Call function that takes grid, i, j and returns a vector of all connected nodes
            let (connected, mut circumfence) = connected_nodes(grid.clone(), i, j);
            //println!("{:?}", connected);
            //println!("{:?}", perimiter);
            
            // Add all connected nodes to the processed vector
            for (i, j) in &connected {
                processed.push((*i, *j));
            }

           //loop over all processed values
           let mut regioncorners = 0;
           for (i, j) in connected.clone() {
                let mut corners = 0;
                let regionid = grid[i][j];
                let mut above: char = ' ';
                let mut below: char = ' ';
                let mut left: char = ' ';
                let mut right: char = ' ';
                let mut aboveright: char = ' ';
                let mut aboveleft: char = ' ';
                let mut belowright: char = ' ';
                let mut belowleft: char = ' ';


                
                if i == 0 {
                    above = '.';
                    aboveleft = '.';
                    aboveright = '.';
                } 
                if i == grid.len() - 1 {
                    below = '.';
                    belowleft = '.';
                    belowright = '.';
                }
                if j == 0 {
                    left = '.';
                    aboveleft = '.';
                    belowleft = '.';
                }
                if j == grid[i].len() - 1 {
                    right = '.';
                    aboveright = '.';
                    belowright = '.';
                }

                if right != '.' {
                    if grid[i][j+1] == regionid {
                        right = regionid;
                    } else {
                        right = '.';
                    }
                }
                if left != '.' {
                    if grid[i][j-1] == regionid {
                        left = regionid;
                    } else {
                        left = '.';
                    }
                }
                if above != '.' {
                    if grid[i-1][j] == regionid {
                        above = regionid;
                    } else {
                        above = '.';
                    }
                }
                if below != '.' {
                    if grid[i+1][j] == regionid {
                        below = regionid;
                    } else {
                        below = '.';
                    }
                }
                if aboveright != '.' {
                    if grid[i-1][j+1] == regionid {
                        aboveright = regionid;
                    } else {
                        aboveright = '.';
                    }
                }
                if aboveleft != '.' {
                    if grid[i-1][j-1] == regionid {
                        aboveleft = regionid;
                    } else {
                        aboveleft = '.';
                    }
                }
                if belowright != '.' {
                    if grid[i+1][j+1] == regionid {
                        belowright = regionid;
                    } else {
                        belowright = '.';
                    }
                }
                if belowleft != '.' {
                    if grid[i+1][j-1] == regionid {
                        belowleft = regionid;
                    } else {
                        belowleft = '.';
                    }
                }


                // Outer corners
                if above == '.' && left == '.' {
                    corners += 1;
                }
                if above == '.' && right == '.' {
                    corners += 1;
                }
                if below == '.' && left == '.' {
                    corners += 1;
                }
                if below == '.' && right == '.' {
                    corners += 1;
                }
                // Inner corners
                if above == regionid && left == regionid && aboveleft != regionid {
                    corners += 1;
                }
                if above == regionid && right == regionid && aboveright != regionid {
                    corners += 1;
                }
                if below == regionid && left == regionid && belowleft != regionid {
                    corners += 1;
                }
                if below == regionid && right == regionid && belowright != regionid {
                    corners += 1;
                }





                //println!("Corners: {} at {}, {}", corners, i, j);
                regioncorners = regioncorners + corners;
           }
            
            println!("Region corners: {}", regioncorners);
            let area = connected.len() as i64;
            println!("Area: {}", area);
            let cost = area * regioncorners as i64;
            println!("Cost: {}", cost);
           total = total + cost;
        }
    }
    println!("Total cost: {}", total);
}