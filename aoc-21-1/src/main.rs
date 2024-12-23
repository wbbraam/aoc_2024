use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use std::io::{self, Write};
use regex::Regex;
use cached::proc_macro::cached;
use cached::UnboundCache;

// Function that reads file and returns a vector of strings
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// Function that prints a grid from a vector of vectors of chars
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn find_pos_in_grid(grid: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    let mut pos = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, n) in row.iter().enumerate() {
            if *n == c {
                pos = (i, j);
            }
        }
    }
    pos
}

fn shortest_steps_in_numpad(grid: &Vec<Vec<char>>, startpos: (usize, usize), endpos: (usize, usize)) -> Vec<char> {
    // any move that is only horizontal or vertical process first
    let mut steps = 0;
    let mut path: Vec<char> = Vec::new();
    let mut dif_col = (endpos.1 as i32 - startpos.1 as i32);
    let mut dif_row = (endpos.0 as i32 - startpos.0 as i32);

    if (startpos.1 == endpos.1) {
        // only move vertical
        for i in 0..dif_row.abs() {
            if dif_row > 0 {
                path.push('V');
            } else {
                path.push('^');
            }
        }
    } else if (startpos.0 == endpos.0) {
        // only move horizontal
        for i in 0..dif_col.abs() {
            if dif_col > 0 {
                path.push('>');
            } else {
                path.push('<');
            }
        }
    } 

    // are we moving up and left print!
    if dif_col < 0 && dif_row < 0 {
        println!("Moving up and left");
        // if we start on bottom row we first move up
        if startpos.0 == 3 && endpos.1 == 0 {
            for i in 0..dif_row.abs() {
                path.push('^');
            }
            for i in 0..dif_col.abs() {
                path.push('<');
            }
        } else {
            for i in 0..dif_col.abs() {
                path.push('<');
            }
            for i in 0..dif_row.abs() {
                path.push('^');
            }
        }
    } 

    // are we moving up and right print!
    if dif_col > 0 && dif_row < 0 {
        println!("Moving up and right");
        // First move up then right
        for i in 0..dif_row.abs() {
            path.push('^');
        }
        for i in 0..dif_col.abs() {
            path.push('>');
        }


    }

    // are we moving down and left print!
    if dif_col < 0 && dif_row > 0 {
        println!("Moving down and left");
        // First move left then down
        for i in 0..dif_col.abs() {
            path.push('<');
        }
        for i in 0..dif_row.abs() {
            path.push('V');
        }
    }

    // are we moving down and right print!
    if dif_col > 0 && dif_row > 0 {
        println!("Moving down and right");
        if endpos.0 == 3 && startpos.1 == 0{
            // First move right then down
            for i in 0..dif_col.abs() {
                path.push('>');
            }
            for j in 0..dif_row.abs() {
                path.push('V');
            }
        } else {
            // First move down then right
            for i in 0..dif_row.abs() {
                path.push('V');
            }
            for i in 0..dif_col.abs() {
                path.push('>');
            }
        }

    }

    path.push('A');
    
    println!("Path: {:?}", path);

    return path
}


fn shortest_steps_in_arrow_pad(grid: &Vec<Vec<char>>, startpos: (usize, usize), endpos: (usize, usize)) -> Vec<char> {
    let mut steps = 0;
    let mut path: Vec<char> = Vec::new();
    let mut dif_col = (endpos.1 as i32 - startpos.1 as i32);
    let mut dif_row = (endpos.0 as i32 - startpos.0 as i32);
  
    

    // any move that is only horizontal or vertical process first
    if (startpos.1 == endpos.1) {
        // only move vertical
        for i in 0..dif_row.abs() {
            if dif_row > 0 {
                path.push('V');
            } else {
                path.push('^');
            }
        }
    } else if (startpos.0 == endpos.0) {
        // only move horizontal
        for i in 0..dif_col.abs() {
            if dif_col > 0 {
                path.push('>');
            } else {
                path.push('<');
            }
        }
    }

    // are we moving up and left print!
    if dif_col < 0 && dif_row < 0 {
        println!("Moving up and left");
        // first move left
        for i in 0..dif_col.abs() {
            path.push('<');
        }
        // then move up
        for i in 0..dif_row.abs() {
            path.push('^');
        }
    }

    // are we moving up and right print!
    if dif_col > 0 && dif_row < 0 {
        println!("Moving up and right");
        // First move right
        for i in 0..dif_col.abs() {
            path.push('>');
        }
        // then move up
        for i in 0..dif_row.abs() {
            path.push('^');
        }
    }

    // are we moving down and left print!
    if dif_col < 0 && dif_row > 0 {
        println!("Moving down and left");
        if endpos.1 == 0 {
            // first move down
            for i in 0..dif_row.abs() {
                path.push('V');
            }
            // then move left
            for i in 0..dif_col.abs() {
                path.push('<');
            }
        } else {
            // first move left
            for i in 0..dif_col.abs() {
                path.push('<');
            }
            // then move down
            for i in 0..dif_row.abs() {
                path.push('V');
            }
        }
    }

    // are we moving down and right print!
    if dif_col > 0 && dif_row > 0 {
        println!("Moving down and right");
        // first move down
        for i in 0..dif_row.abs() {
            path.push('V');
        }
        // then move right
        for i in 0..dif_col.abs() {
            path.push('>');
        }
    }



    
    path.push('A');
    
    path
}

