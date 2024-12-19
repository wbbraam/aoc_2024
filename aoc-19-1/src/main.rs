use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use std::io::{self, Write};

// Function that reads file and returns a vector of strings
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn find_options(d: &String, px: &Vec<&str>) -> i32 {
    //println!("{} -> {:?}", d, px);

    if d.len() == 0 {
        return 1;
    }

    let d_new = "".to_string();
    let mut count = 0;

    for p in px {
        //println!("{}", p);
        if d.starts_with(p) {

            //println!("Startswith: {}", p);
            let d_new = d[p.len()..].to_string();
            //println!("d_new: {}", d_new);
            // No need to add when we already have 1
            count = count + find_options(&d_new, px);
            println!("Count: {}", count);
            
            
        }
    }

    return count;
}

fn main() {
    // start timer
    let start_timer = std::time::Instant::now();

    let mut grid: Vec<Vec<(char)>> = Vec::new();
    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-19-1/src/input.txt");

    // split first line on ", " and parse to string
    let mut patterns = all_lines[0].split(", ").collect::<Vec<&str>>();

    // store third line to end of all_lines into a vector of strings
    let mut designs = all_lines[2..].to_vec();

    println!("{:?}", patterns);
    println!("{:?}", designs);

    let mut possible_count = 0;
    for design in designs {
        let amount_of_options = find_options(&design, &patterns);
        if amount_of_options > 0 {
            possible_count += 1;
        }
        println!("{} -> {:?}", design, amount_of_options);
    }

    println!("Possible count: {}", possible_count);
    // print time passed
    println!("Time: {}ms", start_timer.elapsed().as_millis());
}


