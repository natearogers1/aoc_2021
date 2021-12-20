use core::{borrow, num};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn run() {
    let inputs = read_file("inputs/day11.txt");

    let opuses = build_grid(inputs);
    //print_grid(opuses);
    println!("-------------------------------------");
    //
    let num_flashes = part1(opuses, 999);
    println!("{}", num_flashes)
    //
}

fn part1(mut opuses: Vec<Opus>, rounds: u32) -> usize {
    let mut count_flashes = 0;
    for r in 0..rounds {
        // increment all opuses
        for opus in opuses.iter_mut() {
            opus.flash = false;
            opus.exhausted = false;
            opus.increment();
        }

        // get vec of flashed opuses
        let mut flashed_opuses: Vec<Opus> = opuses.iter().filter(|x| x.flash).cloned().collect();
        if r < 100 {
            count_flashes += flashed_opuses.len();
        }
        while flashed_opuses.len() > 0 {
            // find all the neighbors of the flashed opuses
            for flashed_opus in flashed_opuses {
                let neighbor_opuses: Vec<Opus> = opuses
                    .iter()
                    .filter(|o| o.is_neighbor(&flashed_opus))
                    .cloned()
                    .collect();

                // increment those neghbors
                for i in 0..opuses.len() {
                    if neighbor_opuses.contains(&&opuses[i]) {
                        if !opuses[i].flash {
                            let _ = &mut opuses[i].increment();
                        }
                    }
                    // mark all the flashed opuses as exhausted
                    if opuses[i] == flashed_opus {
                        opuses[i].exhausted = true;
                    }
                }
            }

            // overwrite flashed_opuses with newly flashed opuses who are not exhausted
            flashed_opuses = opuses
                .iter()
                .filter(|x| x.flash && !x.exhausted)
                .cloned()
                .collect();
            if r < 100 {
                count_flashes += flashed_opuses.len();
            }
            let all_flashed: Vec<Opus> = opuses.iter().filter(|x| x.flash).cloned().collect();
            if all_flashed.len() == 100 {
                println!("All flashed at step {}", r + 1);
                return count_flashes;
            }
        }
    }

    return count_flashes;
}

#[derive(Clone, Copy)]
struct Position {
    x: i8,
    y: i8,
}
#[derive(Clone, Copy)]
struct Opus {
    energy: i8,
    flash: bool,
    exhausted: bool,
    position: Position,
}
impl Opus {
    fn process_flashes(&mut self, flashes: &Vec<&Opus>) {
        for opus in flashes {
            if self.is_neighbor(opus) {
                self.increment()
            }
        }
    }
    fn increment(&mut self) {
        self.energy += 1;
        if self.energy > 9 {
            self.flash = true;
            self.energy = 0;
        }
    }

    fn is_neighbor(&self, other: &Opus) -> bool {
        if (self.position.x - other.position.x).abs() <= 1 {
            if (self.position.y - other.position.y).abs() <= 1 {
                if !self.eq(other) {
                    return true;
                }
            }
        }
        return false;
    }
}
impl PartialEq for Opus {
    fn eq(&self, other: &Self) -> bool {
        return self.position.x == other.position.x && self.position.y == other.position.y;
    }
}
impl fmt::Display for Opus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.energy,)
    }
}

fn print_grid(opuses: &Vec<Opus>) {
    for row in 0..10 {
        for o in 0..10 {
            print!("{}", opuses[row * 10 + o])
        }
        println!("")
    }
}

fn build_grid(inputs: Vec<Vec<i8>>) -> Vec<Opus> {
    let mut opuses: Vec<Opus> = vec![];
    for (row_num, row) in inputs.iter().enumerate() {
        for (opus_num, opus_level) in row.iter().enumerate() {
            // add Opus to grid
            opuses.push(Opus {
                energy: *opus_level,
                flash: false,
                exhausted: false,
                position: Position {
                    x: i8::try_from(opus_num).unwrap(),
                    y: i8::try_from(row_num).unwrap(),
                },
            });
        }
    }
    return opuses;
}

fn read_file(file: &str) -> Vec<Vec<i8>> {
    let reader = BufReader::new(File::open(file).expect("could not open file"));
    let lines_vec: Vec<Vec<i8>> = reader
        .lines()
        .map(|l| {
            l.expect("couldn't read")
                .split("")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .collect();

    return lines_vec;
}

#[cfg(test)]
mod unit_test {

    use core::num;

    use crate::day11::{part1, Opus, Position};
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_flash_sequential_neighbors() {
        let opus1 = Opus {
            energy: 9,
            flash: false,
            exhausted: false,
            position: Position { x: 0, y: 0 },
        };
        let opus2 = Opus {
            energy: 9,
            flash: false,
            exhausted: false,
            position: Position { x: 0, y: 1 },
        };
        let opus3 = Opus {
            energy: 9,
            flash: false,
            exhausted: false,
            position: Position { x: 0, y: 2 },
        };
        let opuses = vec![opus1, opus2, opus3];
        // one round
        let num_flashes = part1(opuses, 1);
        assert_eq!(num_flashes, 3);
        assert_eq!(opus1.energy, 1);
        // two rounds
        //let num_flashes = part1(opuses, 1);
        //println!("{}", num_flashes);
        //assert_eq!(num_flashes, 0)
    }

    #[test]
    fn test_detect_neighbors() {
        let opus1 = Opus {
            energy: 0,
            flash: false,
            exhausted: false,
            position: Position { x: 5, y: 5 },
        };
        let opus2 = Opus {
            energy: 0,
            flash: false,
            exhausted: false,
            position: Position { x: 6, y: 5 },
        };
        assert_eq!(opus1.is_neighbor(&opus2), true);

        let opus3 = Opus {
            energy: 0,
            flash: false,
            exhausted: false,
            position: Position { x: 5, y: 6 },
        };
        let opus4 = Opus {
            energy: 0,
            flash: false,
            exhausted: false,
            position: Position { x: 5, y: 5 },
        };
        assert_eq!(opus3.is_neighbor(&opus4), true);
        let opus5 = Opus {
            energy: 0,
            flash: false,
            exhausted: false,
            position: Position { x: 5, y: 5 },
        };
        let opus6 = Opus {
            energy: 0,
            flash: false,
            exhausted: false,
            position: Position { x: 4, y: 4 },
        };
        assert_eq!(opus5.is_neighbor(&opus6), true);
        let opus7 = Opus {
            energy: 0,
            flash: false,
            exhausted: false,
            position: Position { x: 5, y: 5 },
        };
        let opus8 = Opus {
            energy: 0,
            flash: false,
            exhausted: false,
            position: Position { x: 9, y: 9 },
        };
        assert_eq!(opus7.is_neighbor(&opus8), false);
        let opus9 = Opus {
            energy: 0,
            flash: false,
            exhausted: false,
            position: Position { x: 5, y: 5 },
        };
        let opus10 = Opus {
            energy: 0,
            flash: false,
            exhausted: false,
            position: Position { x: 5, y: 5 },
        };
        assert_eq!(opus9.is_neighbor(&opus10), false);
    }
}
