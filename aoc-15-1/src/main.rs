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

// Function that reads a file and returns a vector of vectors of chars
fn charcters_from_file(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn main() {
    // start timer
    let start = std::time::Instant::now();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<char> = Vec::new();
    let mut robot: (i32, i32) = (0, 0);

    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-15-1/src/input1.txt");

    // loop over all lines
    let mut firstpart: bool = true;
    for line in all_lines {
        // if line is empty, switch to second part
        if line.is_empty() {
            firstpart = false;
            continue;
        }
        if firstpart {
            // split line into characters and push to grid
            let chars: Vec<char> = line.chars().collect();
            grid.push(chars);
        } else {
            // split line into characters and push to instructions
            let chars: Vec<char> = line.chars().collect();
            // put each instruction in the instructions vector
            for c in chars {
                instructions.push(c);
            }
        }
    }

    // find @ in grid and store position in robot
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '@' {
                robot = (x as i32, y as i32);
            }
        }
    }
    grid[robot.1 as usize][robot.0 as usize] = '.';
    println!("grid {:?}", grid);
    println!("instructions{:?}", instructions);
    println!("robot {:?}", robot);

    // loop over instructions
    for i in instructions {
        // calculate new position
        println!("### NEW CYCLE: robot {:?}", robot);
        for line in &grid {
            for c in line {
                print!("{}", c);
            }
            println!("");
        }

        let new_pos: (i32, i32) = match i {
            '^' => (robot.0, robot.1 - 1),
            'v' => (robot.0, robot.1 + 1),
            '<' => (robot.0 - 1, robot.1),
            '>' => (robot.0 + 1, robot.1),
            _ => robot,
        };
        println!("after move {}, new_pos {:?}", i, new_pos);
        // get character at new position
        let new_char = grid[new_pos.1 as usize][new_pos.0 as usize];
        println!("new_char {:?}", new_char);
        if new_char == '#' {
            // if wall, do nothing
            println!("wall");
        } else if new_char == '.' {
            // if . move robot
            robot = new_pos;
            println!("moved");
        } else if new_char == 'O' {
            // if o test pushability and move robots + o's
            println!("push");
            let mut boxes_to_move: Vec<(i32, i32)> = Vec::new();
            boxes_to_move.push(new_pos);
            let mut searching = true;
            let mut move_pos  = new_pos;
            let mut next_char = ' ';
            // Find all positions in the push direction until wall or empty space
            while searching {
                
                let next_pos: (i32, i32) = match i {
                    '^' => (move_pos.0, move_pos.1 - 1),
                    'v' => (move_pos.0, move_pos.1 + 1),
                    '<' => (move_pos.0 - 1, move_pos.1),
                    '>' => (move_pos.0 + 1, move_pos.1),
                    _ => move_pos,
                };
                next_char = grid[next_pos.1 as usize][next_pos.0 as usize];
                if next_char == '#' {
                    searching = false;
                } else if next_char == '.' {
                    searching = false;
                } else if next_char == 'O' {
                    boxes_to_move.push(next_pos);
                }
                move_pos = next_pos;
            }
            // if wall, do nothing
            if next_char == '#' {
                println!("wall");
            } else {
                println!("We can move boxes {:?}", boxes_to_move);

                // loop backwards over boxes_to_move and move them



                for b in boxes_to_move.iter().rev() {
                    println!("box to move {:?}", b);
                    grid[b.1 as usize][b.0 as usize] = '.';
                    grid[move_pos.1 as usize][move_pos.0 as usize] = 'O';
                    move_pos = *b;
                }
                robot = new_pos;
                    

            }

        }
        // if wall, do nothing
        // if . move robot
        // if o test pushability and move robots + o's
    }
    // End of instructions
    println!("End of instructions");

    // find positions of all O in grid
    let mut o_positions: Vec<(i32, i32)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'O' {
                o_positions.push((x as i32, y as i32));
            }
        }
    }
    println!("o_positions {:?}", o_positions);

    let mut score: i64 = 0;

    for o in o_positions {
        score += (o.0 + (o.1 * 100)) as i64; 
    }

    println!("score: {}", score);
}
