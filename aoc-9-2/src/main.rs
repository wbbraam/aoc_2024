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

    let grid: Vec<Vec<char>> = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-9-1/src/input1.txt");
    println!("{:?}", grid);

    let mut disk: Vec<String> = Vec::new();

    // First lets preprocess all the data into an array resembling the disk
    // loop over characters in first line of grid and add to disk

    let mut file_indicator: bool = true;
    let mut file_id: i32 = 0;
    for i in 0..grid[0].len() {
        let value = grid[0][i].to_digit(10).unwrap();

        if file_indicator {
            //loop the amount of value times
            for _ in 0..value {
                disk.push(file_id.to_string());
            }
            file_id += 1;
        } else {
            //loop the amount of value times
            for _ in 0..value {
                disk.push(".".to_string());
            }
        }
        file_indicator = !file_indicator;    
    }

    // We have the disk contents in an array of strings. Now lets start the process of moving the files
    println!("{:?}", disk);

    let mut done: bool = false;

    let mut fixcount = 0;
    let mut startsearchempty: usize = 0;

    // find highest id in disk
    let mut highest_id: i32 = 0;
    for i in 0..disk.len() {
        if disk[i] != ".".to_string() {
            let id = disk[i].parse::<i32>().unwrap();
            if id > highest_id {
                highest_id = id;
            }
        }
    }
    println!("Highest id is {}", highest_id);

    // loop from highest id to 0
    for i in (0..highest_id + 1).rev() {
        // find the first occurence of i
        let mut firstindex: usize = 0;
        for j in startsearchempty..disk.len() {
            if disk[j] == i.to_string() {
                firstindex = j;
                break;
            }
        }
        // find the last occurence of i
        let mut lastindex: usize = 0;
        for j in (0..disk.len()).rev() {
            if disk[j] == i.to_string() {
                lastindex = j;
                break;
            }
        }
        let filelen = lastindex - firstindex + 1;
        println!("ID {} First index is {}, last index is {}, length {}",i,  firstindex, lastindex, filelen);

        // loop over disk
        let mut firstpos: usize = 0;
        let mut lastpos: usize = 0;
        let mut found: bool = false;
        for j in 0..disk.len() {
            if j > firstindex {
                //println!("No empty space found");
                break;
            }
            if disk[j] == ".".to_string() && !found {
                //println!("Found empty space at {}", j);
                found = true;
                firstpos = j;
            }
            if disk[j] != ".".to_string() && found {
                lastpos = j;
                let spacelen = lastpos - firstpos;
                //println!("Space length is {}", spacelen);
                if spacelen >= filelen {
                    //println!("Found space for file, moving");
                    // move file
                    for k in 0..filelen {
                        disk.swap(firstindex + k, firstpos + k);
                    }
                   // println!("Disk after moving{:?}", disk);
                    break;
                } else {
                    //println!("Space too small, continue search");
                    found = false;
                }

            }

        }

    }
    // File move is done, lets check the disk
    println!("Disk defrag done");
    println!("{:?}", disk);

    // Lets start calculating the score
    // Loop over disk on index number   

    let mut total: i64 = 0;
    for i in 0..disk.len() {
        if disk[i] != ".".to_string() {
            total = total + (disk[i].parse::<i64>().unwrap() * i as i64);
        }
    }

    println!("{:?}", total);
    //end timer and print   
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}