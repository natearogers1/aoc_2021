use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::{thread::sleep, time::Duration};

pub fn run() {
    let file = match File::open("inputs/day1.txt") {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    let reader = BufReader::new(&file);

    let lines: Vec<i32> = reader
        .lines()
        .map(|x| x.unwrap().parse::<i32>())
        .collect::<Result<_, _>>()
        .unwrap();
    // why can't I do this

    let part1_result = part1(&lines);
    print!("part1 result: {}\n", part1_result);

    let part2_result = part2(&lines);
    print!("part2 result: {}\n", part2_result);
}

fn part1(lines: &Vec<i32>) -> i32 {
    let mut num_larger: i32 = 0;
    for window in lines.windows(2) {
        if window[1] > window[0] {
            num_larger += 1;
            //print!("num_larger - {}\n\n", num_larger);
            //sleep(Duration::from_millis(10000));
        }
    }
    return num_larger;
}

fn part2(lines: &Vec<i32>) -> i32 {
    let mut num_larger: i32 = 0;

    for big_window in lines.windows(4) {
        let sum_window1: &i32 = &big_window[..3].iter().sum();
        let sum_window2: &i32 = &big_window[1..].iter().sum();
        if sum_window2 > sum_window1 {
            num_larger += 1
            //sleep(Duration::from_millis(2000));
        }
    }
    return num_larger;
}
