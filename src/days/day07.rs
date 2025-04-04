use crate::problem::Problem;
use std::str::FromStr;
use tuple_map::TupleMap2;

pub struct Problem7;

impl Problem for Problem7 {
    fn get_input(&self) -> String {
        "input7".into()
    }

    fn get_title(&self) -> String {
        "Problem 7: Siren Disruption".into()
    }

    fn part_one(&self, input: &str) -> String {
        let (mut values, swaps, check) = parse_input(input);

        swaps.iter().for_each(|swap| values.swap(swap.0, swap.1));

        format!("{}", values[check])
    }

    fn part_two(&self, input: &str) -> String {
        let (mut values, mut swaps, check) = parse_input(input);
        swaps.push(swaps[0].clone());

        swaps
            .windows(2)
            .map(|swaps| Swap3(swaps[0].0, swaps[0].1, swaps[1].0))
            .for_each(|swap| {
                let hold = values[swap.2];
                values[swap.2] = values[swap.1];
                values[swap.1] = values[swap.0];
                values[swap.0] = hold;
            });

        format!("{}", values[check])
    }

    fn part_three(&self, input: &str) -> String {
        let (mut values, swaps, check) = parse_input(input);

        for swap in swaps {
            let mut i = swap.0.min(swap.1);
            let mut j = swap.1.max(swap.0);
            let max = j;

            while i < max && j < values.len() {
                values.swap(i, j);
                i += 1;
                j += 1;
            }
        }

        format!("{}", values[check])
    }
}

type Number = usize;

#[derive(Debug, Clone)]
struct Swap2(usize, usize);

#[derive(Debug)]
struct Swap3(usize, usize, usize);

impl FromStr for Swap2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s
            .split_once('-')
            .unwrap()
            .map(|x| x.parse::<Number>().unwrap() - 1);

        Ok(Self(l, r))
    }
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Swap2>, Number) {
    let mut sections = input.split("\n\n");
    let values = sections
        .next()
        .unwrap()
        .lines()
        .map(|v| v.parse().unwrap())
        .collect();
    let swaps = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let check = sections.next().unwrap().parse::<usize>().unwrap() - 1;

    (values, swaps, check)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "159
527
827
596
296
413
45
796
853
778

4-8
5-8
10-1
6-5
2-1
6-5
8-7
3-6
7-8
2-10
6-4
8-10
1-9
3-6
7-10

10";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem7;
        assert_eq!(problem.part_one(INPUT), "45");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem7;
        assert_eq!(problem.part_two(INPUT), "796");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem7;
        assert_eq!(problem.part_three(INPUT), "827");
    }
}
