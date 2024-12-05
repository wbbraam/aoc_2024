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

fn split_piped_line_to_i32(line: &str) -> Vec<i32> {
    line.split('|')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn split_comma_line_to_i32(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


fn main() {
    //Allocate what is needed to complete and read the file.
    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-5-1/src/input1.txt");
    let mut total: i32 = 0;

    let mut sorting:   Vec<Vec<i32>> = Vec::new();
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    
    // take all lines from all_lines prior to an empty line into sorting   
    let mut part_flag = true;    
    for line in all_lines {
        if line == "" {
            part_flag = false;
            continue
        }
        if part_flag {
            println!("Sorting: {}", line);
            sorting.push(split_piped_line_to_i32(&line));
        } else {
            println!("Sequences: {}", line);
            sequences.push(split_comma_line_to_i32(&line));
        }
    }

    for line in sequences {
        println!("{:?}", line);
        let mut line_ok = true;
        // Loop through sorting rules   
        for rule in &sorting {
            //println!("{:?}", rule);
            //check if first value in rule occurs before the second value in vector line
            let pos1 = line.iter().position(|&x| x == rule[0]);
            let pos2 = line.iter().position(|&x| x == rule[1]);
            println!("pos1: {:?}, pos2: {:?}", pos1, pos2);
            if pos1 != None && pos2 != None {
                if pos1 > pos2 {
                    line_ok = false;
                }
            }
        }
        if line_ok {
            println!("Line is ok: {:?}", line);
            let len = line.len();
            let middle = line.get(len/2).unwrap();
            print !("Middle: {:?}", middle);
            total = total + middle;

        } else {
            //println!("Line is not ok: {:?}", line);
        }
    }




    println!("Total: {}", total);

}
