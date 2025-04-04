use std::{collections::HashSet, str::FromStr, string::ParseError};

use tuple_map::TupleMap2;

use crate::problem::Problem;

pub struct Problem3;

impl Problem for Problem3 {
    fn get_input(&self) -> String {
        "input3".into()
    }

    fn get_title(&self) -> String {
        "Problem 3: Supplies in Surplus".into()
    }

    fn part_one(&self, input: &str) -> String {
        format!(
            "{}",
            parse_input(input)
                .iter()
                .map(|pair| pair.0.len() + pair.1.len())
                .sum::<Number>()
        )
    }

    fn part_two(&self, input: &str) -> String {
        format!(
            "{}",
            parse_input(input)
                .iter()
                .map(|pair| pair.0.union(&pair.1).count())
                .sum::<Number>()
        )
    }

    fn part_three(&self, input: &str) -> String {
        format!(
            "{}",
            parse_input(input)
                .iter()
                .map(|pair| pair.0.union(&pair.1).collect::<HashSet<_>>())
                .collect::<Vec<_>>()
                .windows(2)
                .map(|window| window[0].union(&window[1]).count())
                .max()
                .unwrap()
        )
    }
}

type Number = usize;
struct PairRange(HashSet<Number>, HashSet<Number>);

impl FromStr for PairRange {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn to_range(range: &str) -> HashSet<Number> {
            let (min, max) = range
                .split_once('-')
                .unwrap()
                .map(|value| value.parse().unwrap());

            (min..=max).collect()
        }

        let (l, r) = s.split_once(' ').unwrap().map(to_range);

        Ok(PairRange(l, r))
    }
}

fn parse_input(input: &str) -> Vec<PairRange> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "8-9 9-10
7-8 8-10
9-10 5-10
3-10 9-10
4-8 7-9
9-10 2-7";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem3;
        assert_eq!(problem.part_one(INPUT), "43");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem3;
        assert_eq!(problem.part_two(INPUT), "35");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem3;
        assert_eq!(problem.part_three(INPUT), "9");
    }
}
