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
    let lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-23-1/src/test.txt");
    
    // for each line in lines, split the line on a "-" into 2 variables and store them in a tuple
    let mut combinations: Vec<(String, String)> = Vec::new();
    let mut computers: Vec<String> = Vec::new();
    let mut networks: Vec<(String, String, String)> = Vec::new();

    for line in lines {
        let split_line: Vec<String> = line.split("-").map(|s| s.to_string()).collect();
        println!("{:?}", split_line);
        combinations.push((split_line[0].clone(), split_line[1].clone()));
        combinations.push((split_line[1].clone(), split_line[0].clone()));
        computers.push(split_line[0].clone());
        computers.push(split_line[1].clone());

    } 
    computers.sort();
    computers.dedup();

    combinations.sort();
    println!("{:?}", combinations);

    // loop over all computers and print them
    for computer in computers {
       // println!("{}", computer);
        // find all combinations where computers is the first element and collec the second elements in a vector
        let mut connected_computers: Vec<String> = Vec::new();
        for combination in &combinations {
            if combination.0 == computer {
                connected_computers.push(combination.1.clone());
            }
        }
        //println!("{:?}", connected_computers);
        // find all possible unique pairs in the values of connected_computers
        let mut unique_pairs: Vec<(String, String)> = Vec::new();
        for i in 0..connected_computers.len() {
            for j in i+1..connected_computers.len() {
                unique_pairs.push((connected_computers[i].clone(), connected_computers[j].clone()));
            }
        }
        //println!("{:?}", unique_pairs);

        // loop over all unique pairs and print them
        for pair in unique_pairs {
            //println!("{:?}", pair);
            //check if pair is in combinations i so print it
            for combination in &combinations {
                if combination.0 == pair.0 && combination.1 == pair.1 {
                    // check if any combination of computer, combination.0 and combination.1 is already in networks
                    // if not add it to networks
                    let mut found = false;
                    if networks.contains(&(computer.clone(), combination.0.clone(), combination.1.clone())) ||
                        networks.contains(&(computer.clone(), combination.1.clone(), combination.0.clone())) ||
                        networks.contains(&(combination.0.clone(), computer.clone(), combination.1.clone())) ||
                        networks.contains(&(combination.0.clone(), combination.1.clone(), computer.clone())) ||
                        networks.contains(&(combination.1.clone(), computer.clone(), combination.0.clone())) ||
                        networks.contains(&(combination.1.clone(), combination.0.clone(), computer.clone())) {

                        found = true;
                    }


                    if !found {
                        networks.push((computer.clone(), combination.0.clone(), combination.1.clone()));
                    }
                    
                }
            }
        }


        
    }

    let mut total = 0;
    for network in networks {
        // if network.0 starts with a t, print it
        if network.0.starts_with("t") ||
           network.1.starts_with("t") ||
           network.2.starts_with("t") {
            println!("{:?}", network);
            total += 1;
           }
    }
    println!("{}", total);




    


}