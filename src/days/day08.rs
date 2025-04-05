use crate::problem::Problem;

pub struct Problem8;

impl Problem for Problem8 {
    fn get_input(&self) -> String {
        "input8".into()
    }

    fn get_title(&self) -> String {
        "Problem 8: Risky Shortcut".into()
    }

    fn part_one(&self, input: &str) -> String {
        format!("{}", input.chars().filter(|c| c.is_alphabetic()).count())
    }

    fn part_two(&self, input: &str) -> String {
        let lines = input
            .lines()
            .map(|line| {
                line.chars()
                    .fold(Vec::new(), |mut acc, c| {
                        if acc.is_empty() {
                            acc.push(c);
                        } else {
                            let prev = acc.last().unwrap();
                            if !prev.is_numeric() && c.is_numeric()
                                || prev.is_numeric() && !c.is_numeric()
                            {
                                acc.pop();
                            } else {
                                acc.push(c);
                            }
                        }

                        acc
                    })
                    .len()
            })
            .collect::<Vec<_>>();

        format!("{}", lines.iter().sum::<usize>())
    }

        fn part_three(&self, input: &str) -> String {
        let lines = input
            .lines()
            .map(|line| {
                line.chars()
                    .fold(Vec::new(), |mut acc, c| {
                        if acc.is_empty() {
                            acc.push(c);
                        } else {
                            let prev = acc.last().unwrap();
                            if prev.is_alphabetic() && c.is_numeric()
                                || prev.is_numeric() && c.is_alphabetic()
                            {
                                acc.pop();
                            } else {
                                acc.push(c);
                            }
                        }

                        acc
                    })
                    .len()
            })
            .collect::<Vec<_>>();

        format!("{}", lines.iter().sum::<usize>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "tv8cmj0i2951190z5w44fe205k542l5818ds05ib425h9lj260ud38-l6a06
a586m0eeuqqvt5-k-8434hb27ytha3i75-lw23-0cj856l7zn8234a05eron";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem8;
        assert_eq!(problem.part_one(INPUT), "52");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem8;
        assert_eq!(problem.part_two(INPUT), "18");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem8;
        assert_eq!(problem.part_three(INPUT), "26");
    }
}
