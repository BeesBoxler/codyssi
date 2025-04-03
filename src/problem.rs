pub trait Problem {
    fn get_input(&self) -> String;
    fn get_title(&self) -> String;
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
    fn part_three(&self, input: &str) -> String;
}
