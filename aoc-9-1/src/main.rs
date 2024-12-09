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

    let grid: Vec<Vec<char>> = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-9-1/src/test.txt");
    println!("{:?}", grid);

    let mut disk: Vec<String> = Vec::new();

    // First lets preprocess all the data into an array resembling the disk
    // loop over characters in first line of grid and add to disk

    let mut file_indicator: bool = true;
    let mut file_id: i32 = 0;
    for i in 0..grid[0].len() {
        let value = grid[0][i].to_digit(10).unwrap();
        println!("{:?}", value);

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

    while !done {
        // find first position in disk with a "."
        let mut index: i32 = 0;
        for i in 0..disk.len() {
            if disk[i] == ".".to_string() {
                index = i as i32;
                break;
            }
        }

        // find the last positioon in disk that is not a "."
        let mut last_index: i32 = 0;
        for i in (0..disk.len()).rev() {
            if disk[i] != ".".to_string() {
                last_index = i as i32;
                break;
            }
        }
     
        //if index % 100 == 0 {
        //    println!("{:?} out of {}", index, disk.len());
        //}
        

        if index < last_index {
            // swap the two positions
            disk.swap(index as usize, last_index as usize);

        } else {
            done = true;
        }

    }
    // File move is done, lets check the disk
    //println!("{:?}", disk);

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