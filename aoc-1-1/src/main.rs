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
    //Allocate what is needed to complete and read the file.
    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-1-1/src/input1.txt");
    let mut total: i32 = 0;
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    //Preprocessing, parsing the lines into two lists
    for line in all_lines {
        println!("{:?}", line);
        //split line into two strings
        let mut split = line.split("   ");
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        list1.push(first.parse::<i32>().unwrap());
        list2.push(second.parse::<i32>().unwrap());
    }
    
    //So we need to work on sorted lists. lets sort these vectors
    list1.sort();
    list2.sort();

    //Iterate over both vectors at once. And calculate the difference between the values
    for (l1, l2) in list1.iter().zip(list2.iter()){
        let diff = l1 - l2;
        // Abs for absolute value
        total = total + diff.abs();
    }

    println!("{}", total);


}
