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

    let all_lines: Vec<String> = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-7-1/src/input1.txt");

    let mut number: i16 = 2;
    // convert number into a string of the complete binary number  
    let mut binary_number: String = format!("{:016b}", number);
    println!("Binary number: {}", binary_number);

    let mut total = 0;

    //iterate over all_lines   
    for line in all_lines.iter() {
        // take the number in line before the : sign
        let target: i64 = line.split(":").next().unwrap().parse().unwrap();
        // parse the space seperated string after : into a vecor of i32
        let numbers: Vec<i64> = line.split(":").nth(1).unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        // calculate the length of numbers by the power of 2
        let combinations: i32 = 2_i32.pow(numbers.len() as u32 - 1);
        let amount_of_numbers: i32 = numbers.len() as i32;
        
        println!("Target: {}", target);
        println!("Numbers: {:?}", numbers);
        println!("combinations: {}", combinations);
        println!("Amount of numbers: {}", amount_of_numbers);

        // iterate from 0 to combinations
        for i in 0..combinations {
            let operatorbits: String = format!("{:016b}", i); 
            println!("Operatorbits: {}", operatorbits);
            let mut sum: i64 = numbers[0];
            // loop over the second till last item in numbers
            for j in 1..amount_of_numbers {
                // if the operatorbits is 1 add the number to the sum
               
                if operatorbits.chars().rev().nth(j as usize - 1).unwrap() == '1' {
                    println!("Adding: {}", numbers[j as usize]);
                    sum = sum + numbers[j as usize];
                } else {
                    println!("multiplying: {}", numbers[j as usize]);
                    sum = sum * numbers[j as usize];
                }
            }
            println!("Sum: {}", sum);
            if sum == target {
                println!("Found the target: {}", target);
                total = total + target;
                break;
            }
        }


    }
    println!("Total: {}", total);
}