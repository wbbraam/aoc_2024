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

fn mix_and_prune(secret: i64, result: i64) -> i64 {
    //println!("Secret: {}", 100000000 % 16777216);
    // mix in.
    let mixed: i64 = result ^ secret;
    // prune
    let pruned: i64 = mixed % 16777216;
   // println!("Pruned: {}", pruned);
    return pruned;
}

fn calculate_next_seret(s: i64) -> i64 {
            // STEP 1
            let mut secret = s;
            let result_s1 = secret * 64;
            secret = mix_and_prune(secret, result_s1);
    
            // STEP 2
            // diviced numberp[i] by 32 and round to nearest integer
            let divided: f64 = secret as f64 / 32.0 as f64;
            //println!("Divided: {}", divided);
            let rounded: i64 = divided.floor() as i64;
            //println!("Rounded: {}", rounded);
            secret = mix_and_prune(secret, rounded);
    
            let multiplied: i64 = secret * 2048;
            secret = mix_and_prune(secret, multiplied);
            return secret;

}

fn rotate_in (sequence: (i32, i32, i32, i32), difference: i32) -> (i32, i32, i32, i32) {
    let mut new_sequence: (i32, i32, i32, i32) = (sequence.1, sequence.2, sequence.3, difference);
    return new_sequence;
}

fn main() {
    let lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-22-1/src/input.txt");
    println!("{:?}", lines);
    // comvert lines to vector of integers
    let mut numbers: Vec<i64> = lines.iter().map(|x| x.parse().unwrap()).collect();
    println!("{:?}", numbers);

    // while loop over numbers
    let mut i = 0;
    let mut total: i64 = 0;
    
    let mut all_sequences: Vec<((i32, i32, i32, i32), usize, i32)> = Vec::new();
    let mut sequence: (i32, i32, i32, i32) = (255, 255, 255, 255);
    while i < numbers.len() {

        sequence = (255, 255, 255, 255);
        println!("Secret {} {:?}", i, numbers[i]);
        let mut last_lowest: i32 = (numbers[i] % 10) as i32;
        for j in 0..2000 {
            numbers[i] = calculate_next_seret(numbers[i]);
              
            // caulclate lowest digit of numbers[i]
            let lowest: i32 = (numbers[i] % 10) as i32;
            
            let difference = lowest - last_lowest;
            last_lowest = lowest;

            //println!("Secret {:?} -> {} -> {}", numbers[i], lowest, difference);
            sequence = rotate_in(sequence, difference);
            //println!("Sequence: {:?}", sequence);
            // if sequence contains a 255 we continue as we do not have a valid one yet
            if sequence.0 == 255 {
                continue;
            } 
            if all_sequences.len() == 0 {
                all_sequences.push((sequence,i,  lowest));
                continue;
            }
            let mut found: bool = false;
            for k in 0..all_sequences.len() {
                // if first item in all_sequences[k] is equal to sequence
                if all_sequences[k].0 == sequence && all_sequences[k].1 != i {
                    // add lowest to all_sequences[k]
                    //println!("Adding {} to all_sequences: {:?}", lowest, all_sequences[k]);
                    all_sequences[k].2 = all_sequences[k].2 + lowest;
                    all_sequences[k].1 = i;
                    found = true;
                    break;
                } 
                if all_sequences[k].0 == sequence && all_sequences[k].1 == i {
                    found = true;
                    break;
                }
            }
            // add sequence to all_sequences
            //println!("New sequence{:?}", sequence);
            if !found {
                all_sequences.push((sequence, i, lowest));
            }
            
            //println!("All sequences: {:?}", all_sequences);
        }


        
        i = i + 1;
    }
    // find the sequence with the highest value
    all_sequences.sort_by(|a, b| b.1.cmp(&a.1));
    // reverse all_sequences
    all_sequences.reverse();
    let mut highest: i32 = 0;
    let mut highest_sequence: (i32, i32, i32, i32) = (255, 255, 255, 255);
    for k in 0..all_sequences.len() {
        //println!("Sequence: {:?}", all_sequences[k]);
        if all_sequences[k].2 > highest {
            highest = all_sequences[k].2;
            highest_sequence = all_sequences[k].0;
        }
    }
    println!("Highest: {:?}", highest);
    println!("Highest sequence: {:?}", highest_sequence);

}