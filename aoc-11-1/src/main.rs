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
    //Allocate what is needed to complete and read the file.
    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-11-1/src/input1.txt");
    // split first line on space into a vector of strings
    let mut split = all_lines[0].split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();


    println!("{:?}", split);
    
    // loop 5 times
    for looped in 0..10 {
        // loop through the vector of strings
        let mut tmpvec: Vec<String> = Vec::new();
        for i in 0..split.len() {
            // if the string is "L" then change it to "#"
            if split[i] == "0" {
                // add 1 rule if 0
                tmpvec.push("1".to_string());
                continue;
            }
            if split[i].len() % 2 == 0 {
                //split string split[i] into 2 halves
                let (left, right) = split[i].split_at(split[i].len() / 2);
                tmpvec.push(left.to_string().parse::<i128>().unwrap().to_string());
                tmpvec.push(right.to_string().parse::<i128>().unwrap().to_string());
                continue
            }
            // Convert split[i] to an i128
            let mut num = split[i].parse::<i128>().unwrap();
            // multiply by 2024
            num *= 2024;
            // convert back to a string and push to tmpvec
            tmpvec.push(num.to_string());
        }
        // set split to tmpvec
        split = tmpvec;
        // print i and length of split
        println!("{}: {}", looped, split.len());
        println!("{:?}", split);
    }
    
}