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
    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-2-1/src/input1.txt");
    let mut total: i32 = 0;


    //Preprocessing, parsing the lines into two lists
    for line in all_lines {
        
        //split line into two strings
        let mut splitted = line.split(" ");
        let mut lastreport = splitted.next().unwrap().parse::<i32>().unwrap();
        let mut direction: String = "unset".to_string();
        let mut safeReport = true;

        println!("{:?}", lastreport);
        for report in splitted {
            let newreport = report.parse::<i32>().unwrap();
            let difference: i32 = newreport - lastreport;
            if direction == "unset" {
                if difference < 0 {
                    direction = "down".to_string();
                } else if difference > 0 {
                    direction = "up".to_string();
                } else {
                    safeReport = false;
                    break
                }
            }

            if difference.abs() < 1 || difference.abs() > 3 {
                safeReport = false;
                break;
            }
            if difference < 0 && direction == "up" {
                println!("Going down instead of up{} {}", difference, direction);
                safeReport = false;
                break;
            }
            if difference > 0 && direction == "down" {
                println!("Going up instead of down{} {}", difference, direction);
                safeReport = false;
                break;
            }
            lastreport = newreport;
        }
        if safeReport {
            println!("Safe:   {:?}", line);
            total = total + 1;
        } else {
            println!("Unsafe: {:?}", line);
        }

    }
    

    println!("{}", total);


}
