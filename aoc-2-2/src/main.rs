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

fn checksafe(splitted: Vec<String>) -> bool {
    //split line into two strings
    
    let mut iter = splitted.into_iter();
    let mut last_report = iter.next().unwrap().parse::<i32>().unwrap();
    let mut direction: String = "unset".to_string();
    let mut safe_report = true;

    for report in iter {
        let newreport = report.parse::<i32>().unwrap();
        let difference: i32 = newreport - last_report;
        if direction == "unset" {
            if difference < 0 {
                direction = "down".to_string();
            } else if difference > 0 {
                direction = "up".to_string();
            } else {
                safe_report = false;
                break
            }
        }

        if difference.abs() < 1 || difference.abs() > 3 {
            safe_report = false;
            break;
        }
        if difference < 0 && direction == "up" {
            //println!("Going down instead of up{} {}", difference, direction);
            safe_report = false;
            break;
        }
        if difference > 0 && direction == "down" {
            //println!("Going up instead of down{} {}", difference, direction);
            safe_report = false;
            break;
        }
        last_report = newreport;
    }

    return safe_report
}

fn main() {
    //Allocate what is needed to complete and read the file.
    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-2-1/src/input1.txt");
    let mut total: i32 = 0;


    //Preprocessing, parsing the lines into two lists
    for line in all_lines {
        let original_vector: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        // Need to clone the Vector else we cannot use it after the check.
        // TODO: This needs to be refactored!!!
        if checksafe(original_vector.clone()) {
            total = total + 1;
        } else{
            //So we know it is unsafe. Lets check if removing 1 element might fix this issue
            //to start we loop oer the current vector  
            for i in 0..original_vector.len() {
                //Now create a new vector, except the element we are pointing at   
                let new_vector: Vec<String> = original_vector.iter().enumerate().filter(|&(index, _)| index != i).map(|(_, s)| s.to_string()).collect();
                if checksafe(new_vector) {
                    total = total + 1;
                    break;
                }
            }
        }

    }
    

    println!("{}", total);


}
