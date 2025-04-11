use std::{cmp::Reverse, collections::{BTreeSet, HashMap, HashSet, VecDeque}};

use crate::problem::Problem;

pub struct Problem13;

impl Problem for Problem13 {
    fn get_input(&self) -> String {
        "input13".into()
    }

    fn get_title(&self) -> String {
        "Problem 13: Laestrygonian Guards".into()
    }

    fn part_one(&self, input: &str) -> String {
        let locations = parse_input(input);

        println!("{:?}", locations);

        "".into()
    }

    fn part_two(&self, _input: &str) -> String {
        "".into()
    }

    fn part_three(&self, _input: &str) -> String {
        "".into()
    }
}

fn search(map: HashMap<String, Node>, target: &str)-> Option(usize) {
    let origin = map.get("STT").unwrap();
    let mut queue = VecDeque::from([(0, origin)]);
    let mut visited = HashSet::new();
    let mut distances = HashMap::<String, usize>::new();

    while let Some((dist, node)) = queue.pop_front() {
        if node.name == target { return Some(dist)}

        if visited.insert(&node.name) {
            for dest in &node.destinations {
                let dist = dist + dest.1;
                if !visited.contains(dest.0) {
                    let entry = distances.entry(dest.0.clone());
                }
            }
        }
    }

    None
}

fn parse_input(input: &str) -> HashMap<String, Node> {
    let mut result = HashMap::new();

    for line in input.lines() {
        let Some((name, other)) = line.split_once(" -> ") else {panic!()};
        let Some((dest_name, dist)) = other.split_once(" | ") else {panic!()};

        result.insert(dest_name.into(), Node::new(dest_name));
        let origin = result.entry(name.into()).or_insert(Node::new(name));

        origin.destinations.insert(dest_name.into(), dist.parse().unwrap());
    }

    result
}

#[derive(Debug)]
struct Node {
    pub name: String,
    pub destinations: HashMap<String, usize>,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            destinations: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "STT -> MFP | 5
AIB -> ZGK | 6
ZGK -> KVX | 20
STT -> AFG | 4
AFG -> ZGK | 16
MFP -> BDD | 13
BDD -> AIB | 5
AXU -> MFP | 4
CLB -> BLV | 20
AIB -> BDD | 13
BLV -> AXU | 17
AFG -> CLB | 2";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem13;
        assert_eq!(problem.part_one(INPUT), "36");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem13;
        assert_eq!(problem.part_two(INPUT), "");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem13;
        assert_eq!(problem.part_three(INPUT), "");
    }
}
