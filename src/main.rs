#![feature(test)]
extern crate test;

mod day1;
mod day2;
mod day4;
mod day5;

fn main() {
    println!("\nday1:");
    day1::run();
    println!("\nday2:");
    day2::run();
    println!("\nday4:");
    day4::run();
    println!("\nday5:");
    day5::run();
}
