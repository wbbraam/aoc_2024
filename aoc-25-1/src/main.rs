use core::net;
use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use std::io::{self, Write};



// Function that reads file and returns a vector of strings
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_code(l1:&String, l2:&String, l3:&String, l4:&String, l5:&String, l6:&String) -> String {
    let mut code:usize = 0;
    let mut linepointer = 0;

    let mut code_string:String = "".to_string();

    while linepointer < l1.len() {
        let line1 = l1.chars().nth(linepointer).unwrap();
        let line2 = l2.chars().nth(linepointer).unwrap();
        let line3 = l3.chars().nth(linepointer).unwrap();
        let line4 = l4.chars().nth(linepointer).unwrap();
        let line5 = l5.chars().nth(linepointer).unwrap();
        let line6 = l6.chars().nth(linepointer).unwrap();

        let mut value_of_pin = 0;
        if line1 == '#' {
            value_of_pin += 1;
        }
        if line2 == '#' {
            value_of_pin += 1;
        }
        if line3 == '#' {
            value_of_pin += 1;
        }
        if line4 == '#' {
            value_of_pin += 1;
        }
        if line5 == '#' {
            value_of_pin += 1;
        }
        if line6 == '#' {
            value_of_pin += 1;
        }
        code_string.push_str(value_of_pin.to_string().as_str());
        linepointer += 1;
    }

    //println!("Code: {:?}", code_string);
    return code_string;
}

fn fit(key:&String, lock:&String) -> bool {
    let mut keypointer = 0;
    let mut lockpointer = 0;
    let mut fit:bool = true;

    while keypointer < key.len() {
        let keychar = key.chars().nth(keypointer).unwrap();
        let lockchar = lock.chars().nth(lockpointer).unwrap();

        let keychar_int = keychar.to_digit(10).unwrap() as i32;
        let lockchar_int = lockchar.to_digit(10).unwrap() as i32;

        if keychar_int + lockchar_int > 5 {
            fit = false;
            break;
        }

        keypointer += 1;
        lockpointer += 1;
    }

    return fit;
}

fn main() {
    let lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-25-1/src/input.txt");
    let mut keys:Vec<String> = Vec::new();
    let mut locks:Vec<String> = Vec::new();

    let mut linepointer = 0;

    while linepointer < lines.len() {
        let line1:String = lines[linepointer].to_string();
        let line2:String = lines[linepointer+1].to_string();
        let line3:String = lines[linepointer+2].to_string();
        let line4:String = lines[linepointer+3].to_string();
        let line5:String = lines[linepointer+4].to_string();
        let line6:String = lines[linepointer+5].to_string();
        let line7:String = lines[linepointer+6].to_string();

        if line1 == "#####".to_string() {
            println!("Lock");
            let lockcode = get_code(&line2, &line3, &line4, &line5, &line6, &line7);
            locks.push(lockcode.clone());
            println!("Lockcode: {:?}", lockcode);
        } else if line7 == "#####".to_string() {
            println!("Key");
            let keycode = get_code(&line1, &line2, &line3, &line4, &line5, &line6);
            keys.push(keycode.clone());
            println!("Keycode: {:?}", keycode);
        } else {
            println!("Error");
        }


        

        linepointer += 8;
    }

    // loop over all keys and locks and print both codes together
    let mut keylockcombo:usize = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if fit(&key, &lock) {
                println!("FIT! Key: {:?} Lock: {:?}", key, lock);
                keylockcombo += 1;
            } else {
                println!("NO FIT! Key: {:?} Lock: {:?}", key, lock);
            }
        }
    }

    println!("Keylockcombo: {:?}", keylockcombo);
}