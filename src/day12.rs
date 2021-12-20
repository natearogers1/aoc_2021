use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn run() {
    let inputs = read_file("inputs/day12.txt");

    let mut finished_paths: Vec<Vec<String>> = vec![];
    let connections: Vec<Vec<&str>> = inputs.iter().map(|x| x.split("-").collect()).collect();

    let mut paths: Vec<Vec<String>> = vec![vec![String::from("start")]];

    while paths.len() > 0 {
        let path = paths.pop().unwrap();
        let mut next_hops = get_next_hops(&path, &connections);
        //println!("{:?} : {:?}", path, next_hops);
        while next_hops.len() > 0 {
            let next_hop = next_hops.pop().unwrap();
            if next_hop == "end" {
                let mut final_path = path.to_vec();
                final_path.push(String::from("end"));
                finished_paths.push(final_path);
            } else {
                let mut intermediate_path = path.to_vec();
                intermediate_path.push(next_hop);
                paths.push(intermediate_path)
            }
        }
    }
    //println!("{:?}", finished_paths);
    println!("{:?}", finished_paths.len());
}

fn get_next_hops(path: &Vec<String>, connections: &Vec<Vec<&str>>) -> Vec<String> {
    let mut next_hops: Vec<String> = vec![];

    let last_cave = &path[path.len() - 1];
    //println!("{}", last_cave);
    for c in connections {
        let mut potential = String::from("");

        if c[0] == last_cave.trim_end_matches("x2") {
            potential = c[1].to_string();
        } else if c[1] == last_cave.trim_end_matches("x2") {
            potential = c[0].to_string();
        }
        if potential != "" {
            if potential == "start" {
                continue;
            } else if potential == "end" {
                next_hops.push(potential)
            } else if potential.chars().all(|c| c.is_ascii_lowercase()) {
                // if the next cave is small, check if we have been to this cave before
                if path.iter().any(|x| x == &potential) {
                    // if we have, check for any other small cave doubles
                    if !path.iter().any(|x| x.contains("x2")) {
                        // only add the small cave if we haven't visited a small cave twice before.
                        potential = potential + "x2";
                        next_hops.push(potential);
                    }
                    // if we have not been here before, add the small cave unchanged
                } else if !path.iter().any(|x| x == &potential) {
                    next_hops.push(potential)
                }
            } else if potential.chars().all(|c| !c.is_ascii_lowercase()) {
                // large cave
                next_hops.push(potential)
            }
        }
    }
    return next_hops;
}

fn read_file(file: &str) -> Vec<String> {
    let reader = BufReader::new(File::open(file).expect("could not open file"));
    reader
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}
