use crate::problem::Problem;

pub struct Problem6;

impl Problem for Problem6 {
    fn get_input(&self) -> String {
        "input6".into()
    }

    fn get_title(&self) -> String {
        "Problem 6: Lotus Scramble".into()
    }

    fn part_one(&self, input: &str) -> String {
        format!("{}", parse_input(input).len())
    }

    fn part_two(&self, input: &str) -> String {
        format!(
            "{}",
            parse_input(input)
                .iter()
                .map(|c| *c as Number)
                .sum::<Number>()
        )
    }

    fn part_three(&self, input: &str) -> String {
        let mut prev_value = 0;

        format!(
            "{}",
            input.bytes().fold(0, |acc, x| {
                prev_value = match x {
                    97..=122 => x - 96,
                    65..=90 => x - 38,
                    _ => (((prev_value as i32) * 2 - 5 + 52) % 52) as u8,
                };

                acc + (prev_value) as usize
            })
        )
    }
}

type Number = usize;

fn parse_input(input: &str) -> Vec<u8> {
    input
        .bytes()
        .filter_map(|c| {
            Some(match c {
                97..=122 => c - 96,
                65..=90 => c - 38,
                _ => return None,
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "t#UD$%%DVd*L?^p?S$^@#@@$pF$?xYJ$LLv$@%EXO&$*iSFZuT!^VMHy#zKISHaBj?e*#&yRVdemc#?&#Q%j&ev*#YWRi@?mNQ@eK";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem6;
        assert_eq!(problem.part_one(INPUT), "59");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem6;
        assert_eq!(problem.part_two(INPUT), "1742");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem6;
        assert_eq!(problem.part_three(INPUT), "2708");
    }
}
