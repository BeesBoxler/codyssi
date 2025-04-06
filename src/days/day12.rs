use std::{collections::VecDeque, str::FromStr};

use crate::problem::Problem;

pub struct Problem12;

impl Problem for Problem12 {
    fn get_input(&self) -> String {
        "input12".into()
    }

    fn get_title(&self) -> String {
        "Problem 12: Challenging the Whirlpool".into()
    }

    fn part_one(&self, input: &str) -> String {
        let (mut grid, instructions, _) = parse_input(input);

        instructions
            .iter()
            .for_each(|instruction| instruction.execute(&mut grid));

        format!("{}", find_largest_value(grid))
    }

    fn part_two(&self, input: &str) -> String {
        let (mut grid, mut instructions, actions) = parse_input(input);

        let mut curr = None;

        for action in actions {
            match action {
                Action::Take => curr = instructions.pop_front(),
                Action::Cycle => instructions.push_back(curr.clone().unwrap()),
                Action::Act => curr.clone().unwrap().clone().execute(&mut grid),
            }
        }

        format!("{}", find_largest_value(grid))
    }

    fn part_three(&self, input: &str) -> String {
        let (mut grid, mut instructions, actions) = parse_input(input);

        let mut curr = Some(instructions[0].clone());

        while instructions.len() > 0 {
            for action in actions.clone() {
                if curr.is_none() {
                    break;
                }

                match action {
                    Action::Take => curr = instructions.pop_front(),
                    Action::Cycle => instructions.push_back(curr.clone().unwrap()),
                    Action::Act => curr.clone().unwrap().clone().execute(&mut grid),
                }
            }
        }

        format!("{}", find_largest_value(grid))
    }
}

fn parse_input(input: &str) -> (Grid, VecDeque<Instruction>, Vec<Action>) {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    (
        sections[0]
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|c| c.parse().unwrap())
                    .collect()
            })
            .collect(),
        sections[1]
            .lines()
            .map(|line| line.parse().unwrap())
            .collect(),
        sections[2]
            .lines()
            .map(|line| line.parse().unwrap())
            .collect(),
    )
}

fn find_largest_value(grid: Grid) -> usize {
    grid.iter()
        .map(|row| row.iter().sum::<usize>())
        .max()
        .unwrap()
        .max(
            (0..grid[0].len())
                .map(|i| (0..grid.len()).map(|j| grid[j][i]).sum::<usize>())
                .max()
                .unwrap(),
        )
}

type Grid = Vec<Vec<usize>>;

#[derive(Debug, Clone)]
struct Instruction(Manipulation, Target);

impl Instruction {
    fn execute(&self, grid: &mut Grid) {
        match self.0 {
            Manipulation::Shift(amt) => match self.1 {
                Target::Col(i) => (0..amt).for_each(|_| {
                    let len = grid.len();
                    let temp = grid[len - 1][i];
                    (0..len - 1).rev().for_each(|j| grid[j + 1][i] = grid[j][i]);
                    grid[0][i] = temp;
                }),
                Target::Row(i) => grid[i].rotate_right(amt),
                _ => panic!("Invalid Instruction"),
            },
            Manipulation::Add(amt) => match self.1 {
                Target::Col(i) => grid
                    .iter_mut()
                    .for_each(|row| row[i] = (row[i] + amt) % 1073741824),
                Target::Row(i) => grid[i]
                    .iter_mut()
                    .for_each(|v| *v = (*v + amt) % 1073741824),
                Target::All => grid
                    .iter_mut()
                    .for_each(|row| row.iter_mut().for_each(|v| *v = (*v + amt) % 1073741824)),
            },
            Manipulation::Sub(amt) => match self.1 {
                Target::Col(i) => grid
                    .iter_mut()
                    .for_each(|row| row[i] = (row[i] - amt) % 1073741824),
                Target::Row(i) => grid[i]
                    .iter_mut()
                    .for_each(|v| *v = (*v - amt) % 1073741824),
                Target::All => grid
                    .iter_mut()
                    .for_each(|row| row.iter_mut().for_each(|v| *v = (*v - amt) % 1073741824)),
            },
            Manipulation::Multiply(amt) => match self.1 {
                Target::Col(i) => grid
                    .iter_mut()
                    .for_each(|row| row[i] = (row[i] * amt) % 1073741824),
                Target::Row(i) => grid[i]
                    .iter_mut()
                    .for_each(|v| *v = (*v * amt) % 1073741824),
                Target::All => grid
                    .iter_mut()
                    .for_each(|row| row.iter_mut().for_each(|v| *v = (*v * amt) % 1073741824)),
            },
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();

        let manipulation = match words[0] {
            "SHIFT" => Manipulation::Shift(words[4].parse().unwrap()),
            "ADD" => Manipulation::Add(words[1].parse().unwrap()),
            "SUB" => Manipulation::Sub(words[1].parse().unwrap()),
            "MULTIPLY" => Manipulation::Multiply(words[1].parse().unwrap()),
            _ => return Err("Invalid instruction".into()),
        };

        let target = match manipulation {
            Manipulation::Shift(_) => match words[1] {
                "COL" => Target::Col(words[2].parse::<usize>().unwrap() - 1),
                "ROW" => Target::Row(words[2].parse::<usize>().unwrap() - 1),
                _ => return Err("Invalid instruction".into()),
            },
            _ => match words[2] {
                "ALL" => Target::All,
                "COL" => Target::Col(words[3].parse::<usize>().unwrap() - 1),
                "ROW" => Target::Row(words[3].parse::<usize>().unwrap() - 1),
                _ => return Err("Invalid instruction".into()),
            },
        };

        Ok(Instruction(manipulation, target))
    }
}

#[derive(Debug, Clone)]
enum Manipulation {
    Shift(usize),
    Add(usize),
    Sub(usize),
    Multiply(usize),
}

#[derive(Debug, Clone)]
enum Target {
    Col(usize),
    Row(usize),
    All,
}

#[derive(Debug, Clone)]
enum Action {
    Take,
    Cycle,
    Act,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "TAKE" => Self::Take,
            "CYCLE" => Self::Cycle,
            "ACT" => Self::Act,
            _ => return Err("Invalid Action".into()),
        })
    }
}

fn draw_grid(grid: &Grid) {
    for row in grid {
        for entry in row {
            print!("{} ", entry);
        }
        println!();
    }
    println!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "222 267 922 632 944
110 33 503 758 129
742 697 425 362 568
833 408 425 349 631
874 671 202 430 602

SHIFT COL 2 BY 1
MULTIPLY 4 COL 5
SUB 28 ALL
SHIFT COL 4 BY 2
MULTIPLY 4 ROW 4
ADD 26 ROW 3
SHIFT COL 4 BY 2
ADD 68 ROW 2

TAKE
CYCLE
TAKE
ACT
TAKE
CYCLE";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem12;
        assert_eq!(problem.part_one(INPUT), "18938");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem12;
        assert_eq!(problem.part_two(INPUT), "11496");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem12;
        assert_eq!(problem.part_three(INPUT), "19022");
    }
}
