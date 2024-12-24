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


fn main() {
    let lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-24-1/src/input.txt");
    let mut firstpart: bool = true;

    let mut nodes:Vec<(String, String, String)> = Vec::new();
    let mut edges:Vec<(String, String)> = Vec::new();
    let mut notdone:Vec<(String, String, String)> = Vec::new();



    for line in &lines {
        
        if line == "" {
            firstpart = false;
            continue;
        }

        if firstpart {
            //split line on ": "
            let parts: Vec<&str> = line.split(": ").collect();
            nodes.push((parts[0].to_string(), "None".to_string(), parts[1].to_string()));
            

        } else {
            // x00 AND y00 -> z00
            let parts: Vec<&str> = line.split(" "). collect();
            nodes.push((parts[4].to_string(), parts[1].to_string(), " ".to_string()));
            edges.push((parts[0].to_string(), parts[4].to_string()));
            edges.push((parts[2].to_string(), parts[4].to_string()));
            notdone.push((parts[4].to_string(), parts[1].to_string(), " ".to_string()));
        }
        
    }

    for node in nodes.clone() {
        //println!("Node {:?}", node)
    }

    for edge in edges.clone() {
        //println!("Edge {:?}", edge)
    }

    for notdone in notdone.clone() {
        //println!("Notdone {:?}", notdone)
    }



    while notdone.clone().len() > 0 {
        let mut new_notdone: Vec<(String, String, String)> = Vec::new();
        for node in notdone.clone() {
            
            //find connecting nodes in edges
            let mut found: bool = false;
            let mut connecting_node1: (String, String, String) = ("None".to_string(), "None".to_string(), "None".to_string());
            let mut connecting_node2: (String, String, String) = ("None".to_string(), "None".to_string(), "None".to_string());
            for edge in edges.clone() {
                if edge.1 == node.0 {
                    if found {
                        //get node for edge.0
                        for node in nodes.clone() {
                            if node.0 == edge.0 {
                                connecting_node2 = node;
                            }
                        }
                    } else {
                        //get node for edge.1
                        for node in nodes.clone() {
                            if node.0 == edge.0 {
                                connecting_node1 = node;
                            }
                        }
                        found = true;
                    }
                }
            }
            // if node.0 starts with "Z" print node.2
            if node.0.starts_with("z") {
                println!("Connecting nodes: {:?} -> {:?} {:?}", node, connecting_node1, connecting_node2);
            }
            

            if connecting_node1.2 == " " || connecting_node2.2 == " " {
                // We are not ready for this one yet, we need both values
                // push this node back on new_notdone
                new_notdone.push(node);
                continue;
            }
        
            // We have both values, calculate new value
            let mut newvalue:String = " ".to_string();
            if node.1 == "AND" {
                if connecting_node1.2 == "0" || connecting_node2.2 == "0" {
                    newvalue = "0".to_string();
                } else if connecting_node1.2 == "1" && connecting_node2.2 == "1" {
                    newvalue = "1".to_string();
                } else {
                    println!("Error: unknown value {:?}", node.1);
                    return;
                }

            } else if node.1 == "OR" {
                if connecting_node1.2 == "1" || connecting_node2.2 == "1" {
                    newvalue = "1".to_string();
                } else if connecting_node1.2 == "0" && connecting_node2.2 == "0" {
                    newvalue = "0".to_string();
                } else {
                    println!("Error: unknown value {:?}", node.1);
                    return;
                }


            } else if node.1 == "XOR" {
                if (connecting_node1.2 == "1" && connecting_node2.2 == "0") || (connecting_node1.2 == "0" && connecting_node2.2 == "1") {
                    newvalue = "1".to_string();
                } else if (connecting_node1.2 == "0" && connecting_node2.2 == "0") || (connecting_node1.2 == "1" && connecting_node2.2 == "1") {
                    newvalue = "0".to_string();
                } else {
                    println!("Error: unknown value {:?}", node.1);
                    return;
                }

            } else {
                println!("Error: unknown operator {:?}", node.1);
                return;
            }

            println!("New value: {:?}", newvalue);
            // Update node with new value
            let mut newnodes: Vec<(String, String, String)> = Vec::new();
            for n in nodes.clone() {
                if n.0 == node.0 {
                    newnodes.push((n.0.clone(), n.1.clone(), newvalue.clone()));
                } else {
                    newnodes.push(n);
                }
            }
            nodes = newnodes.clone();
            for node in nodes.clone() {
                //println!("Node {:?}", node)
            }
            
            
        }
        notdone = new_notdone.clone();
    }


    // order all nodes on node.0
    nodes.sort_by(|a, b| a.0.cmp(&b.0));

    //reverse order in nodes
    nodes.reverse();
    for node in nodes.clone() {
        //println!("Node rev {:?}", node)
    }

    let mut binarynr: String = "".to_string();

    for node in nodes.clone() {
        // if node.0 starts with "Z" print node.2
        if node.0.starts_with("z") {
            //println!("{:?}", node.2);
            // contact node.2 to binarynr
            binarynr = binarynr + &node.2;

        }
    }

    println!("Binary number: {:?}", binarynr);
    // convert binarynr to decimal
    let decimal = i128::from_str_radix(&binarynr, 2).unwrap();
    println!("Decimal number: {:?}", decimal);
    
}
   