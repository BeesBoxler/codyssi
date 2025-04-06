use crate::problem::Problem;
use std::str::FromStr;

const LOOKUP: [char; 68] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '!', '@', '#', '$', '%', '^',
];

pub struct Problem11;

impl Problem for Problem11 {
    fn get_input(&self) -> String {
        "input11".into()
    }

    fn get_title(&self) -> String {
        "Problem 11: Games in a Storm".into()
    }

    fn part_one(&self, input: &str) -> String {
        format!(
            "{}",
            parse_input(input)
                .iter()
                .map(|pair| pair.convert())
                .max()
                .unwrap()
        )
    }

    fn part_two(&self, input: &str) -> String {
        to_b68(parse_input(input).iter().map(|pair| pair.convert()).sum())
    }

    fn part_three(&self, input: &str) -> String {
        let sum = parse_input(input)
            .iter()
            .map(|pair| pair.convert())
            .sum::<usize>();
        let mut n;
        let mut len = 100;
        let mut base = 68;

        while len > 4 {
            len = 0;
            n = sum.clone();
            base += 1;

            while n > 0 {
                len += 1;
                n /= base;
            }
        }

        format!("{}", base)
    }
}

struct BasePair(String, usize);

impl BasePair {
    fn convert(&self) -> usize {
        self.0.chars().fold(0, |n, c| self.1 * n + to_number(c))
    }
}

impl FromStr for BasePair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, base) = s.split_once(' ').unwrap();

        Ok(BasePair(value.into(), base.parse().unwrap()))
    }
}

fn to_b68(mut n: usize) -> String {
    let mut result = String::new();
    while n > 0 {
        result.insert(0, LOOKUP[n % 68]);
        n /= 68;
    }

    result
}

fn to_number(c: char) -> usize {
    LOOKUP.iter().position(|x| *x == c).unwrap()
}

fn parse_input(input: &str) -> Vec<BasePair> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "32IED4E6L4 22
1111300022221031003013 4
1C1117A3BA88 13
1100010000010010010001111000000010001100101 2
7AJ5G2AB4F 22
k6IHxTD 61";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem11;
        assert_eq!(problem.part_one(INPUT), "9047685997827");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem11;
        assert_eq!(problem.part_two(INPUT), "4iWAbo%6");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem11;
        assert_eq!(problem.part_three(INPUT), "2366");
    }
}
