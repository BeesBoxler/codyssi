use std::str::FromStr;

pub fn run(input: &str) {
    println!("Day 5:");
    println!("  Part 1: {}", part_one(input));
    println!("  Part 2: {}", part_two(input));
    println!("  Part 3: {}", part_three(input));
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

fn part_one(input: &str) -> Number {
    let curr = Point(0, 0);
    let distances = parse_input(input)
        .iter()
        .map(|point| point.manhattan(&curr))
        .collect::<Vec<_>>();

    distances.iter().max().unwrap() - distances.iter().min().unwrap()
}

fn part_two(input: &str) -> Number {
    let origin = Point(0, 0);
    let distances = parse_input(input);
    let min_from_origin = distances
        .iter()
        .min_by(|a, b| a.manhattan(&origin).cmp(&b.manhattan(&origin)))
        .unwrap();
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
}

fn part_three(_input: &str) -> Number {
    0
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
        assert_eq!(part_one(INPUT), 226);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 114);
    }

    #[test]
    fn part_three_returns_correct_output() {
        assert_eq!(part_three(INPUT), 0);
    }
}
