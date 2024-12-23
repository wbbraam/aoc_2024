use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use std::io::{self, Write};
use regex::Regex;
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

fn arrow_options(from: char, to: char) -> Vec<String> {

    let mut options: Vec<String> = Vec::new();

    if from == 'A' {
        if to == 'A'{ return vec!["A".to_string()];}
        if to == '^'{ return vec!["<A".to_string()];}
        if to == 'v'{ return vec!["<vA".to_string(), "v<A".to_string()];}
        if to == '<'{ return vec!["v<<A".to_string(), "<v<A".to_string()];}
        if to == '>'{ return vec!["vA".to_string()];}
    }
    if from == '^' {
        if to == 'A'{ return vec![">A".to_string()];}
        if to == '^'{ return vec!["A".to_string()];}
        if to == 'v'{ return vec!["vA".to_string()];}
        if to == '<'{ return vec!["v<A".to_string()];}
        if to == '>'{ return vec!["v>A".to_string(), ">vA".to_string()];}
    }
    if from == 'v' { 
        if to == 'A'{ return vec!["^>A".to_string(), ">^A".to_string()];}
        if to == '^'{ return vec!["^A".to_string()];}
        if to == 'v'{ return vec!["A".to_string()];}
        if to == '<'{ return vec!["<A".to_string()];}
        if to == '>'{ return vec![">A".to_string()];}
    }
    if from == '<' {
        if to == 'A'{ return vec![">>^A".to_string(), ">^>A".to_string()];}
        if to == '^'{ return vec![">^A".to_string()];}
        if to == 'v'{ return vec![">A".to_string()];}
        if to == '<'{ return vec!["A".to_string()];}
        if to == '>'{ return vec![">>A".to_string()];}
    }
    if from == '>' {
        if to == 'A'{ return vec!["^A".to_string()];}
        if to == '^'{ return vec!["<^A".to_string(), "^<A".to_string()];}
        if to == 'v'{ return vec!["<A".to_string()];}
        if to == '<'{ return vec!["<<A".to_string()];}
        if to == '>'{ return vec!["A".to_string()];}
    }

    return options;
}

