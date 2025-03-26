pub fn run(input: &str) {
    println!("Day X:");
    println!("  Part 1: {}", part_one(input));
    println!("  Part 2: {}", part_two(input));
    println!("  Part 3: {}", part_three(input));
}

const ALPHABET_OFFSET: u8 = 64;
const NUMBER_OFFSET: u8 = 48;

type Number = usize;

fn parse_input(input: &str) -> Vec<Vec<Number>> {
    input.lines().map(|line| line.bytes().map(|byte| (byte - ALPHABET_OFFSET) as Number).collect()).collect()
} 

fn part_one(input: &str) -> Number {
    parse_input(input).iter().map(|line| line.iter().sum::<Number>()).sum()
}

fn part_two(input: &str) -> Number {
    parse_input(input).iter().map(|line| {
        let len = line.len();
        let buffer = len / 10;
        let removed = len - 2*buffer;

        line[0..buffer].iter().sum::<Number>()
        + removed.to_string().bytes().map(|x| (x - NUMBER_OFFSET) as Number).sum::<Number>()
        + line[len-buffer..len].iter().sum::<Number>()
    }).sum()
}

fn part_three(_input: &str) -> Number {
    0
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
        assert_eq!(part_one(INPUT), 1247);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 219);
    }

    #[test]
    fn part_three_returns_correct_output() {
        assert_eq!(part_three(INPUT), 0);
    }
}
