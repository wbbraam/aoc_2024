use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use regex::Regex;

//Function taht reads file and returns it as one string
fn read_file_to_string(filename: impl AsRef<Path>) -> String {
    let file = File::open(filename).expect("no such file");
    let mut buf = BufReader::new(file);
    let mut contents = String::new();
    buf.read_to_string(&mut contents).expect("Could not read file");
    //remove end of lines from contents 
    contents = contents.replace("\n", "");
    return contents;
}

// Function that reads file and returns a vector of strings
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

//function that finds the first number in a string  and returns it as an integer
fn find_first_number(line: &str) -> i32 {
    let re = Regex::new(r"\d{1,3}").unwrap();
    let first = re.find(line).unwrap();
    let first_str = &line[first.start()..first.end()];
    let first_int = first_str.parse::<i32>().unwrap();
    return first_int;
}

//function that finds two numbers in a string and returns them multiplied as an integer
fn find_multiplication(line: &str) -> i32 {
    let re = Regex::new(r"\d{1,3}").unwrap();
    let numbers = re.find_iter(&line);
    let mut number1: i32 = 0;
    let mut number2: i32 = 0;
    for number in numbers {
        if number1 == 0 {
            number1 = number.as_str().parse::<i32>().unwrap();
        } else {
            number2 = number.as_str().parse::<i32>().unwrap();
        }
    }

    return number1 * number2;
}

fn main() {
    //Allocate what is needed to complete and read the file.
    let all_lines = read_file_to_string("/Users/ES94CO/Developer/rust/Advent/aoc-3-1/src/input1.txt");
    let mut total: i32 = 0;

    //find replace values in all_lines matching regex   
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    //replace all matches of regex in all_lines with " " and store in new string
    let new_string = re.replace_all(&all_lines, " ");

    //define regex to find correct matches
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    //find all matches of regex in the file
    let matches = re.find_iter(&new_string);
    //iterate over the matches
    for m in matches {
        
        //get the match as a string
        let match_str = &new_string[m.start()..m.end()];
        println!("{}", match_str);
        let multiple = find_multiplication(match_str);

        total = total + multiple;
    }
    

    println!("{}", total);


}

