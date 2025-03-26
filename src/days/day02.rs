use std::str::FromStr;

pub fn run(input: &str) {
    println!("Day X:");
    println!("  Part 1: {}", part_one(input));
    println!("  Part 2: {}", part_two(input));
    println!("  Part 3: {}", part_three(input));
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
            let value = words[words.len() -1].parse::<Number>().unwrap();
            let instruction = words[words.len() -2];

            match instruction {
                "ADD" => Box::new(move |x| x + value),
                "MULTIPLY" =>  Box::new(move |x| x * value),
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
    let (instructions, values) =input.split_once("\n\n").unwrap();
    
    (
        instructions.parse().unwrap(),
        values.lines().map(|v| v.parse().unwrap()).collect()
    )

}

fn part_one(input: &str) -> Number {
    let (instructions, mut values) = parse(input);
    values.sort();

    (instructions.a)((instructions.b)((instructions.c)(values[values.len()/2])))
}

fn part_two(_input: &str) -> i64 {
    0
}

fn part_three(_input: &str) -> i64 {
    0
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
        assert_eq!(part_one(INPUT), 9130674516975);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 1000986169836015);
    }

    #[test]
    fn part_three_returns_correct_output() {
        assert_eq!(part_three(INPUT), 0);
    }
}
