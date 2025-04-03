use std::str::FromStr;

use crate::problem::Problem;

pub struct Problem2;

impl Problem for Problem2 {
    fn get_input(&self) -> String {
        "input2".into()
    }

    fn get_title(&self) -> String {
        "Problem 2: Absurd Arithmetic!".into()
    }

    fn part_one(&self, input: &str) -> String {
        let (instructions, mut values) = parse(input);
        values.sort();

        format!(
            "{}",
            (instructions.a)((instructions.b)((instructions.c)(values[values.len() / 2])))
        )
    }

    fn part_two(&self, input: &str) -> String {
        let (instructions, values) = parse(input);
        let room_values = values.iter().filter(|v| **v % 2 == 0).sum();

        format!(
            "{}",
            (instructions.a)((instructions.b)((instructions.c)(room_values)))
        )
    }

    fn part_three(&self, input: &str) -> String {
        let (instructions, mut values) = parse(input);
        let upper_bound: Number = 15000000000000;
        values.sort();
        values.reverse();

        format!(
            "{}",
            *values
                .iter()
                .find(
                    |value| (instructions.a)((instructions.b)((instructions.c)(**value)))
                        < upper_bound
                )
                .unwrap()
        )
    }
}

type Number = u64;

struct Instructions {
    pub a: Box<dyn Fn(Number) -> Number>,
    pub b: Box<dyn Fn(Number) -> Number>,
    pub c: Box<dyn Fn(Number) -> Number>,
}

impl FromStr for Instructions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        fn get_method(line: &str) -> Box<dyn Fn(Number) -> Number> {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let value = words[words.len() - 1].parse::<Number>().unwrap();
            let instruction = words[words.len() - 2];

            match instruction {
                "ADD" => Box::new(move |x| x + value),
                "MULTIPLY" => Box::new(move |x| x * value),
                _ => Box::new(move |x| x.pow(value as u32)),
            }
        }

        Ok(Instructions {
            a: get_method(lines[0]),
            b: get_method(lines[1]),
            c: get_method(lines[2]),
        })
    }
}

fn parse(input: &str) -> (Instructions, Vec<Number>) {
    let (instructions, values) = input.split_once("\n\n").unwrap();

    (
        instructions.parse().unwrap(),
        values.lines().map(|v| v.parse().unwrap()).collect(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Function A: ADD 495
Function B: MULTIPLY 55
Function C: RAISE TO THE POWER OF 3

5219
8933
3271
7128
9596
9407
7005
1607
4084
4525
5496";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem2;
        assert_eq!(problem.part_one(INPUT), "9130674516975");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem2;
        assert_eq!(problem.part_two(INPUT), "1000986169836015");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem2;
        assert_eq!(problem.part_three(INPUT), "5496");
    }
}
