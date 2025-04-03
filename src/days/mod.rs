#![allow(dead_code)]
use std::fs::read_to_string;

use crate::problem::Problem;

mod day01;
use day01::Problem1;
mod day02;
use day02::Problem2;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub fn run_day<P: Problem + ?Sized>(problem: &P, input: &str) {
    println!("{}:", problem.get_title());
    println!("  Part One: {}", problem.part_one(input));
    println!("  Part Two: {}", problem.part_two(input));
    println!("  Part Three: {}", problem.part_three(input));
    println!();
}

pub fn run() {
    let problems: Vec<Box<dyn Problem>> = vec![Box::new(Problem1), Box::new(Problem2)];

    problems.iter().for_each(|problem| {
        let input = read_to_string(format!("inputs/{}.txt", problem.get_input())).unwrap();
        run_day(problem.as_ref(), &input)
    });
}