fn main() {
    let numpad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A']
    ];
    let numpad_startpos = (3, 2);
    let mut multiply_values: Vec<i32> = Vec::new();

    let arrow_pad = vec![
        vec![' ', '^', 'A'],
        vec!['<', 'V', '>'],
    ];

    print_grid(&numpad);
    println!("Start position: {:?}", numpad[numpad_startpos.0][numpad_startpos.1]);
    print_grid(&arrow_pad);

    let lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-21-1/src/input.txt");

    println!("Lines: {:?}", lines);

    // First step is to loop over the lines and characters in the lines
    let mut currentpos = numpad_startpos;
    let mut total_steps_robot_1: Vec<char> = Vec::new();
    for line in lines {
        for c in line.chars() {
            //println!("NEW CHAR TO PROCESS: {}", c);
            // find position of character c in numpad
            let mut newpos = find_pos_in_grid(&numpad, c);
            println!("{:?} to {} - {:?}", currentpos,c, newpos);
            let steps = shortest_steps_in_numpad(&numpad, currentpos, newpos);
            //println!("{:?}", steps);
            // add all steps to total_steps_robot_1
            total_steps_robot_1.append(&mut steps.clone());
            currentpos = newpos;
        }
        total_steps_robot_1.push('|');
        // find decimals in line
        let re = Regex::new(r"(\d+)").unwrap();
        // only get it from line
        let caps = re.captures(&line).unwrap();
        // get the first match
        let first_match = caps.get(1).unwrap();
        // get the value of the match
        let value = first_match.as_str().parse::<i32>().unwrap();
        println!("Value: {:?}", value);
        multiply_values.push(value);    
        println!("Total steps robot 1: {:?}", total_steps_robot_1);
           
    }
    println!("Total steps robot 1: {:?}", total_steps_robot_1);
    // Ok now we need to loop over an x amount of robots:
    
    println!("Multiply values: {:?}", multiply_values);
    

    // combine total_steps_robot_1 into one string
    let all_steps_string: String = total_steps_robot_1.iter().collect();
    //cut up all_steps_string in substrings on '|'
    let mut split_lines: Vec<&str> = all_steps_string.split('|').collect();
    // enumerate over split_lines
    for l in split_lines {
        println!("Line: {}", l);
    }
    
    for robot_number in 1 .. 3 {
        currentpos = find_pos_in_grid(&arrow_pad, 'A');
        //println!("Robot number: {}", robot_number);
        //println!("Start position: {:?}", currentpos);
        let mut stepspointer = 0;
        let mut tmp_steps: Vec<char> = Vec::new();
        while stepspointer < total_steps_robot_1.len() {
            if total_steps_robot_1[stepspointer] == '|' {
                stepspointer += 1;
                tmp_steps.push('|');
                println!("Robot: {} -> New line", robot_number);
                continue;
            }
            //println!("Robot: {} -> Processing step: {}", robot_number, total_steps_robot_1[stepspointer]);
            let mut newpos = find_pos_in_grid(&arrow_pad, total_steps_robot_1[stepspointer]);
            //println!("move from {:?} to {:?}", currentpos, newpos);
            let steps = shortest_steps_in_arrow_pad(&arrow_pad, currentpos, newpos);
            //println!("{:?}", steps);
            tmp_steps.append(&mut steps.clone());
            currentpos = newpos;
            stepspointer += 1;
        }
        
        total_steps_robot_1 = tmp_steps.clone();

    }
    let all_steps_string: String = total_steps_robot_1.iter().collect();
    println!("Total steps: {:?}", all_steps_string);
    
    // remove last character fro all_steps_string
    let all_steps_string = all_steps_string.trim_end_matches('|');

    // Split totalline in strings on '|'
    let mut split_lines: Vec<&str> = all_steps_string.split('|').collect();

    // enumerate over split_liens
    let mut total_score = 0;
    for (i, line) in split_lines.iter().enumerate() {
        println!("Line {}: {}", i, line);
        let length = line.len();
        let originallinescore = multiply_values[i];
        println!("Original line score: {}", originallinescore);
        println!("Length: {}", length);
        let score = length * multiply_values[i] as usize;
        total_score += score;
        
    }

    println!("Total score: {}", total_score);

}