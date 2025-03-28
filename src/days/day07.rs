use std::str::FromStr;
use tuple_map::TupleMap2;

pub fn run(input: &str) {
    println!("Day 7:");
    println!("  Part 1: {}", part_one(input));
    println!("  Part 2: {}", part_two(input));
    println!("  Part 3: {}", part_three(input));
}

type Number = usize;

#[derive(Debug)]
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

fn create_swap2_vec(input: &str) -> Vec<Swap2> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn create_swap3_vec(input: &str) -> Vec<Swap3> {
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.push(lines[0]);
    lines
        .windows(2)
        .map(|w| {
            let ints = w
                .iter()
                .flat_map(|line| line.split('-').map(|c| c.parse::<usize>().unwrap() - 1))
                .collect::<Vec<_>>();

            Swap3(ints[0], ints[1], ints[2])
        })
        .collect()
}

fn parse_input2(input: &str) -> (Vec<Number>, Vec<Swap2>, Number) {
    let mut sections = input.split("\n\n");
    let values = sections
        .next()
        .unwrap()
        .lines()
        .map(|v| v.parse().unwrap())
        .collect();
    let swaps = create_swap2_vec(sections.next().unwrap());
    let check = sections.next().unwrap().parse().unwrap();

    (values, swaps, check)
}

fn parse_input3(input: &str) -> (Vec<Number>, Vec<Swap3>, Number) {
    let mut sections = input.split("\n\n");
    let values = sections
        .next()
        .unwrap()
        .lines()
        .map(|v| v.parse().unwrap())
        .collect();
    let swaps = create_swap3_vec(sections.next().unwrap());
    let check = sections.next().unwrap().parse().unwrap();

    (values, swaps, check)
}

fn part_one(input: &str) -> Number {
    let (mut values, swaps, check) = parse_input2(input);

    swaps.iter().for_each(|swap| values.swap(swap.0, swap.1));

    values[check - 1]
}

fn part_two(input: &str) -> Number {
    let (mut values, swaps, check) = parse_input3(input);

    swaps.iter().for_each(|swap| {
        let hold = values[swap.2];
        values[swap.2] = values[swap.1];
        values[swap.1] = values[swap.0];
        values[swap.0] = hold;
    });

    values[check - 1]
}

fn part_three(_input: &str) -> Number {
    0
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
        assert_eq!(part_one(INPUT), 45);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 796);
    }

    #[test]
    fn part_three_returns_correct_output() {
        assert_eq!(part_three(INPUT), 0);
    }
}
