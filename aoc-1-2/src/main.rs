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
    // If I have the time, change this to functional programming instead of iterating.
    for item in list.iter() {
        if *item == value {
            count = count + 1;
        }
    }
    return count;
}

fn main() {
    //Allocate what is needed to complete and read the file.
    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-1-1/src/input1.txt");
    let mut total: i32 = 0;
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    //Preprocessing, parsing the lines into two lists
    for line in all_lines {
        println!("{:?}", line);
        //split line into two strings
        let mut split = line.split("   "); //There has to be a better way!
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        list1.push(first.parse::<i32>().unwrap());
        list2.push(second.parse::<i32>().unwrap());
    }
    
    //This time we iterate over te left list and find the amount of occurences in the other list
    for item in list1.iter(){
        // Took out occurence counting to a function, to keep it readable (and reusable later on :D)
        let mut simscore = item * count_occurrences(&list2, *item); //Pass as reference else it fails horribly
        total = total + simscore
    }

    println!("{}", total);


}
