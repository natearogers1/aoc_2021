use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn run() {
    let lines = read_file("inputs/day8.txt");

    //part1(lines)
    part2(lines);
}

fn part2(lines: Vec<String>) {
    for InputOutputLine in lines {
        let signals = deduce_signals(&InputOutputLine);
    }
}

fn deduce_signals(InputOutputLine: &str) -> HashMap<char, char> {
    let mut SignalMappings = HashMap::new();
    let inputs: &str = InputOutputLine.split("|").collect::<Vec<&str>>()[0];
    let outputs: &str = InputOutputLine.split("|").collect::<Vec<&str>>()[1];
    let a_signal = deduce_a(inputs);
    SignalMappings.insert('a', a_signal);
    let e_signal = deduce_e(inputs);

    SignalMappings.insert('a', a_signal);

    println!("{:?}", SignalMappings);

    return SignalMappings;
}

fn deduce_a(input_line: &str) -> char {
    let MsgOne = input_line
        .split_whitespace()
        .filter(|x| x.chars().count() == 2)
        .next()
        .unwrap();
    let MsgSeven = input_line
        .split_whitespace()
        .filter(|x| x.chars().count() == 3)
        .next()
        .unwrap();
    let a = MsgSeven
        .chars()
        .filter(|x| MsgOne.contains(*x) == false)
        .next()
        .unwrap();

    return a;
}

fn deduce_e(input_line: &str) -> char {
    let MsgOne = input_line
        .split_whitespace()
        .filter(|x| x.chars().count() == 2)
        .next()
        .unwrap();
    let MsgEight = input_line
        .split_whitespace()
        .filter(|x| x.chars().count() == 7)
        .next()
        .unwrap();
    let MsgNine = input_line
        .split_whitespace()
        .filter(|x| x.chars().count() == 6)
        .filter(|x| x.contains(MsgOne))
        .next()
        .unwrap();

    return 'e';
}

fn part1(lines: Vec<String>) {
    let inputs: Vec<&str> = lines
        .iter()
        .map(|x| x.split("|").collect::<Vec<&str>>()[0])
        .collect::<Vec<&str>>();
    let outputs: Vec<&str> = lines
        .iter()
        .map(|x| x.split("|").collect::<Vec<&str>>()[1])
        .collect::<Vec<&str>>();

    let output_list = gather_outputs(outputs);
    count_outputs(&output_list);
    println!("{}", &output_list.iter().count())
}

fn count_outputs(outputs: &Vec<i8>) {
    let one_count = outputs.iter().filter(|x| **x == 1).count();
    let four_count = outputs.iter().filter(|x| **x == 4).count();
    let seven_count = outputs.iter().filter(|x| **x == 7).count();
    let eight_count = outputs.iter().filter(|x| **x == 8).count();

    println!("{}", one_count);
    println!("{}", four_count);
    println!("{}", seven_count);
    println!("{}", eight_count);
}

fn gather_outputs(outputs: Vec<&str>) -> Vec<i8> {
    let mut values: Vec<i8> = vec![];

    for output in outputs {
        let patterns: Vec<&str> = output.split_whitespace().collect();
        for pattern in patterns {
            match pattern.chars().count() {
                2 => values.push(1),
                4 => values.push(4),
                3 => values.push(7),
                7 => values.push(8),
                _ => (),
            }
        }
    }
    return values;
}

fn read_file(file: &str) -> Vec<String> {
    let reader = BufReader::new(File::open(file).expect("could not open file"));
    reader
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}
