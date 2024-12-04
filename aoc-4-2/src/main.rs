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

// Function that tests for the X-MAS pattern
 fn test_cross(lines: Vec<Vec<char>>, linenumber: i32, charnumber: i32) -> i32 {
    let linepointer = linenumber as usize;
    let charpointer = charnumber as usize;
    let test_vector = lines[linepointer].clone();
    let vector_length = test_vector.len();
 
     let mut amount: i32 = 0;
     if linepointer == 0 || charpointer == 0 || linepointer == vector_length - 1 || charpointer == vector_length - 1 {
         return 0;
     }

     if ((lines[linepointer - 1][charpointer - 1] == 'M' && lines[linepointer + 1][charpointer + 1] == 'S') ||
         (lines[linepointer - 1][charpointer - 1] == 'S' && lines[linepointer + 1][charpointer + 1] == 'M')) &&
        ((lines[linepointer - 1][charpointer + 1] == 'M' && lines[linepointer + 1][charpointer - 1] == 'S') || 
         (lines[linepointer - 1][charpointer + 1] == 'S' && lines[linepointer + 1][charpointer - 1] == 'M')) {
             //println!("X-MAS found at line {} and char {}", linenumber, charnumber);
             amount = amount + 1;
         }

     return amount
 
 }


fn main() {
    let all_lines: Vec<Vec<char>> = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-4-1/src/input1.txt");
    let mut linenumber: i32 = 0;
    let mut total = 0;
    let copyoflines = all_lines.clone();

    for line in all_lines {
        let mut charnumber: i32 = 0;
        
        for c in line {
            if c == 'A' {
                //println!("A found at line {} and char {}", linenumber, charnumber);
                total = total + test_cross(copyoflines.clone(), linenumber, charnumber);


            }
            charnumber = charnumber + 1;
        }
        linenumber = linenumber + 1;
    }

    println!("Total amount of X-MAS found: {}", total);   
}

