use crate::problem::Problem;
use std::{cmp::Ordering, str::FromStr};

pub struct Problem5;

impl Problem for Problem5 {
    fn get_input(&self) -> String {
        "input5".into()
    }

    fn get_title(&self) -> String {
        "Problem 5: Patron Islands".into()
    }

    fn part_one(&self, input: &str) -> String {
        let curr = Point(0, 0);
        let distances = parse_input(input)
            .iter()
            .map(|point| point.manhattan(&curr))
            .collect::<Vec<_>>();

        format!(
            "{}",
            distances.iter().max().unwrap() - distances.iter().min().unwrap()
        )
    }

    fn part_two(&self, input: &str) -> String {
        let origin = Point(0, 0);
        let distances = parse_input(input);
        let min_from_origin = distances
            .iter()
            .min_by(|a, b| a.manhattan(&origin).cmp(&b.manhattan(&origin)))
            .unwrap();

        format!(
            "{}",
            distances
                .iter()
                .map(|point| {
                    if point == min_from_origin {
                        Number::MAX
                    } else {
                        point.manhattan(min_from_origin)
                    }
                })
                .min()
                .unwrap()
        )
    }

    fn part_three(&self, input: &str) -> String {
        let sort_by_distance_than_value = |a: &Point, b: &Point, previous: &Point| -> Ordering {
            let cmp = &a.manhattan(previous).cmp(&b.manhattan(previous));

            if cmp != &Ordering::Equal {
                *cmp
            } else if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        };

        let mut total = 0;
        let mut curr = Point(0, 0);
        let mut points = parse_input(input);

        while !points.is_empty() {
            points.sort_by(|a, b| sort_by_distance_than_value(a, b, &curr));
            points.reverse();
            let new = points.pop().unwrap();
            total += new.manhattan(&curr);
            curr = new;
        }

        format!("{}", total)
    }
}

type Number = i32;

#[derive(Debug, PartialEq, Eq)]
struct Point(Number, Number);

impl Point {
    fn manhattan(&self, other: &Self) -> Number {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(", ").unwrap();
        let l = l.replace('(', "");
        let r = r.replace(')', "");

        Ok(Point(l.parse().unwrap(), r.parse().unwrap()))
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "(-16, -191)
(92, 186)
(157, -75)
(39, -132)
(-42, 139)
(-74, -150)
(200, 197)
(-106, 105)";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem5;
        assert_eq!(problem.part_one(INPUT), "226");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem5;
        assert_eq!(problem.part_two(INPUT), "114");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem5;
        assert_eq!(problem.part_three(INPUT), "1384");
    }
}
