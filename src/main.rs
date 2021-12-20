#![feature(test)]
extern crate itertools;
extern crate test;

mod day1;
mod day11;
//mod day11_issue;
mod day12;
mod day13;
mod day2;
mod day4;
mod day5;
mod day7;
mod day8;

fn main() {
    println!("\nday1:");
    //day1::run();
    println!("\nday2:");
    //day2::run();
    println!("\nday4:");
    //day4::run();
    println!("\nday5:");
    //day5::run();
    println!("\nday7:");
    //day7::run();
    //println!("\nday8:");
    //day8::run();
    println!("\nday11:");
    //day11::run();
    //day11_issue::run()
    //day12::run();
    day13::run();
}