fn keypad_options(from:char, to:char) -> Vec<String> {

    let mut options: Vec<String> = Vec::new();

    if from == 'A'{
        if to == 'A'{ return vec!["A".to_string()]; }
        if to == '0'{ return vec!["<A".to_string()]; }
        if to == '1'{ return vec!["^<<A".to_string()]; }
        if to == '2'{ return vec!["<^A".to_string(), "^<A".to_string()]; }
        if to == '3'{ return vec!["^A".to_string()]; }
        if to == '4'{ return vec!["^^<<A".to_string()]; }
        if to == '5'{ return vec!["<^^A".to_string(), "^^<A".to_string()]; }
        if to == '6'{ return vec!["^^A".to_string()]; }
        if to == '7'{ return vec!["^^^<<A".to_string()]; }
        if to == '8'{ return vec!["<^^^A".to_string(), "^^^<A".to_string()]; }
        if to == '9'{ return vec!["^^^A".to_string()]; }
    }
    
    if from == '0' {
        if to == 'A'{ return vec![">A".to_string()]; }
        if to == '0'{ return vec!["A".to_string()]; }
        if to == '1'{ return vec!["^<A".to_string()]; }
        if to == '2'{ return vec!["^A".to_string()]; }
        if to == '3'{ return vec!["^>A".to_string(), ">^A".to_string()]; }
        if to == '4'{ return vec!["^^<A".to_string(), "^<^A".to_string()]; }
        if to == '5'{ return vec!["^^A".to_string()]; }
        if to == '6'{ return vec!["^^>A".to_string(), ">^^A".to_string()]; }
        if to == '7'{ return vec!["^^^<A".to_string()]; }
        if to == '8'{ return vec!["^^^A".to_string()]; }
        if to == '9'{ return vec!["^^^>A".to_string(), ">^^^A".to_string()]; }
    }

    if from == '1' {
        if to == 'A'{ return vec![">>vA".to_string()]; }
        if to == '0'{ return vec![">vA".to_string()]; }
        if to == '1'{ return vec!["A".to_string()]; }
        if to == '2'{ return vec![">A".to_string()]; }
        if to == '3'{ return vec![">>A".to_string()]; }
        if to == '4'{ return vec!["^A".to_string()]; }
        if to == '5'{ return vec!["^>A".to_string(), ">^A".to_string()]; }
        if to == '6'{ return vec!["^>>A".to_string(), ">>^A".to_string()]; }
        if to == '7'{ return vec!["^^A".to_string()]; }
        if to == '8'{ return vec!["^^>A".to_string(), ">^^A".to_string()]; }
        if to == '9'{ return vec!["^^>>A".to_string(), ">>^^A".to_string()]; }
    }
    if from == '2' {
        if to == 'A'{ return vec![">vA".to_string(), "v>A".to_string()]; }
        if to == '0'{ return vec!["vA".to_string()]; }
        if to == '1'{ return vec!["<A".to_string()]; }
        if to == '2'{ return vec!["A".to_string()]; }
        if to == '3'{ return vec![">A".to_string()]; }
        if to == '4'{ return vec!["^<A".to_string(), "<^A".to_string()]; }
        if to == '5'{ return vec!["^A".to_string()]; }
        if to == '6'{ return vec!["^>A".to_string(), ">^A".to_string()]; }
        if to == '7'{ return vec!["^^<A".to_string(), "<^^A".to_string()]; }
        if to == '8'{ return vec!["^^A".to_string()]; }
        if to == '9'{ return vec!["^^>A".to_string(), ">^^A".to_string(), "^>^A".to_string()]; }
    }
    if from == '3' {
        if to == 'A'{ return vec!["vA".to_string()]; }
        if to == '0'{ return vec!["v<A".to_string(), "<vA".to_string()]; }
        if to == '1'{ return vec!["<<A".to_string()]; }
        if to == '2'{ return vec!["<A".to_string()]; }
        if to == '3'{ return vec!["A".to_string()]; }
        if to == '4'{ return vec!["^<<A".to_string(), "<<^A".to_string()]; }
        if to == '5'{ return vec!["^<A".to_string(), "<^A".to_string()]; }
        if to == '6'{ return vec!["^A".to_string()]; }
        if to == '7'{ return vec!["<<^^A".to_string(), "^^<<A".to_string()]; }
        if to == '8'{ return vec!["^^<A".to_string(), "<^^A".to_string()]; }
        if to == '9'{ return vec!["^^A".to_string()]; }
    }
    if from == '4' {
        if to == 'A'{ return vec![">>vvA".to_string()]; }
        if to == '0'{ return vec![">vvA".to_string()]; }
        if to == '1'{ return vec!["vA".to_string()]; }
        if to == '2'{ return vec!["v>A".to_string(), ">vA".to_string()]; }
        if to == '3'{ return vec!["v>>A".to_string(), ">>vA".to_string()]; }
        if to == '4'{ return vec!["A".to_string()]; }
        if to == '5'{ return vec![">A".to_string()]; }
        if to == '6'{ return vec![">>A".to_string()]; }
        if to == '7'{ return vec!["^A".to_string()]; }
        if to == '8'{ return vec!["^>A".to_string(), ">^A".to_string()]; }
        if to == '9'{ return vec![">>^A".to_string(), "^>>A".to_string()]; }
    }
    if from == '5' {
        if to == 'A'{ return vec![">vvA".to_string(), "vv>A".to_string()]; }
        if to == '0'{ return vec!["vvA".to_string()]; }
        if to == '1'{ return vec!["v<A".to_string(), "<vA".to_string()]; }
        if to == '2'{ return vec!["vA".to_string()]; }
        if to == '3'{ return vec!["v>A".to_string(), ">vA".to_string()]; }
        if to == '4'{ return vec!["<A".to_string()]; }
        if to == '5'{ return vec!["A".to_string()]; }
        if to == '6'{ return vec![">A".to_string()]; }
        if to == '7'{ return vec!["^<A".to_string(), "<^A".to_string()]; }
        if to == '8'{ return vec!["^A".to_string()]; }
        if to == '9'{ return vec![">^A".to_string(), "^>A".to_string()]; }
    }
    if from == '6' {
        if to == 'A'{ return vec!["vvA".to_string()]; }
        if to == '0'{ return vec!["vv<A".to_string(), "<vvA".to_string()]; }
        if to == '1'{ return vec!["v<<A".to_string(), "<<vA".to_string()]; }
        if to == '2'{ return vec!["v<A".to_string(), "<vA".to_string()]; }
        if to == '3'{ return vec!["vA".to_string()]; }
        if to == '4'{ return vec!["<<A".to_string()]; }
        if to == '5'{ return vec!["<A".to_string()]; }
        if to == '6'{ return vec!["A".to_string()]; }
        if to == '7'{ return vec!["^<<A".to_string(), "<<^A".to_string()]; }
        if to == '8'{ return vec!["^<A".to_string(), "<^A".to_string()]; }
        if to == '9'{ return vec!["^A".to_string()]; }
    }
    if from == '7' {
        if to == 'A'{ return vec![">>vvvA".to_string()]; }
        if to == '0'{ return vec![">vvvA".to_string()]; }
        if to == '1'{ return vec!["vvA".to_string()]; }
        if to == '2'{ return vec!["vv>A".to_string(), ">vvA".to_string()]; }
        if to == '3'{ return vec!["vv>>A".to_string(), ">>vvA".to_string()]; }
        if to == '4'{ return vec!["vA".to_string()]; }
        if to == '5'{ return vec!["v>A".to_string(), ">vA".to_string()]; }
        if to == '6'{ return vec!["v>>A".to_string(), ">>vA".to_string()]; }
        if to == '7'{ return vec!["A".to_string()]; }
        if to == '8'{ return vec![">A".to_string()]; }
        if to == '9'{ return vec![">>A".to_string()]; }
    }
    if from == '8' {
        if to == 'A'{ return vec![">vvvA".to_string(), "vvv>A".to_string()]; }
        if to == '0'{ return vec!["vvvA".to_string()]; }
        if to == '1'{ return vec!["vv<A".to_string(), "<vvA".to_string()]; }
        if to == '2'{ return vec!["vvA".to_string()]; }
        if to == '3'{ return vec!["vv>A".to_string(), ">vvA".to_string()]; }
        if to == '4'{ return vec!["v<A".to_string(), "<vA".to_string()]; }
        if to == '5'{ return vec!["vA".to_string()]; }
        if to == '6'{ return vec!["v>A".to_string(), ">vA".to_string()]; }
        if to == '7'{ return vec!["<A".to_string()]; }
        if to == '8'{ return vec!["A".to_string()]; }
        if to == '9'{ return vec![">A".to_string()]; }
    }
    if from == '9' {
        if to == 'A'{ return vec!["vvvA".to_string()]; }
        if to == '0'{ return vec!["vvv<A".to_string(), "<vvvA".to_string()]; }
        if to == '1'{ return vec!["vv<<A".to_string(), "<<vvA".to_string()]; }
        if to == '2'{ return vec!["vv<A".to_string(), "<vvA".to_string()]; }
        if to == '3'{ return vec!["vvA".to_string()]; }
        if to == '4'{ return vec!["v<<A".to_string(), "<<vA".to_string()]; }
        if to == '5'{ return vec!["v<A".to_string(), "<vA".to_string()]; }
        if to == '6'{ return vec!["vA".to_string()]; }
        if to == '7'{ return vec!["<<A".to_string()]; }
        if to == '8'{ return vec!["<A".to_string()]; }
        if to == '9'{ return vec!["A".to_string()]; }
    }
    
    return options;
}

