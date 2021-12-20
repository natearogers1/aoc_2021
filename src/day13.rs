use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn run() {
    let (mut dots, mut folds) = read_file("inputs/day13.txt");

    while !folds.is_empty() {
        let fold = folds.pop().unwrap();
        println!("{:?}", fold);
        dots = dots
            .into_iter()
            .map(|p| {
                if fold.axis == 'y' {
                    p.fold_y(fold.value)
                } else if fold.axis == 'x' {
                    p.fold_x(fold.value)
                } else {
                    p
                }
            })
            .collect();
    }

    let mut unique_folded: Vec<Pos> = vec![];
    for d in dots {
        if !unique_folded.contains(&d) {
            unique_folded.push(d)
        }
    }
    println!("{:?}", unique_folded);
    //println!("{:?}", unique_folded.len());
    print_paper(unique_folded)
}

fn print_paper(paper: Vec<Pos>) {
    for row in 0..50 {
        for column in 0..50 {
            let pos: Vec<&Pos> = paper
                .iter()
                .filter(|p| p.x == column && p.y == row)
                .collect();
            if pos.len() > 0 {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!("")
    }
}

#[derive(Debug)]
struct Fold {
    axis: char,
    value: i32,
}
#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn fold_x(self, x: i32) -> Self {
        if self.x > x {
            return Pos {
                x: ((self.x - x) - x).abs(),
                y: self.y,
            };
        }
        self
    }

    fn fold_y(self, y: i32) -> Self {
        if self.y > y {
            return Pos {
                x: self.x,
                y: ((self.y - y) - y).abs(),
            };
        }
        self
    }
}
impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn read_file(file: &str) -> (Vec<Pos>, Vec<Fold>) {
    let reader = BufReader::new(File::open(file).expect("could not open file"));
    let mut lines_vec: Vec<String> = reader
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect();

    let mut folds: Vec<Fold> = vec![];

    while let Some(line) = lines_vec.pop() {
        if line == "" {
            break;
        }
        let values: Vec<&str> = line.trim_start_matches("fold along ").split("=").collect();
        folds.push(Fold {
            axis: values[0].parse::<char>().unwrap(),
            value: values[1].parse::<i32>().unwrap(),
        })
    }

    let dots = lines_vec
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| Pos {
            x: l.split(",").collect::<Vec<&str>>()[0]
                .parse::<i32>()
                .unwrap(),
            y: l.split(",").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap(),
        })
        .collect();

    return (dots, folds);
}
