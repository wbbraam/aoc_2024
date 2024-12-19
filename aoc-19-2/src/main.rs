use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use std::io::{self, Write};
use cached::proc_macro::cached;
use cached::UnboundCache;

// Function that reads file and returns a vector of strings
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[cached(
    type = "UnboundCache<(String, Vec<String>), i64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (d.to_string(), px.to_vec()) }"#
)]
fn find_options(d: &String, px: &Vec<String>) -> i64 {


    if d.len() == 0 {
        return 1;
    }

    let d_new = "".to_string();
    let mut count: i64 = 0;

    for p in px {
        //println!("{}", p);
        if d.starts_with(p) {

            //println!("Startswith: {}", p);
            let d_new = d[p.len()..].to_string();
            //println!("d_new: {}", d_new);
            // No need to add when we already have 1
            count = count + find_options(&d_new, &px);
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
    let patterns: Vec<String> = all_lines[0]
    .split(", ")
    .map(|s| s.to_string())
    .collect();

    // store third line to end of all_lines into a vector of strings
    let designs: Vec<String> = all_lines[2..]
    .iter()
    .map(|s| s.to_string())
    .collect();

    println!("{:?}", patterns);
    println!("{:?}", designs);

    let mut possible_count = 0;
    let mut total_possible: i64 = 0;
    for design in designs {
        let amount_of_options = find_options(&design, &patterns);
        total_possible += amount_of_options;
        if amount_of_options > 0 {
            possible_count += 1;
        }
        println!("{} -> {:?}", design, amount_of_options);
    }

    println!("Possible count: {}", possible_count);
    println!("Total possible: {}", total_possible);
    // print time passed
    println!("Time: {}ms", start_timer.elapsed().as_millis());
}


