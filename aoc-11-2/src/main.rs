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


fn main() {
    // start timer
    let start = std::time::Instant::now();
    //Allocate what is needed to complete and read the file.

    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-11-1/src/input1.txt");
    // split first line on space into a vector of strings
    let mut split = all_lines[0].split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();


    println!("{:?}", split);

    let mut counters: Vec<(i64, i64)> = Vec::new();

    // loop over split
    for i in 0..split.len() {
        counters.push((split[i].parse::<i64>().unwrap(), 1));
    }

    println!("{:?}", counters);

    // loop 5 times
    for looped in 0..75 {
        
        //loop over counters
        let mut tmpvec: Vec<(i64, i64)> = Vec::new();
        for i in 0..counters.len() {
            
            // if the string is "L" then change it to "#"
            if counters[i].0 == 0 {
                // add 1 rule if 0
                tmpvec.push((1, counters[i].1));
                continue;
            }
            let value:String = counters[i].0.to_string();
            if value.len() % 2 == 0 {
                //split string split[i] into 2 halves
                
                let (left, right) = value.split_at(value.len() / 2);
                //println!("{}: splitting into {} {}", value, left, right);
                
                tmpvec.push((left.to_string().parse::<i64>().unwrap(), counters[i].1));
                tmpvec.push((right.to_string().parse::<i64>().unwrap(), counters[i].1));
                continue
            }

            tmpvec.push((counters[i].0 * 2024, counters[i].1));      
        }
        tmpvec.sort();
        // add duplicates in tmpvec together
        let mut i = 0;
        while i < tmpvec.len() - 1 {
            if tmpvec[i].0 == tmpvec[i + 1].0 {
                tmpvec[i].1 += tmpvec[i + 1].1;
                tmpvec.remove(i + 1);
            } else {
                i += 1;
            }
        }
        println!("{i}: {:?}", tmpvec.len());

        counters = tmpvec;
        
    }

    // loop over counters add all the values together

    let mut sum:i64 = 0;
    println!("total diff values{:?}", counters.len());
    for i in 0..counters.len() {
        sum = sum + counters[i].1;
    }
    println!("{:?}", sum);

    // print time passed    
    println!("Time: {}ms", start.elapsed().as_millis());
}


    

    
