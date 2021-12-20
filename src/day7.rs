use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::num;

pub fn run() {
    let input = read_file("inputs/test/day7.txt");
    let max = input.iter().max().unwrap();
    let min = input.iter().min().unwrap();

    let part_1 = part1(&input, *min, *max);
    println!("{}", part_1);
    let part_2 = part2(&input, *min, *max);
    println!("{}", part_2);
}

fn part1(input: &Vec<i32>, min: i32, max: i32) -> i32 {
    let mut min_cost = i32::MAX;
    for i in min..max {
        let abs_vals: Vec<i32> = input.iter().map(|x| (x - i).abs()).collect();
        let cost: i32 = abs_vals.iter().sum::<i32>();
        if cost < min_cost {
            min_cost = cost;
        }
    }
    return min_cost;
}

fn part2(input: &Vec<i32>, min: i32, max: i32) -> i32 {
    let mut min_cost = i32::MAX;
    for i in min..max {
        let abs_vals: Vec<i32> = input.iter().map(|x| (x - i).abs()).collect();
        let x: Vec<i32> = abs_vals.iter().map(|x|((x ^ 2) + x) / 2).collect();
        let cost: i32 = x.iter().sum::<i32>();
        if cost < min_cost {
            min_cost = cost;
        }
    }
    return min_cost;
}

fn read_file(file: &str) -> Vec<i32> {
    let line = fs::read_to_string(file).unwrap();
    let vec_i32: Vec<i32> = line.split(",").map(|l| l.parse::<i32>().unwrap()).collect();
    return vec_i32;
}
