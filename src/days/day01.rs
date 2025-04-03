use crate::problem::Problem;

pub struct Problem1;

impl Problem for Problem1 {
    fn get_input(&self) -> String {
        "input1".into()
    }

    fn get_title(&self) -> String {
        "Problem 1: Compass Calibration".into()
    }

    fn part_one(&self, input: &str) -> String {
        let (mut radians, mut instructions) = parse_input(input);
        radians.reverse();
        instructions.reverse();

        let mut total = radians.pop().unwrap();

        let pairs = radians.iter().zip(instructions.iter()).rev();

        for (radian, instruction) in pairs {
            match instruction {
                '+' => total += radian,
                '-' => total -= radian,
                _ => panic!("Not a valid instruction"),
            }
        }

        format!("{total}")
    }

    fn part_two(&self, input: &str) -> String {
        let (mut radians, instructions) = parse_input(input);
        radians.reverse();

        let mut total = radians.pop().unwrap();

        let pairs = radians.iter().zip(instructions.iter()).rev();

        for (radian, instruction) in pairs {
            match instruction {
                '+' => total += radian,
                '-' => total -= radian,
                _ => panic!("Not a valid instruction"),
            }
        }

        format!("{total}")
    }

    fn part_three(&self, input: &str) -> String {
        let (mut radians, mut instructions) = parse_input(input);
        radians.reverse();
        instructions.reverse();

        let mut total = radians.pop().unwrap() * 10 + radians.pop().unwrap();
        radians.reverse();

        let pairs = radians
            .chunks(2)
            .map(|chunk| chunk[0] * 10 + chunk[1])
            .zip(instructions.iter());

        for (radian, instruction) in pairs {
            match instruction {
                '+' => total += radian,
                '-' => total -= radian,
                _ => panic!("Not a valid instruction"),
            }
        }

        format!("{total}")
    }
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
        let problem = Problem1;
        assert_eq!(problem.part_one(INPUT), "21");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem1;
        assert_eq!(problem.part_two(INPUT), "23");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem1;
        assert_eq!(problem.part_three(INPUT), "189");
    }
}
