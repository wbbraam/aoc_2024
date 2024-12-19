use std::{
    fs::File, i64, io::{prelude::*, BufReader}, path::Path
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

    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-17-1/src/input.txt");

    let mut instruction_pointer: i64 = 0;

    let mut instructions: Vec<i8> = Vec::new();
    let re_registers = Regex::new(r"(\d+)").unwrap();

    let mut stored_register_a: i64 = re_registers.captures(&all_lines[0]).unwrap()[1].parse::<i64>().unwrap();
    let mut stored_register_b: i64 = re_registers.captures(&all_lines[1]).unwrap()[1].parse::<i64>().unwrap();
    let mut stored_register_c: i64 = re_registers.captures(&all_lines[2]).unwrap()[1].parse::<i64>().unwrap();

    instructions = re_registers.captures_iter(&all_lines[4]).map(|x| x[1].parse::<i8>().unwrap()).collect();

    println!("Register a: {}", stored_register_a);
    println!("Register b: {}", stored_register_b);
    println!("Register c: {}", stored_register_c);
    println!("Instructions: {:?}", instructions);


    let mut register_a = stored_register_a;
    let mut register_b = stored_register_b;
    let mut register_c = stored_register_c;

    let mut results: Vec<i64> = Vec::new();
    let mut i: i64 = 1;
    while true{
        register_a = i;

        if i > 10000000000000000 { break;}
        register_b = stored_register_b;
        register_c = stored_register_c;
        instruction_pointer = 0;


        
        results = Vec::new();
        while instruction_pointer < instructions.len() as i64
        {
        
            let opcode =  instructions[instruction_pointer as usize];
            let operand = instructions[instruction_pointer as usize + 1];
            let mut combo_operator: i64 = 0;
            
            

            // lets calculate combo oprator so we only have to define that logic once.
            if operand >= 0 && operand <= 3 {
                combo_operator = operand as i64
            } else if operand == 4 {
                combo_operator = register_a;
            } else if operand == 5 {
                combo_operator = register_b;
            } else if operand == 6 {
                combo_operator = register_c;
            } else {
                
            }

            //println!("instructionpointer: {}. opcode: {}, operand: {}, combooperator: {}", instruction_pointer, opcode, operand, combo_operator);
            //println!("register_a: {}", register_a);
            //println!("register_b: {}", register_b);
            //println!("register_c: {}", register_c);

            if (opcode == 0) {
                let base: i64 = 2;
                let newvalue: i64 = register_a / base.pow(combo_operator as u32);
                register_a = newvalue;
                //println!("Register a changed into: {}", register_a);
            } else if (opcode == 1) {
                // bitwise XOR of register b and operand

                // bitwise XOR of register b and operand
                //println!("OPCODE 1 PROCESSING:");
                //println!("opcode {}", opcode);
                //println!("operand {}", operand);
                //println!("Register b startss at: {}", register_b);
                let newvalue: i64 = register_b ^ operand as i64;
                //println!("newvalue: {}", newvalue);
                register_b = newvalue;
                //println!("Register b changed into: {}", register_b);
            } else if (opcode == 2) {
                let newvalue: i64 = combo_operator % 8;
                register_b = newvalue;
                //println!("Register b changed into: {}", register_b);

            } else if (opcode == 3) {
                if register_a == 0 {
                    instruction_pointer += 2;
                    continue;
                }
                instruction_pointer = operand as i64;
                //println!("Jumping to instruction_pointer: {}", instruction_pointer);
                continue;

            } else if (opcode == 4) {
                let newvalue: i64 = register_b ^ register_c;
                register_b = newvalue;
                //println!("Register b changed into: {}", register_b);

            } else if (opcode == 5) {
                let newvalue: i64 = combo_operator % 8;
                //println!("{},", newvalue);
                results.push(newvalue);

            } else if (opcode == 6) {
                let base: i64 = 2;
                let newvalue: i64 = register_a / base.pow(combo_operator as u32);
                register_b = newvalue;
                //println!("Register b changed into: {}", register_b);

            } else if (opcode == 7) {
                let base: i64 = 2;
                let newvalue: i64 = register_a / base.pow(combo_operator as u32);
                register_c = newvalue;
                //println!("Register c changed into: {}", register_c);

            } 

            instruction_pointer += 2;
            
        }


        // check if results == instructions
        //println!("Results: {:?}", results);
        //println!("Instructions: {:?}", instructions);
        let mut same:bool = true;

        

        // loop on index backwards over results
        let length_results = results.len();
        let length_instructions = instructions.len();

        for pos in 0..length_results {
            //println!("pos: {}", pos);
            //println!("results pointer: {}", length_results - pos);
            //println!("instructions pointer: {}", length_instructions - pos);
            if results[length_results - pos - 1] != instructions[length_instructions - pos - 1] as i64 {
                same = false;
                break;
            }
        }


        //return;
        println!("results: {:?}", results);
        
    
        if same {
            //println!("Results match instructions");
            //println!("Register a: {}", i);
            //println!("Iteration: {} -> {:?} -> {} -> {:?}", i, results, results.len(), instructions);
            if results.len() == instructions.len() {
                println!(" RESULT Register a: {}", i);
                break;
            }
            i = i * 8;
        } else {
            i = i+1;
        }

        
        
    }
    // end timer and print
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    //println!("Register a: {}", register_a);
    //println!("Register b: {}", register_b);
    //println!("Register c: {}", register_c);

    //println!("Results: {:?}", results);
}




