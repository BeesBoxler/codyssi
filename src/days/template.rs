use std::str::FromStr;

use crate::problem::Problem;

pub struct ProblemX;

impl Problem for ProblemX {
    fn get_input(&self) -> String {
        "inputX".into()
    }

    fn get_title(&self) -> String {
        "Problem X: __________".into()
    }

    fn part_one(&self, _input: &str) -> String {
        "".into()
    }

    fn part_two(&self, _input: &str) -> String {
        "".into()
    }

    fn part_three(&self, _input: &str) -> String {
        "".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = ProblemX;
        assert_eq!(problem.part_one(INPUT), "");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = ProblemX;
        assert_eq!(problem.part_two(INPUT), "");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = ProblemX;
        assert_eq!(problem.part_three(INPUT), "");
    }
}