fn processNumber(command: String) -> Vec<String> {
    let mut current: char = 'A';
    let mut output: Vec<String> = Vec::new();
    // For character in string
    for c in command.chars() {
        //println!("starting character: {}", c);
        let paths = keypad_options(current, c);
        //println!("New paths: {:?}", paths);
        if output.len() == 0 {
            output = paths;
        } else {
            if paths.len() == 1 {
                // append path[0] to all elements in output
                let mut new_output: Vec<String> = Vec::new();
                for o in output {
                    new_output.push(o + &paths[0]);
                }
                output = new_output;
            } else {
                let mut new_output: Vec<String> = Vec::new();
                for o in output {
                    for path in paths.clone() {
                        new_output.push(o.clone() + &path.clone());
                    }
                }

                output = new_output
            }
        }
        //println!("New output: {:?}", output);
        current = c;
    }

    return output;
}


#[cached(
    type = "UnboundCache<String, Vec<String>>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ command.to_string() }"#
)]
fn helper(command: String) -> Vec<String> {
    let mut current: char = 'A';
    
    
    
    let mut output: Vec<String> = Vec::new();
    for c in command.chars() {
        //println!("starting character: {}", c);
        let paths = arrow_options(current, c);
        //println!("New paths: {:?}", paths);
        if output.len() == 0 {
            output = paths;
        } else {
            if paths.len() == 1 {
                // append path[0] to all elements in output
                let mut new_output: Vec<String> = Vec::new();
                for o in output {
                    let newp1 = o.clone() + &paths[0].clone();
                    new_output.push(newp1);
                }
                output = new_output;
            } else {
                //println!("Merging!!");
                // append path[0] to all elements in output
                let mut new_output: Vec<String> = Vec::new();
                for o in output {
                    let newp1 = o.clone() + &paths[0].clone();
                    let newp2 = o.clone() + &paths[1].clone();
                    //println!("o: {} newp1: {} newp2: {}", o, newp1, newp2);
                    new_output.push(newp1);
                    new_output.push(newp2);
                }
                
                output = new_output;
            }
        }
        //println!("output after processing {}: {:?}", c, output);

        current = c;
    }
    return output;
}

