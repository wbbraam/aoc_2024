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

// Function that finds first digit in String
fn first_number_in_string(s: &str) -> i32 {
    s.chars().find_map(|c| c.to_digit(10)).unwrap() as i32
}

// Function taht finds last digit in String
fn last_number_in_string(s: &str) -> i32 {
    s.chars().rev().find_map(|c| c.to_digit(10)).unwrap() as i32
}

fn concat_two_integers_to_string(a: i32, b: i32) -> String {
    let a_str = a.to_string();
    let b_str = b.to_string();
    let result = a_str + &b_str;
    result
}

fn main() {

    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/ex1/src/input1.txt");
    let mut total: i32 = 0;

    for line in all_lines {
        let number = concat_two_integers_to_string(first_number_in_string(&line), last_number_in_string(&line));

        println!("{:?}, {}", line, number);
        total += number.parse::<i32>().unwrap();
        
    }
    println!("{}", total);

}
