use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

pub fn run() {
    // read the inputs from file
    let movement_strings = read_file("inputs/day2.txt");

    // cast the input lines into a single vector consisting of Movement structs Arr
    let movements: Vec<Movement> = movement_strings.iter().map(|l| new_movement(l)).collect();

    part1(&movements);
    part2(&movements);
}

fn part1(movements: &Vec<Movement>) {
    // instantiate new struct position with 0 values
    let mut position = Position { x: 0, y: 0, aim: 0 };
    // for each movement increment the appropriate member of struct Position
    for movement in movements {
        match movement.direction {
            Direction::Up => position.y -= movement.value,
            Direction::Down => position.y += movement.value,
            Direction::Forward => position.x += movement.value,
        }
    }
    println!("{:?}", position);
    println!("product: {}", position.x * position.y);
}

fn part2(movements: &Vec<Movement>) {
    // instantiate new struct position with 0 values
    let mut position = Position { x: 0, y: 0, aim: 0 };

    for movement in movements {
        match movement.direction {
            Direction::Up => position.aim -= movement.value,
            Direction::Down => position.aim += movement.value,
            Direction::Forward => {
                position.x += movement.value;
                position.y += position.aim * movement.value;
            }
        }
    }
    println!("{:?}", position);
    println!("product: {}", position.x * position.y);
}

enum Direction {
    Up,
    Down,
    Forward,
}
impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(()),
        }
    }
}
struct Movement {
    direction: Direction,
    value: u32,
}

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
    aim: u32,
}

fn new_movement(movement: &str) -> Movement {
    let m: Vec<&str> = movement.split(' ').collect();
    let direction = m[0];
    let direction_enum = Direction::from_str(direction).unwrap();
    let value = m[1].parse::<u32>().unwrap();

    return Movement {
        direction: direction_enum,
        value: value,
    };
}

fn read_file(file: &str) -> Vec<String> {
    let reader = BufReader::new(File::open(file).expect("could not open file"));
    reader
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}
