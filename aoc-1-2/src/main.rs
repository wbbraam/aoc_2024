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

// Function that calculates the amount of time an I32 occurs in a vector of I32 values
// list is passed as a reference to avoid moving it (fix after error during compilation, LEARN!!!)
fn count_occurrences(list: &Vec<i32>, value: i32) -> i32 {
    let mut count = 0;
    for item in list.iter() {
        if *item == value {
            count = count + 1;
        }
    }
    return count;
}

fn main() {

    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-1-1/src/input1.txt");
    let mut total: i32 = 0;
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in all_lines {
        println!("{:?}", line);
        //split line into two strings
        let mut split = line.split("   ");
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        list1.push(first.parse::<i32>().unwrap());
        list2.push(second.parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    for item in list1.iter(){
        let mut simscore = item * count_occurrences(&list2, *item);
        total = total + simscore
    }

    println!("{}", total);


}
