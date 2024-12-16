use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use regex::Regex;

// Function that reads file and returns a vector of strings
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    //Allocate what is needed to complete and read the file.
    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-13-1/src/input1.txt");
    // remove empty lines
    let mut lines = all_lines.iter().filter(|&x| x != "").collect::<Vec<_>>();
    println!("{:?}", lines);

    let mut totalscore = 0;
    // Loop per 3 lines
    for i in (0..lines.len()).step_by(3) {
        let line1 = &lines[i];
        let line2 = &lines[i+1];
        let line3 = &lines[i+2];
        println!("{} {} {}", line1, line2, line3);

        let button_a_x: i32 = 0;
        let button_a_y: i32 = 0;
        let button_b_x: i32 = 0;
        let button_b_y: i32 = 0;
        let prize_x: i32 = 0;
        let prize_y: i32 = 0;

        //regex to find numbers in line1
        let re = Regex::new(r"\d+").unwrap();
        let mut numbers = re.find_iter(line1);
        let button_a_x = numbers.next().unwrap().as_str().parse::<i32>().unwrap();
        let button_a_y = numbers.next().unwrap().as_str().parse::<i32>().unwrap();
        println!("button a: {} {}", button_a_x, button_a_y);

        //regex to find numbers in line2
        let re = Regex::new(r"\d+").unwrap();
        let mut numbers = re.find_iter(line2);
        let button_b_x = numbers.next().unwrap().as_str().parse::<i32>().unwrap();
        let button_b_y = numbers.next().unwrap().as_str().parse::<i32>().unwrap();
        println!("button b: {} {}", button_b_x, button_b_y);

        //regex to find numbers in line3
        let re = Regex::new(r"\d+").unwrap();
        let mut numbers = re.find_iter(line3);
        let prize_x = numbers.next().unwrap().as_str().parse::<i32>().unwrap();
        let prize_y = numbers.next().unwrap().as_str().parse::<i32>().unwrap();
        println!("prize: {} {}", prize_x, prize_y);

        let mut done = false;
        let mut presses_on_a = 1;
        let mut lowest_score = i32::MAX;
        
        while !done {
            //println!("presses_on_a: {}", presses_on_a);
            let tally_x: i32 = button_a_x * presses_on_a;
            let tally_y: i32 = button_a_y * presses_on_a;
            //println!("tally after pressing a: {} {}", tally_x, tally_y);
            // TODO what if this is enough

            //calculate of the remainder of prize_x could be found with button_b_x
            let modulo = (prize_x - tally_x) % button_b_x;
            if modulo == 0 {
                let presses_on_b = (prize_x - tally_x) / button_b_x;
                if (prize_y - tally_y) - (button_b_y * presses_on_b) == 0 {
                    let score = (presses_on_a * 3) + presses_on_b;
                    if score < lowest_score {
                        println!("score: {}", score);
                        lowest_score = score;
                    }
                }
            }
            presses_on_a += 1;
            if presses_on_a > 100 {
                done = true;
            }
        
        }
        if lowest_score == 2147483647 {
            println!("No solution found");
        } else {
            println!("lowest_score: {}", lowest_score);
            totalscore += lowest_score;
        }
        

    }
    println!("Total score: {}", totalscore);
}