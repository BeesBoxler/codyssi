use std::{collections::HashMap, str::FromStr};

pub fn run(input: &str) {
    println!("Day X:");
    println!("  Part 1: {}", part_one(input));
    println!("  Part 2: {}", part_two(input));
    println!("  Part 3: {}", part_three(input));
}

type Number = i32;
type Ledger = HashMap<String, Number>;

#[derive(Debug)]
struct Instruction {
    from: String,
    to: String,
    amount: Number,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[5..];
        let (from, s) = s.split_once(" TO ").unwrap();
        let (to, amount) = s.split_once(" AMT ").unwrap();

        Ok(Self {
            from: from.into(),
            to: to.into(),
            amount: amount.parse::<Number>().unwrap(),
        })
    }
}

fn parse_input(input: &str) -> (Ledger, Vec<Instruction>) {
    let (ledger_input, instructions) = input.split_once("\n\n").unwrap();
    let mut ledger = HashMap::new();
    for line in ledger_input.lines() {
        let (person, amount) = line.split_once(" HAS ").unwrap();

        ledger.insert(person.into(), amount.parse::<Number>().unwrap());
    }

    let instructions = instructions
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ledger, instructions)
}

fn part_one(input: &str) -> Number {
    let (mut ledger, instructions) = parse_input(input);

    for Instruction { from, to, amount } in instructions {
        {
            let from = ledger.get_mut(&from).unwrap();
            *from -= amount;
        }
        {
            let to = ledger.get_mut(&to).unwrap();
            *to += amount;
        }
    }

    let mut values = ledger.values().collect::<Vec<_>>();
    values.sort();
    values.reverse();

    values[..3].iter().map(|v| **v).sum()
}

fn part_two(input: &str) -> Number {
    let (mut ledger, instructions) = parse_input(input);

    for Instruction {
        from,
        to,
        mut amount,
    } in instructions
    {
        {
            let from = ledger.get_mut(&from).unwrap();
            amount = amount.min(*from);
            *from -= amount;
        }
        {
            let to = ledger.get_mut(&to).unwrap();
            *to += amount;
        }
    }

    let mut values = ledger.values().collect::<Vec<_>>();
    values.sort();
    values.reverse();

    values[..3].iter().map(|v| **v).sum()
}

fn part_three(_input: &str) -> Number {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Alpha HAS 131
Bravo HAS 804
Charlie HAS 348
Delta HAS 187
Echo HAS 649
Foxtrot HAS 739

FROM Echo TO Foxtrot AMT 328
FROM Charlie TO Bravo AMT 150
FROM Charlie TO Delta AMT 255
FROM Alpha TO Delta AMT 431
FROM Foxtrot TO Alpha AMT 230
FROM Echo TO Foxtrot AMT 359
FROM Echo TO Alpha AMT 269
FROM Delta TO Foxtrot AMT 430
FROM Bravo TO Echo AMT 455
FROM Charlie TO Delta AMT 302";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 2870);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 2542);
    }

    #[test]
    fn part_three_returns_correct_output() {
        assert_eq!(part_three(INPUT), 0);
    }
}
