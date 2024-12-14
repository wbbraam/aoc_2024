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

        let button_a_x: f64 = 0.0;
        let button_a_y: f64 = 0.0;
        let button_b_x: f64 = 0.0;
        let button_b_y: f64 = 0.0;
        let prize_x: f64 = 0.0;
        let prize_y: f64 = 0.0;

        //regex to find numbers in line1
        let re = Regex::new(r"\d+").unwrap();
        let mut numbers = re.find_iter(line1);
        let button_a_x = numbers.next().unwrap().as_str().parse::<f64>().unwrap();
        let button_a_y = numbers.next().unwrap().as_str().parse::<f64>().unwrap();
        println!("button a: {} {}", button_a_x, button_a_y);

        //regex to find numbers in line2
        let re = Regex::new(r"\d+").unwrap();
        let mut numbers = re.find_iter(line2);
        let button_b_x = numbers.next().unwrap().as_str().parse::<f64>().unwrap();
        let button_b_y = numbers.next().unwrap().as_str().parse::<f64>().unwrap();
        println!("button b: {} {}", button_b_x, button_b_y);

        //regex to find numbers in line3
        let re = Regex::new(r"\d+").unwrap();
        let mut numbers = re.find_iter(line3);
        let prize_x = numbers.next().unwrap().as_str().parse::<f64>().unwrap() + 10000000000000.0;
        
        let prize_y = numbers.next().unwrap().as_str().parse::<f64>().unwrap() + 10000000000000.0;
        println!("prize: {} {}", prize_x, prize_y);

        let press_a: f64 = (prize_x * button_b_y - button_b_x * prize_y) / 
                      (button_b_y * button_a_x - button_a_y * button_b_x);

        let press_b: f64  = (prize_y * button_a_x - prize_x * button_a_y) /
                      (button_a_x * button_b_y - button_b_x * button_a_y);

        if press_a.fract() > 0.0 || press_b.fract() > 0.0 {
            continue;
        }

        let lowest_score = 3 * (press_a as i64) + (press_b as i64);
        println!("Lowest score: {}", lowest_score);
        totalscore += lowest_score;
        

    }
    println!("Total score: {}", totalscore);
}