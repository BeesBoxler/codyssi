pub fn run(input: &str) {
    println!("Day X:");
    println!("  Part 1: {}", part_one(input));
    println!("  Part 2: {}", part_two(input));
    println!("  Part 3: {}", part_three(input));
}

fn part_one(_input: &str) -> i64 {
    0
}

fn part_two(input: &str) -> i64 {
    0
}

fn part_three(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 0);
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
