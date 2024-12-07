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

fn generate_combinations(n: usize, current: &mut Vec<u8>, result: &mut Vec<Vec<u8>>) {
    if current.len() == n {
        result.push(current.clone());
        return;
    }

    for &value in &[0, 1, 2] {
        current.push(value);
        generate_combinations(n, current, result);
        current.pop();
    }
}

fn main() {

    let startime = std::time::Instant::now();

    let all_lines: Vec<String> = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-7-1/src/input1.txt");

    let mut number: i16 = 2;
    // convert number into a string of the complete binary number  
    let mut binary_number: String = format!("{:016b}", number);
    //println!("Binary number: {}", binary_number);

    let mut total: i64 = 0;



    
    //for combination in result {
    //    println!("{:?}", combination);
    //}


    //iterate over all_lines   
    for line in all_lines.iter() {
        // take the number in line before the : sign
        let target: i64 = line.split(":").next().unwrap().parse().unwrap();
        // parse the space seperated string after : into a vecor of i32
        let numbers: Vec<i64> = line.split(":").nth(1).unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        // calculate the length of numbers by the power of 2
        let combinations: i32 = 3_i32.pow(numbers.len() as u32 - 1);
        let amount_of_numbers: i32 = numbers.len() as i32;
        
        //println!("Target: {}", target);
        //println!("Numbers: {:?}", numbers);
        //println!("combinations: {}", combinations);
        //println!("Amount of numbers: {}", amount_of_numbers);


        let n = amount_of_numbers - 1; // Change this to your desired length
        let mut result = Vec::new();
        let mut current = Vec::new();
        generate_combinations(n as usize, &mut current, &mut result);

        for combination in result {

            let mut sum: i64 = numbers[0];
            //println!("Combination: {:?}", combination);

            for j in 1..amount_of_numbers {
                if combination[j as usize - 1] == 0 {
                    //println!("Adding: {}", numbers[j as usize]);
                    sum = sum + numbers[j as usize];
                } else if combination[j as usize - 1] == 1 {
                    //println!("Multiplying: {}", numbers[j as usize]);
                    sum = sum * numbers[j as usize];
                } else {
                    //println!("concatonating: {}", numbers[j as usize]);
                    //concatenate sum with the number
                    let mut sum_str = sum.to_string();
                    let number_str = numbers[j as usize].to_string();
                    sum_str.push_str(&number_str);
                    sum = sum_str.parse().unwrap();
                }
            }

            if sum == target {
                //println!("Found the target: {}", target);
                total = total + target;
                break;
            }
        }
    }
    println!("Total: {}", total);
    let endtime = startime.elapsed();
    println!("Elapsed time: {:?}", endtime);
}