use crate::problem::Problem;

pub struct Problem4;

impl Problem for Problem4 {
    fn get_input(&self) -> String {
        "input4".into()
    }

    fn get_title(&self) -> String {
        "Problem 4: Aeolian Transmissions".into()
    }

    fn part_one(&self, input: &str) -> String {
        format!(
            "{}",
            parse_input(input)
                .iter()
                .map(|line| line.iter().sum::<Number>())
                .sum::<Number>()
        )
    }

    fn part_two(&self, input: &str) -> String {
        format!(
            "{}",
            parse_input(input)
                .iter()
                .map(|line| {
                    let len = line.len();
                    let buffer = len / 10;
                    let removed = len - 2 * buffer;

                    line[0..buffer].iter().sum::<Number>()
                        + number_to_value(removed)
                        + line[len - buffer..len].iter().sum::<Number>()
                })
                .sum::<Number>()
        )
    }

    fn part_three(&self, input: &str) -> String {
        let mut total = 0;
        let mut curr = 0;
        let mut count = 0;

        for line in parse_input(input) {
            for byte in line {
                if byte == curr {
                    count += 1;
                } else {
                    total += curr + number_to_value(count);
                    curr = byte;
                    count = 1;
                }
            }

            total += curr + number_to_value(count);
            curr = 0;
            count = 0;
        }

        format!("{}", total)
    }
}

const ALPHABET_OFFSET: u8 = 64;
const NUMBER_OFFSET: u8 = 48;

type Number = usize;

fn parse_input(input: &str) -> Vec<Vec<Number>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|byte| (byte - ALPHABET_OFFSET) as Number)
                .collect()
        })
        .collect()
}

fn number_to_value(number: Number) -> Number {
    number
        .to_string()
        .bytes()
        .map(|x| (x - NUMBER_OFFSET) as Number)
        .sum::<Number>()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "NNBUSSSSSDSSZZZZMMMMMMMM
PWAAASYBRRREEEEEEE
FBBOFFFKDDDDDDDDD
VJAANCPKKLZSSSSSSSSS
NNNNNNBBVVVVVVVVV";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem4;
        assert_eq!(problem.part_one(INPUT), "1247");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem4;
        assert_eq!(problem.part_two(INPUT), "219");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem4;
        assert_eq!(problem.part_three(INPUT), "539");
    }
}
