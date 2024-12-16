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

    // inflate grid
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for y in 0..grid.len() {
        let mut new_row: Vec<char> = Vec::new();
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                new_row.push('#');
                new_row.push('#');
            } else if grid[y][x] == 'O' {
                new_row.push('[');
                new_row.push(']');
            } else if grid[y][x] == '.' {
                new_row.push('.');
                new_row.push('.');
            } else if grid[y][x] == '@' {
                new_row.push('@');
                new_row.push('.');
            }
        }
        new_grid.push(new_row);
    }
    grid = new_grid;


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
        println!("Press Enter to continue...");
        let mut input = String::new();
        //io::stdin().read_line(&mut input).unwrap();

        // calculate new position
        println!("### NEW CYCLE: robot {:?}", robot);
        let mut tmpgrid = grid.clone();
        println!("Starting grid");
        tmpgrid[robot.1 as usize][robot.0 as usize] = '@';
        for line in &tmpgrid {
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
        } else if (new_char == '[' || new_char == ']') && (i == '<' || i == '>') {
            // if o test pushability and move robots + o's
 
            println!("horizontal push");
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
                } else if next_char == '[' || next_char == ']' {
                    boxes_to_move.push(next_pos);
                }
                move_pos = next_pos;
            }
            // if wall, do nothing
            if next_char == '#' {
                println!("WALL WALL WALL WALL WALL");
            } else {
                println!("MOVE MOVE MOVE MOVE MOVE");

                // loop backwards over boxes_to_move and move them



                for b in boxes_to_move.iter().rev() {
                    //println!("box to move {:?}", b);
                    grid[move_pos.1 as usize][move_pos.0 as usize] = grid[b.1 as usize][b.0 as usize];
                    move_pos = *b;
                }
                grid[move_pos.1 as usize][move_pos.0 as usize] = '.';
                robot = new_pos;
                    

            }

        }  else if (new_char == '[' || new_char == ']') && (i == '^' || i == 'v') {
            // So this is more difficult, but let's go for it
            let mut boxes_to_move: Vec<(i32, i32)> = Vec::new();
            let mut boxes_to_test: Vec<(i32, i32)> = Vec::new();
            boxes_to_test.push(new_pos);
            if new_char == '[' {
                // pusing second part of box
                boxes_to_test.push((new_pos.0 + 1, new_pos.1));
            } else {
                // pushing second part of box
                boxes_to_test.push((new_pos.0 - 1, new_pos.1));
            }
            println!("VERTICAL MOVE! boxes_to_test {:?}", boxes_to_test);
            // add all boxes_to_test to boxes_to_move
            for b in boxes_to_test.clone() {
                boxes_to_move.push(b);
            }

            let mut searching = true;
            let mut wall         = false;
            let mut empty_spaces = true;

            while searching {
                println!("boxes_to_test {:?}", boxes_to_test);
                empty_spaces = true;
                let mut next_boxes_to_test: Vec<(i32, i32)> = Vec::new();
                for b in boxes_to_test {
                    println!("box to test {:?}", b);
                    let mut next_pos: (i32, i32) = (0, 0);
                    if i == '^' {
                        next_pos = (b.0, b.1 - 1);
                    } else if i == 'v' {
                        next_pos = (b.0, b.1 + 1);
                    }
                    let cur_char = grid[b.1 as usize][b.0 as usize];
                    let next_char = grid[next_pos.1 as usize][next_pos.0 as usize];
                    // if wall stop searching and set wall to true and empty_spaces to false
                    if next_char == '#' {
                        println!("wall");
                        wall = true;
                        empty_spaces = false;
                        searching = false;
                        break;
                    // if empty space, we continue
                    } else if next_char == '.' {
                        println!("empty space");
                        continue;
                    // if box, we add it to next_boxes_to_test
                        // add other part of box to next_boxes_to_test
                        // set empty_spaces to false
                    } else if next_char == '[' || next_char == ']' {
                        println!("box: {}", next_char);
                        empty_spaces = false;
                        next_boxes_to_test.push(next_pos);
                        if next_char == '[' && (next_char != cur_char) {
                            next_boxes_to_test.push((next_pos.0 + 1, next_pos.1));
                        } else if next_char == ']' && (next_char != cur_char) {
                            next_boxes_to_test.push((next_pos.0 - 1, next_pos.1));
                        }
                    }
                }

                println!("next_boxes_to_test {:?}", next_boxes_to_test);
                // move all next_boxes_to_test to boxes_to_move
                for b in next_boxes_to_test.clone() {
                    boxes_to_move.push(b);
                }
                // move next_boxes_to_test to boxes_to_test
                boxes_to_test = next_boxes_to_test;
                // if wall, stop searching
                if wall {
                    searching = false;
                }
                // if empty_spaces stop searching
                if empty_spaces {
                    searching = false;
                }
                println!("boxes to move so far {:?}", boxes_to_move);
                
            }
            
            // if wall, do nothing
            if wall {
                println!("WALL WALL WALL WALL WALL");
            } else {
                println!("MOVE MOVE MOVE MOVE MOVE");
                // loop backwards over boxes_to_move
                let mut moved_boxes: Vec<(i32, i32)> = Vec::new();
                for b in boxes_to_move.iter().rev() {
                    //println!("box to move {:?}", b);
                    //Check if box was already moved
                    if moved_boxes.contains(b) {
                        //println!("box already moved {} {}", b.0, b.1);
                        continue;
                    }
                    let next_pos: (i32, i32) = match i {
                        '^' => (b.0, b.1 - 1),
                        'v' => (b.0, b.1 + 1),
                        _ => *b,
                    };
                    grid[next_pos.1 as usize][next_pos.0 as usize] = grid[b.1 as usize][b.0 as usize];
                    grid[b.1 as usize][b.0 as usize] = '.';
                    moved_boxes.push(*b);
                }
                robot = new_pos;
            }
            






        }

        let mut tmpgrid = grid.clone();
        println!("END grid");
        tmpgrid[robot.1 as usize][robot.0 as usize] = '@';
        for line in &tmpgrid {
            for c in line {
                print!("{}", c);
            }
            println!("");
        }

    }

    
    // End of instructions
    println!("End of instructions");
    for line in &grid {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }

    // find positions of all O in grid
    let mut o_positions: Vec<(i32, i32)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '[' {
                o_positions.push((x as i32, y as i32));
            }
        }
    }
    println!("o_positions {:?}", o_positions);

    let mut score: i64 = 0;

    for o in o_positions {
        score += (o.0 + (o.1 * 100)) as i64; 
        //println!("score: {}", score);
    }

    println!("score: {}", score);
}
