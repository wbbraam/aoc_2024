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

fn test_xmas(lines: Vec<Vec<char>>, linenumber: i32, charnumber: i32) -> i32 {
    // print linenumber and charnumber
    println!("Linenumber: {} {}", linenumber, charnumber);
    return 0
}

fn test_horizontal(lines: Vec<Vec<char>>, linenumber: i32, charnumber: i32) -> i32 {
   let linepointer = linenumber as usize;
   let charpointer = charnumber as usize;
   let test_vector = lines[linepointer].clone();
   let vector_length = test_vector.len();

    let mut amount: i32 = 0;
    // Test if characters right of charnumber are "M", "A", "S" 
    if charnumber + 2 < vector_length as i32 {
        if test_vector[charpointer + 1] == 'M' && test_vector[charpointer + 2] == 'A' && test_vector[charpointer + 3] == 'S' {
            println!("MAS found at line {} and char {}", linenumber, charnumber);
            amount = amount + 1;
        }
    }

    // Test if characters left of charnumber are "M", "A", "S"
    if charnumber - 3 >= 0 {
        if test_vector[charpointer - 1] == 'M' && test_vector[charpointer - 2] == 'A' && test_vector[charpointer - 3] == 'S' {
            println!("MAS found at line {} and char {}", linenumber, charnumber);
            amount = amount + 1;
        }
    }

    return amount

}

fn test_vertical(lines: Vec<Vec<char>>, linenumber: i32, charnumber: i32) -> i32 {
    let linepointer = linenumber as usize;
    let charpointer = charnumber as usize;
    let test_vector = lines[linepointer].clone();
    let vector_length = test_vector.len();
 
    let mut amount: i32 = 0;
 
        // Test if characters below charnumber are "M", "A", "S"
        if linenumber + 3 < vector_length as i32 {
            if lines[linepointer + 1][charpointer] == 'M' && lines[linepointer + 2][charpointer] == 'A' && lines[linepointer + 3][charpointer] == 'S' {
                println!("MAS found at line {} and char {}", linenumber, charnumber);
                amount = amount + 1;
            }
        }

        // Test if characters above charnumber are "M", "A", "S"   
        if linenumber - 3 >= 0 {
            if lines[linepointer - 1][charpointer] == 'M' && lines[linepointer - 2][charpointer] == 'A' && lines[linepointer - 3][charpointer] == 'S' {
                println!("MAS found at line {} and char {}", linenumber, charnumber);
                amount = amount + 1;
            }
        }

     return amount
 
 }

 fn test_diagonal(lines: Vec<Vec<char>>, linenumber: i32, charnumber: i32) -> i32 {
    let linepointer = linenumber as usize;
    let charpointer = charnumber as usize;
    let test_vector = lines[linepointer].clone();
    let vector_length = test_vector.len();
 
    let mut amount: i32 = 0;
 
    // Test if characters below and right of charnumber are "M", "A", "S"
    if linenumber + 3 < vector_length as i32 && charnumber + 3 < vector_length as i32 {
        if lines[linepointer + 1][charpointer + 1] == 'M' && lines[linepointer + 2][charpointer + 2] == 'A' && lines[linepointer + 3][charpointer + 3] == 'S' {
            println!("MAS found at line {} and char {}", linenumber, charnumber);
            amount = amount + 1;
        }
    }
    // Test if characters above and left of charnumber are "M", "A", "S"
    if linenumber - 3 >= 0 && charnumber - 3 >= 0 {
        if lines[linepointer - 1][charpointer - 1] == 'M' && lines[linepointer - 2][charpointer - 2] == 'A' && lines[linepointer - 3][charpointer - 3] == 'S' {
            println!("MAS found at line {} and char {}", linenumber, charnumber);
            amount = amount + 1;
        }
    }
    // Test if characters below and left of charnumber are "M", "A", "S"
    if linenumber + 3 < vector_length as i32 && charnumber - 3 >= 0 {
        if lines[linepointer + 1][charpointer - 1] == 'M' && lines[linepointer + 2][charpointer - 2] == 'A' && lines[linepointer + 3][charpointer - 3] == 'S' {
            println!("MAS found at line {} and char {}", linenumber, charnumber);
            amount = amount + 1;
        }
    }
    // Test if characters above and right of charnumber are "M", "A", "S"
    if linenumber - 3 >= 0 && charnumber + 3 < vector_length as i32 {
        if lines[linepointer - 1][charpointer + 1] == 'M' && lines[linepointer - 2][charpointer + 2] == 'A' && lines[linepointer - 3][charpointer + 3] == 'S' {
            println!("MAS found at line {} and char {}", linenumber, charnumber);
            amount = amount + 1;
        }
    }    


     return amount
 
 }


fn main() {
    let all_lines: Vec<Vec<char>> = charcters_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-4-1/src/input1.txt");
    let mut linenumber: i32 = 0;
    let mut total = 0;
    let amountoflines = all_lines.len();
    let copyoflines = all_lines.clone();
    let linelength = all_lines[0].len();

    for line in all_lines {
        let mut charnumber: i32 = 0;
        
        for c in line {
            if c == 'X' {
                println!("X found at line {} and char {}", linenumber, charnumber);
                total = total + test_horizontal(copyoflines.clone(), linenumber, charnumber);
                total = total + test_vertical(copyoflines.clone(), linenumber, charnumber);
                total = total + test_diagonal(copyoflines.clone(), linenumber, charnumber);

            }
            charnumber = charnumber + 1;
        }
        linenumber = linenumber + 1;
    }

    println!("Total amount of MAS found: {}", total);   
}

