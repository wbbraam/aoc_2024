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
    let mut robots: Vec<(i32, i32, i32, i32, i32)> = Vec::new();
    let ID = 0;
    let POS_X = 1;
    let POX_Y = 2;
    let SPD_X = 3;
    let XPD_Y = 4;

    let FIELD_LEN_X = 101;
    let FIELD_LEN_Y = 103;

    let STEPS = 100;

    //Allocate what is needed to complete and read the file.
    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-14-1/src/input1.txt");

    // loop over all lines
    let mut count = 0;
    for line in all_lines {
        // p=3,0 v=-2,-2
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let cap = re.captures(&line).unwrap();
        let pos_x = cap[1].parse::<i32>().unwrap();
        let pos_y = cap[2].parse::<i32>().unwrap();
        let spd_x = cap[3].parse::<i32>().unwrap();
        let spd_y = cap[4].parse::<i32>().unwrap();
        robots.push((count, pos_x, pos_y, spd_x, spd_y));
        count += 1;
    }
    println!("{:?}", robots);

    let mut final_positions: Vec<(i32, i32)> = Vec::new();

    for robot in robots.iter_mut() {
        let mut x: i32 = (robot.1 + robot.3 * STEPS) % FIELD_LEN_X;
        if x < 0 {
            x = FIELD_LEN_X + x;
        }

        let mut y: i32 = (robot.2 + robot.4 * STEPS) % FIELD_LEN_Y;
        if y < 0 {
            y = FIELD_LEN_Y + y;
        }
        final_positions.push((x, y));
        //println!("robot {:?}", robot);
        println!("ends up at: {}, {}", x, y);

    }

    println!("{:?}", final_positions);
    let xmiddle = (FIELD_LEN_X - 1) / 2;
    let ymiddle = (FIELD_LEN_Y - 1) / 2;
    println!("middle: {}, {}", xmiddle, ymiddle);

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for pos in final_positions.iter() {
        if pos.0 < xmiddle && pos.1 < ymiddle {
            q1 += 1;
        } else if pos.0 > xmiddle && pos.1 < ymiddle {
            q2 += 1;
        } else if pos.0 < xmiddle && pos.1 > ymiddle {
            q3 += 1;
        } else if pos.0 > xmiddle && pos.1 > ymiddle {
            q4 += 1;
        }
    }
    println!("q1: {}, q2: {}, q3: {}, q4: {}", q1, q2, q3, q4);

    let total: i64 = q1 * q2 * q3 * q4;
    println!("total: {}", total);


}