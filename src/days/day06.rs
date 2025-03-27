pub fn run(input: &str) {
    println!("Day 6:");
    println!("  Part 1: {}", part_one(input));
    println!("  Part 2: {}", part_two(input));
    println!("  Part 3: {}", part_three(input));
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

fn part_one(input: &str) -> Number {
    parse_input(input).len()
}

fn part_two(input: &str) -> Number {
    parse_input(input)
        .iter()
        .map(|c| *c as Number)
        .sum::<Number>()
}

fn part_three(input: &str) -> Number {
    let mut prev_value = 0;

    input.bytes().fold(0, |acc, x| {
        prev_value = match x {
            97..=122 => x - 96,
            65..=90 => x - 38,
            _ => (((prev_value as i32) * 2 - 5 + 52) % 52) as u8,
        };

        acc + (prev_value) as usize
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "t#UD$%%DVd*L?^p?S$^@#@@$pF$?xYJ$LLv$@%EXO&$*iSFZuT!^VMHy#zKISHaBj?e*#&yRVdemc#?&#Q%j&ev*#YWRi@?mNQ@eK";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 59);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 1742);
    }

    #[test]
    fn part_three_returns_correct_output() {
        assert_eq!(part_three(INPUT), 2708);
    }
}
