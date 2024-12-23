use std::{
    fs::File, i32, io::{prelude::*, BufReader}, path::Path
};
use std::io::{self, Write};
use std::collections::{HashSet, VecDeque};
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use std::collections::HashMap;

fn bron_kerbosch(
    graph: &UnGraph<(), ()>,
    r: &mut HashSet<NodeIndex>,
    p: &mut HashSet<NodeIndex>,
    x: &mut HashSet<NodeIndex>,
    cliques: &mut Vec<HashSet<NodeIndex>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    } else {
        let mut p_clone = p.clone();
        for v in p_clone.iter() {
            let mut r_new = r.clone();
            r_new.insert(*v);

            let neighbors: HashSet<_> = graph.neighbors(*v).collect();
            let mut p_new: HashSet<_> = p.intersection(&neighbors).cloned().collect();
            let mut x_new: HashSet<_> = x.intersection(&neighbors).cloned().collect();

            bron_kerbosch(graph, &mut r_new, &mut p_new, &mut x_new, cliques);

            p.remove(v);
            x.insert(*v);
        }
    }
}

fn find_cliques(graph: &UnGraph<(), ()>) -> Vec<HashSet<NodeIndex>> {
    let mut cliques = Vec::new();
    let mut r = HashSet::new();
    let mut p: HashSet<_> = graph.node_indices().collect();
    let mut x = HashSet::new();

    bron_kerbosch(graph, &mut r, &mut p, &mut x, &mut cliques);
    cliques
}


// Function that reads file and returns a vector of strings
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let lines = lines_from_file("/Users/ES94CO/Developer/rust/Advent/aoc-23-1/src/input.txt");
    let mut nodes: Vec<String> = Vec::new();
    let mut edges: Vec<(String, String)> = Vec::new();

    for line in lines {
        let split_line: Vec<String> = line.split("-").map(|s| s.to_string()).collect();
        println!("{:?}", split_line);
        nodes.push(split_line[0].clone());
        nodes.push(split_line[1].clone());
        edges.push((split_line[0].clone(), split_line[1].clone()));
        edges.push((split_line[1].clone(), split_line[0].clone()));
    } 
   
    nodes.sort();
    nodes.dedup();
    edges.sort();
    edges.dedup();
    
    let mut graph = UnGraph::<(), ()>::default();

    let mut node_indices = HashMap::new();

    for node in &nodes {
        let index = graph.add_node(());
        node_indices.insert(node, index);
    }

    for edge in &edges {
        let a = node_indices.get(&edge.0).unwrap();
        let b = node_indices.get(&edge.1).unwrap();
        graph.add_edge(*a, *b, ());
    }

    println!("{:?}", node_indices);
    let cliques = find_cliques(&graph);

    // sort the cliques by size
    let mut sorted_cliques = cliques.clone();
    sorted_cliques.sort_by_key(|c| c.len());
    
    println!("{:?}", sorted_cliques);

    for clique in sorted_cliques {
        let mut lan_party:Vec<String> = Vec::new();
        for node in clique {
            lan_party.push((*node_indices.iter().find(|(_, &v)| v == node).unwrap().0).to_string())
        }
        // print lan_party as comma separated string
        println!("{:?}", lan_party.join(","));
    }






    





}