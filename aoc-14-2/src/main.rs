// This generates images of the robot positions in png format
// first I ran it for 1-500 and saw a pattern emerging on certain modulo
// next I ran it for 1-50k only for i values that match those modulo values
// scrolling through the images I found the one that looked like the christmas tree

use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use regex::Regex;
extern crate image;

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

    
    //Loop 200 times
    for i in 0..50000 {
        if (i % 101 != 98) {
            continue;
        }
        let mut final_positions: Vec<(i32, i32)> = Vec::new();
        println!("Iteration: {}", i);
        for robot in robots.iter_mut() {
            let mut x: i32 = (robot.1 + robot.3 * i) % FIELD_LEN_X;
            if x < 0 {
                x = FIELD_LEN_X + x;
            }

            let mut y: i32 = (robot.2 + robot.4 * i) % FIELD_LEN_Y;
            if y < 0 {
                y = FIELD_LEN_Y + y;
            }
            final_positions.push((x, y));
            //println!("robot {:?}", robot);
        }
        //loop over FIELD_LEN_x and FIELD_LEN_Y
        let mut buffer: Vec<u8> = Vec::new();
        for y in 0..FIELD_LEN_Y {
            for x in 0..FIELD_LEN_X {
                let mut found = false;
                for pos in final_positions.iter() {
                    if pos.0 == x && pos.1 == y {
                        buffer.push(255);
                        buffer.push(255);
                        buffer.push(255);
                        buffer.push(255);
                        //print!("#");
                        found = true;
                        break;
                    }
                }
                if !found {
                    //print!(".");
                    buffer.push(0);
                    buffer.push(0);
                    buffer.push(0);
                    buffer.push(0);
                }
            }
            //println!();
        }
        
        image::save_buffer(&Path::new(&("images/".to_owned()+&i.to_string()+".png")), &buffer, FIELD_LEN_X as u32, FIELD_LEN_Y as u32, image::ExtendedColorType::Rgba8);
        
    
    }

}