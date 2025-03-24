pub fn run(input: &str) {
    println!("Day 1:");
    println!("  Part 1: {}", part_one(input));
    println!("  Part 2: {}", part_two(input));
    println!("  Part 3: {}", part_three(input));
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<char>) {
    let mut radians = vec![];
    let mut instructions = vec![];

    input.lines().for_each(|line| {
        if let Ok(value) = line.parse() {
            radians.push(value);
        } else {
            instructions = line.chars().collect();
        }
    });
    

    (radians, instructions)
}

fn part_one(input: &str) -> i64 {
    let (mut radians, mut instructions) = parse_input(input);
    radians.reverse();
    instructions.reverse();

    let mut total = radians.pop().unwrap();

    let mut pairs = radians.iter().zip(instructions.iter()).rev();

    while let Some((radian, instruction)) = pairs.next() {
        match instruction {
            '+' => total += radian,
            '-' => total -= radian,
            _ => panic!("Not a valid instruction"),
        }
    }

    total
}

fn part_two(_input: &str) -> usize {
    0
}

fn part_three(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "8
1
5
5
7
6
5
4
3
1
-++-++-++";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 21);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 0);
    }

    #[test]
    fn part_three_returns_correct_output() {
        assert_eq!(part_three(INPUT), 0);
    }
}