use crate::problem::Problem;

pub struct Problem10;

impl Problem for Problem10 {
    fn get_input(&self) -> String {
        "input10".into()
    }

    fn get_title(&self) -> String {
        "Problem 10: Cyclops Chaos".into()
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
        let problem = Problem10;
        assert_eq!(problem.part_one(INPUT), "");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem10;
        assert_eq!(problem.part_two(INPUT), "");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem10;
        assert_eq!(problem.part_three(INPUT), "");
    }
}