#[cached(
    type = "UnboundCache<String, i128>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{}{}", command, depth) }"#
)]
fn processArrows(command: String, depth: i32) -> i128 {
    //println!("Depth: {} {}",  depth, command);
    //println!("Depth: {} {}",  depth, command);
    println!("Starting at Depth: {} {}",  depth, command);
    let mut total: i128 = 0;
    if depth == 0 {
        println!("  - returning {}", command.len());
        return command.len() as i128; 
    }
    //println!("Command: {}", command);
    let output = helper(command);
    println!("  - Output of helper: {:?}", output);
    //println!("Output: {:?}", output);
    // We have all new possible paths -> call ourselves for each one and add all the results together
    let mut minimal_score: i128 = i128::MAX;
    for o in output {
        //println!("New command: {}", o);
        let mut subtotal: i128 = 0;
        //minimal_score = i128::MAX;
        // split string o on "A" and call processArrows on each part
        let mut parts: Vec<&str> = o.split("A").collect();
        let short_string= "".to_string();
        //println!("total: {:?}", o);
        // remove last element from parts
        //println!("parts: {:?}", parts);
        parts.pop();
        println!("  -parts: {:?}", parts);
        for p in parts {
            
            //if p.len() == 0 { continue; }
            let new_com: String = p.to_string();
            //println!("New command: {}", new_com);
            let mut new_com = new_com.clone();
            new_com.push('A');
            //println!("p: {}", new_com);
            
            //println!("New command: {}", new_com);
            subtotal += processArrows(new_com, depth - 1);
            
        }
        println!("  - Subtotal: {}", subtotal);
        if subtotal < minimal_score {
            minimal_score = subtotal;
        }   
    }

    //println!("Depth {} Minimal score: {}", depth, minimal_score);
    return minimal_score;

}

fn main () {

    let lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-21-1/src/input.txt");
    let mut total: i128 = 0;
    
    for line in lines {
        //println!("starting line: {}", line);
        //println!("starting line: {}", line);
        // remove A from line and store in number as i32
        let number = line.replace("A", "").parse::<i32>().unwrap();
        let arrowcommands: Vec<String> = processNumber(line);
        //println!("Arrow commands: {:?}", arrowcommands);
        let mut min_size: i128 = i128::MAX;
        for ac in arrowcommands {
            //println!("Arrow command: {}", ac);
            println!("Arrow command: {}", ac);
            let new_size = processArrows(ac.clone(), 25 as i32);
            print!("BACK AT BASE size: {}", new_size);
            if new_size < min_size {
                min_size = new_size;
            }
            
            
        }
        println!("Min size: {}", min_size);
        total = total + (min_size * number as i128);

    }

    println!("Total: {}", total);
}