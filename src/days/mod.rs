#![allow(dead_code)]
use std::fs::read_to_string;

use crate::problem::Problem;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

pub fn run_day<P: Problem + ?Sized>(problem: &P, input: &str) {
    println!("{}", problem.get_title());
    println!("  Part One: {}", problem.part_one(input));
    println!("  Part Two: {}", problem.part_two(input));
    println!("  Part Three: {}", problem.part_three(input));
    println!();
}

pub fn run() {
    let problems: Vec<Box<dyn Problem>> = vec![
        Box::new(day01::Problem1),
        Box::new(day02::Problem2),
        Box::new(day03::Problem3),
        Box::new(day04::Problem4),
        Box::new(day05::Problem5),
        Box::new(day06::Problem6),
        Box::new(day07::Problem7),
        Box::new(day08::Problem8),
        Box::new(day09::Problem9),
        Box::new(day10::Problem10),
        Box::new(day11::Problem11),
        Box::new(day12::Problem12),
        Box::new(day13::Problem13),
    ];

    problems.iter().for_each(|problem| {
        let input = read_to_string(format!("inputs/{}.txt", problem.get_input())).unwrap();
        run_day(problem.as_ref(), &input)
    });
}
