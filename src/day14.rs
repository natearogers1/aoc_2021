use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use itertools::Itertools;

pub fn run() {
    let (init_chain, transforms) = read_file("inputs/day14.txt");

    let part1 = part1(&init_chain, &transforms);
    println!("part1: {}", part1);
    let part2 = part2(&init_chain, &transforms);
}
fn part2(init_chain: &String, transforms: &Vec<Transform>) -> i128 {
    let mut letter_map = HashMap::new();
    for char in init_chain.chars() {
        *letter_map.entry(char.to_string()).or_insert(0) += 1;
    }
    println!("{:?}", letter_map);
    let mut final_map = HashMap::new();
    // initial map population
    for (f, l) in init_chain.split("").into_iter().tuple_windows() {
        if f == "" || l == "" {
            continue;
        }
        *final_map.entry(f.to_owned() + l).or_insert(0) += 1;
    }
    for i in 0..40 {
        //println!("{:?}", final_map);

        let mut new_map: HashMap<String, i128> = HashMap::new();
        for (pair, count) in &final_map {
            // find transform
            match transforms.iter().find(|x| &x.pair == pair) {
                Some(t) => {
                    let (p1, p2) = t.new_pairs();
                    *new_map.entry(p1).or_insert(0) += count;
                    *new_map.entry(p2).or_insert(0) += count;
                    *new_map.entry(pair.to_string()).or_insert(0) -= count;
                    *letter_map.entry(t.insert.to_string()).or_insert(0) += count;
                }
                _ => continue,
            }
        }
        // add new_map keys to final map all at once
        for (pair, count) in &new_map {
            *final_map.entry(pair.to_string()).or_insert(0) += count;
        }
    }

    let most_char = letter_map.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    let least_char = letter_map.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();

    println!("{:?}", letter_map);
    println!("{:?}", most_char.1 - least_char.1);
    return most_char.1 - least_char.1;
}

fn part1(init_chain: &String, transforms: &Vec<Transform>) -> i32 {
    let mut new_chain = init_chain.to_owned();
    for i in 0..10 {
        new_chain = step(&new_chain, transforms);
    }
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in new_chain.chars() {
        *map.entry(c).or_insert(1) += 1;
    }
    let most_char = map.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    let least_char = map.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();
    //println!("{}", most_char.0);
    //println!("{}", least_char.0);

    return most_char.1 - least_char.1;
}

fn step(init_chain: &String, transforms: &Vec<Transform>) -> String {
    let mut new_chain = String::new();
    let mut iter = init_chain.chars().into_iter();
    let mut first_char = iter.next().unwrap();

    new_chain += &first_char.to_string();

    while let Some(next_char) = iter.next() {
        let pair = first_char.to_string() + &next_char.to_string();
        for t in transforms {
            if t.pair == pair {
                //println!("match - {}", t.pair);
                new_chain += &t.insert;
            }
        }
        new_chain += &next_char.to_string();
        first_char = next_char;
    }
    //println!("{}", new_chain);
    return new_chain;
}

/*
fn part1(init_chain: &String, transforms: &Vec<Transform>) {
    let mut final_chain = init_chain.to_owned();
    for t in transforms {
        //println!("next transform");
        let mut new_chain = final_chain.to_owned();
        //println!("new_chain loop - {}", new_chain);
        for i in 0..final_chain.chars().count() {
            let (front, back) = final_chain.split_at(i);
            if front == "" || back == "" {
                continue;
            }
            let pair = front.to_owned().chars().last().unwrap().to_string()
                + &back.to_owned().chars().next().unwrap().to_string();
            if t.pair == pair {
                println!("match - {}:{}", pair, t.pair);
                new_chain.insert_str(i, &t.insert);
            }
        }
        if final_chain != new_chain {
            println!("chain update");
            final_chain = new_chain;
            println!("{}", &final_chain)
        }
    }
}
*/

#[derive(Debug)]
struct Transform {
    pair: String,
    insert: String,
}
impl Transform {
    fn new(line: &String) -> Self {
        let split: Vec<&str> = line.split(" -> ").collect();
        Transform {
            pair: split[0].to_string(),
            insert: split[1].to_string(),
        }
    }
    fn new_pairs(&self) -> (String, String) {
        let (l1, l2) = self.pair.chars().collect_tuple().unwrap();
        return (
            l1.to_string() + &self.insert,
            self.insert.to_owned() + &l2.to_string(),
        );
    }
}

fn read_file(file: &str) -> (String, Vec<Transform>) {
    let reader = BufReader::new(File::open(file).expect("could not open file"));
    let lines_vec: Vec<String> = reader
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect();

    let (init_chain, transforms) = lines_vec.split_first().unwrap().to_owned();
    let transforms = transforms
        .into_iter()
        .filter(|l| l.trim() != "")
        .map(|l| Transform::new(l))
        .collect();
    return (init_chain.to_owned(), transforms);
}
