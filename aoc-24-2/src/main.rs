use core::net;
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


fn main() {
    let lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-24-1/src/input.txt");
    let mut firstpart: bool = true;

    let mut nodes:Vec<(String, String, String)> = Vec::new();
    let mut edges:Vec<(String, String)> = Vec::new();
    let mut notdone:Vec<(String, String, String)> = Vec::new();



    for line in &lines {
        
        if line == "" {
            firstpart = false;
            continue;
        }

        if firstpart {
            //split line on ": "
            let parts: Vec<&str> = line.split(": ").collect();
            nodes.push((parts[0].to_string(), "None".to_string(), parts[1].to_string()));
            

        } else {
            // x00 AND y00 -> z00
            let parts: Vec<&str> = line.split(" "). collect();
            nodes.push((parts[4].to_string(), parts[1].to_string(), " ".to_string()));
            edges.push((parts[0].to_string(), parts[4].to_string()));
            edges.push((parts[2].to_string(), parts[4].to_string()));
            notdone.push((parts[4].to_string(), parts[1].to_string(), " ".to_string()));
            if parts[4].starts_with("z") && parts[1] != "XOR" {
                println!("WRONG1! {:?}", parts);
            }

            if parts[1] == "XOR" {
                if (parts[0].starts_with("x") || parts[0].starts_with("y")) && (parts[2].starts_with("x") || parts[2].starts_with("y"))  {
                    //println!("OK! {:?}", parts);
                } else {
                    if parts[4].starts_with("z") {
                        //println!("OK 2! {:?}", parts);
                    } else {
                        println!("WRONG2! {:?}", parts);
                    }
                }
            }
        }
        
    }



    for edge in edges.clone() {
        //println!("Edge {:?}", edge)
        
    }

    for node in nodes.clone() {
        //println!("Node {:?}", node)
    }

    println!("IF z45 is in the output. That one might be good!");
    println!("Compelete by checking XOR with input of x and y to proceed into XOR or AND");
    println!("Check if AND with input x and y does proceed into OR");

}