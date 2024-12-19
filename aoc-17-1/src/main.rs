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
    // start timer
    let start = std::time::Instant::now();

    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-17-1/src/test.txt");

    let mut instruction_pointer: i32 = 0;

    let mut instructions: Vec<i8> = Vec::new();
    let re_registers = Regex::new(r"(\d+)").unwrap();

    let mut register_a: i32 = re_registers.captures(&all_lines[0]).unwrap()[1].parse::<i32>().unwrap();
    let mut register_b: i32 = re_registers.captures(&all_lines[1]).unwrap()[1].parse::<i32>().unwrap();
    let mut register_c: i32 = re_registers.captures(&all_lines[2]).unwrap()[1].parse::<i32>().unwrap();

    instructions = re_registers.captures_iter(&all_lines[4]).map(|x| x[1].parse::<i8>().unwrap()).collect();

    println!("Register a: {}", register_a);
    println!("Register b: {}", register_b);
    println!("Register c: {}", register_c);
    println!("Instructions: {:?}", instructions);

    let mut results: Vec<i32> = Vec::new();

    while instruction_pointer < instructions.len() as i32
    {
        let opcode =  instructions[instruction_pointer as usize];
        let operand = instructions[instruction_pointer as usize + 1];
        let mut combo_operator: i32 = 0;

        // lets calculate combo oprator so we only have to define that logic once.
        if operand >= 0 && operand <= 3 {
            combo_operator = operand as i32
        } else if operand == 4 {
            combo_operator = register_a;
        } else if operand == 5 {
            combo_operator = register_b;
        } else if operand == 6 {
            combo_operator = register_c;
        } else {
            
        }

        println!("instructionpointer: {}. opcode: {}, operand: {}, combooperator: {}", instruction_pointer, opcode, operand, combo_operator);
        println!("register_a: {}", register_a);
        println!("register_b: {}", register_b);
        println!("register_c: {}", register_c);

        if (opcode == 0) {
            let base: i32 = 2;
            let newvalue: i32 = register_a / base.pow(combo_operator as u32);
            register_a = newvalue;
            println!("Register a changed into: {}", register_a);
        } else if (opcode == 1) {
            // bitwise XOR of register b and operand

            // bitwise XOR of register b and operand
            println!("OPCODE 1 PROCESSING:");
            println!("opcode {}", opcode);
            println!("operand {}", operand);
            println!("Register b startss at: {}", register_b);
            let newvalue: i32 = register_b ^ operand as i32;
            println!("newvalue: {}", newvalue);
            register_b = newvalue;
            println!("Register b changed into: {}", register_b);
        } else if (opcode == 2) {
            let newvalue: i32 = combo_operator % 8;
            register_b = newvalue;
            println!("Register b changed into: {}", register_b);

        } else if (opcode == 3) {
            if register_a == 0 {
                instruction_pointer += 2;
                continue;
            }
            instruction_pointer = operand as i32;
            println!("Jumping to instruction_pointer: {}", instruction_pointer);
            continue;

        } else if (opcode == 4) {
            let newvalue: i32 = register_b ^ register_c;
            register_b = newvalue;
            println!("Register b changed into: {}", register_b);

        } else if (opcode == 5) {
            let newvalue: i32 = combo_operator % 8;
            println!("{},", newvalue);
            results.push(newvalue);

        } else if (opcode == 6) {
            let base: i32 = 2;
            let newvalue: i32 = register_a / base.pow(combo_operator as u32);
            register_b = newvalue;
            println!("Register b changed into: {}", register_b);

        } else if (opcode == 7) {
            let base: i32 = 2;
            let newvalue: i32 = register_a / base.pow(combo_operator as u32);
            register_c = newvalue;
            println!("Register c changed into: {}", register_c);

        } 

        instruction_pointer += 2;
        
    }
    println!("Register a: {}", register_a);
    println!("Register b: {}", register_b);
    println!("Register c: {}", register_c);

    println!("Results: {:?}", results);

    let mut same:bool = true;
    for i in 0..results.len() {
        if results[i] != instructions[i] as i32 {

            same = false;
            break;            

        }
    }

    if same {
        println!("Results match instructions");
    } 
}




