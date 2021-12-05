use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn run() {
    // read inputs from a file
    let input = read_file("inputs/day4.txt");

    // take the first line as the Vec<u32> of bingo numbers

    // use a sliding window of 5 lines to instantiate bingo structs where all lines are populated
    // results in Vec<Board>
    let mut boards = find_boards(&input);

    // for each bingo number, loop through the Vec<Board> and add the number
    fill_boards(&mut boards, &input);
}
// 5 lines of 5 numbers each
#[derive(Debug, Copy, Clone)]
struct Value {
    number: u32,
    marked: bool,
}
#[derive(Debug)]
struct Board {
    grid: [[Value; 5]; 5],
    solved: bool,
}
impl Board {
    fn from_window(window: &[String]) -> Self {
        let mut grid = [[Value {
            number: 0,
            marked: false,
        }; 5]; 5];
        for (row_num, row_values) in window.iter().enumerate() {
            let column_values: Vec<u32> = row_values
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect();
            for (column_num, column_value) in column_values.iter().enumerate() {
                grid[row_num][column_num].number = *column_value
            }
        }
        Board {
            grid: grid,
            solved: false,
        }
    }
    fn mark_number(&mut self, number: &u32) {
        for row in &mut (self).grid {
            for value in row {
                if value.number == *number {
                    value.marked = true;
                }
            }
        }
    }

    fn is_solved(&self) -> bool {
        // check for row completions
        for row in &(self).grid {
            if row.iter().all(|x| x.marked) {
                return true;
            }
        }
        // check for column completions
        // get length of grid vec - to support any number of columns
        let grid_len = self.grid.iter().count();
        for i in 0..grid_len {
            let column: Vec<&Value> = self.grid.iter().map(|x| &x[i]).collect();
            if column.iter().all(|x| x.marked) {
                return true;
            }
        }
        return false;
    }

    fn sum_unpicked(&self) -> u32 {
        let mut sum = 0;
        for row in &self.grid {
            for value in row {
                if value.marked == false {
                    sum += value.number
                }
            }
        }
        return sum;
    }
}

fn find_boards(input: &Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    for window in input.windows(5) {
        if window.iter().all(|x| x != "") {
            let board = Board::from_window(window);
            boards.push(board)
        }
    }
    return boards;
}

fn fill_boards(boards: &mut Vec<Board>, input: &Vec<String>) {
    let mut first_board_score = 0;
    let mut last_board_score = 0;

    let bingo_numbers: Vec<u32> = input[0]
        .split(',')
        .map(|l| l.parse::<u32>().expect("couldn't parse"))
        .collect();

    for bingo_number in bingo_numbers {
        for board in boards // filter every time for unsolved boards
            .iter_mut()
            .filter(|b| b.is_solved() == false)
            .collect::<Vec<&mut Board>>()
        {
            board.mark_number(&bingo_number);
            if board.is_solved() == true {
                if first_board_score == 0 {
                    first_board_score = board.sum_unpicked() * bingo_number;
                }
                last_board_score = board.sum_unpicked() * bingo_number;
            }
        }
    }
    println!("First board score: {}", first_board_score);
    println!("Last board score: {}", last_board_score);
}

fn read_file(file: &str) -> Vec<String> {
    let reader = BufReader::new(File::open(file).expect("could not open file"));
    reader
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}
