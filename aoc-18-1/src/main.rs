use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use std::io::{self, Write};
use regex::Regex;


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

// Function that prints a grid from a vector of vectors of chars
fn print_grid(grid: Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn find_character_in_grid(grid: &Vec<Vec<char>>, character: char) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == character {
                return (i, j);
            }
        }
    }
    return (0, 0);
}

fn find_dead_end_character(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '.' {
                let mut closed_paths = 0;
                if grid[i-1][j] == '#' {
                    closed_paths += 1;
                }
                if grid[i+1][j] == '#' {
                    closed_paths += 1;
                }
                if grid[i][j-1] == '#' {
                    closed_paths += 1;
                }
                if grid[i][j+1] == '#' {
                    closed_paths += 1;
                }
                if closed_paths == 3 {
                    return (i, j);
                }

            }
        }
    }
    return (0, 0);
}

fn main() {
    // start timer
    let start_timer = std::time::Instant::now();

    let mut grid: Vec<Vec<(char)>> = Vec::new();
    let all_lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-18-1/src/input1.txt");
    let mut walls: Vec<(usize, usize)> = Vec::new();
    let  mut lastamountleft = 99999;

    let regextouse = Regex::new(r"(\d+)").unwrap();
    for line in all_lines {
        let mut firstdigit: usize = 0;
        let mut seconddigit: usize = 0;
        let mut i = 0;
        for cap in regextouse.captures_iter(&line) {
            if i == 0 {
                firstdigit = cap[0].parse().unwrap();
            } 
            if i == 1 {
                seconddigit = cap[1].parse().unwrap();
            }
            i = i + 1;
        }

        println!("First digit: {}", firstdigit);
        println!("Second digit: {}", seconddigit);
        walls.push((firstdigit, seconddigit));
    }
    //walls.sort();
    println!("Walls: {:?}", walls);
    

    let startpos: (usize, usize) = (0,0);
    let endpos: (usize, usize) = (70,70);


    let mut nodes: Vec<(usize, usize)> = Vec::new();
    let mut edges: Vec<((usize, usize), (usize, usize), usize)> = Vec::new();

    println!("Start position: {:?}", startpos);
    println!("End position: {:?}", endpos);
    
    // Let's start with closing off paths that lead nowhere



    // build nodes... we assume every '.' combined with its direction is a node

    // loop over all characters in the grid
    // loop over all options in a 7 by 7 grid
    for i in 0..71 {
        for j in 0..71 {
            //check if in walls
            let mut in_walls = false;
            for wall in walls.iter() {
                if wall.0 == i && wall.1 == j {
                    in_walls = true;
                }
            }
            if in_walls {
                continue;
            } else {
                nodes.push((i, j));
            }

        }
    }

    


    for node in nodes.iter() {
        println!("{:?}", node);
    }

  
    
    // build edges by checking connecting nodes
    for node in nodes.iter() {
        let mut nodeup: (usize, usize) = (5000, 5000);
        if node.1 > 0 {
            nodeup = (node.0, node.1 - 1);
        }
        
        let nodedown = (node.0, node.1 + 1);
        
        let mut nodeleft: (usize, usize) = (5000, 5000);
        if node.0 > 0 {
            nodeleft = (node.0 - 1, node.1);
        }
        
        let noderight = (node.0 + 1, node.1);
        if nodes.contains(&nodeup) {
            edges.push((*node, nodeup, 1));
        }
        if nodes.contains(&nodedown) {
            edges.push((*node, nodedown, 1));
        }
        if nodes.contains(&nodeleft) {
            edges.push((*node, nodeleft, 1));
        }
        if nodes.contains(&noderight) {
            edges.push((*node, noderight, 1));
        }
    }


    for edge in edges.iter() {
        println!("{:?} -> {:?} = {}", edge.0, edge.1, edge.2);
    }
    println!("Nodes: {}", nodes.len());
    println!("Edges: {}", edges.len());
    //return;

    // So Dijkstra... we need a vector of distances and a vector of visited nodes
    let mut distances: Vec<i32> = vec![i32::MAX; nodes.len()];
    let mut visited: Vec<bool> = vec![false; nodes.len()];
    let mut previous: Vec<usize> = vec![0; nodes.len()];
    let mut current: usize = 0;
    let mut current_distance: i32 = 0;

    println!("Start position: {:?}", startpos);

    // find index of node with start position and direction
    for (i, node) in nodes.iter().enumerate() {
        if node.0 == startpos.0 && node.1 == startpos.1 {
            current = i;
            distances[i] = 0;
            visited[i] = true;
        }
    }
    println!("Start node: {:?} : {}", nodes[current], current);

    // find all edges that start with current node  
    let mut current_edges: Vec<((usize, usize), (usize, usize), usize)> = Vec::new();
    for edge in edges.iter() {
        if edge.0 == nodes[current] {
            current_edges.push(*edge);
        }
    }

    println!("Current edges: {:?}", current_edges);
    // Update distances
    for edge in current_edges.iter() {
        for (i, node) in nodes.iter().enumerate() {
            if node == &edge.1 {
                if distances[i] > distances[current] + edge.2 as i32 {
                    distances[i] = distances[current] + edge.2 as i32;
                    previous[i] = current;
                }
            }
        }
    }

    println!("Distances: {:?}", distances);

    // find amount of visited that are false
    let mut unvisited: i32 = 0;
    for v in visited.iter() {
        if !v {
            unvisited += 1;
        }
    }

    while unvisited > 0 {
        // find the node with the smallest distance
        let mut smallest_distance: i32 = i32::MAX;
        let mut smallest_index: usize = 0;
        for (i, d) in distances.iter().enumerate() {
            if !visited[i] && *d < smallest_distance {
                smallest_distance = *d;
                smallest_index = i;
            }
        }
        current = smallest_index;
        visited[current] = true;
        //println!("Current node: {:?} : {}", nodes[current], current);

        // find all edges that start with current node and do not end at previous
        let mut current_edges: Vec<((usize, usize), (usize, usize), usize)> = Vec::new();
        for edge in edges.iter() {
            if edge.0 == nodes[current] && edge.1 != nodes[previous[current]] {
                current_edges.push(*edge);
            }
        }
        //println!("Current edges: {:?}", current_edges);

        // Update distances
        for edge in current_edges.iter() {
            for (i, node) in nodes.iter().enumerate() {
                if node == &edge.1 {
                    if distances[i] > distances[current] + edge.2 as i32 {
                        distances[i] = distances[current] + edge.2 as i32;
                        previous[i] = current;
                    }
                }
            }
        }

        //println!("Distances: {:?}", distances);

        // find amount of visited that are false
        unvisited = 0;
        for v in visited.iter() {
            if !v {
                unvisited += 1;
            }
        }
       println!("Unvisited: {}", unvisited);
        if unvisited == lastamountleft {
            break;
        }
        lastamountleft = unvisited;
        
    }

    println!("nodes: {:?}", nodes);
    println!("distances: {:?}", distances);
    // find nodes that start with end_position
    let mut end_nodes: Vec<usize> = Vec::new();
    for (i, node) in nodes.iter().enumerate() {
        if node.0 == endpos.0 && node.1 == endpos.1 {
            println!("End node: {:?} : {}", node, distances[i]);
        }
    }


}