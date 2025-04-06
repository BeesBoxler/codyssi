use crate::problem::Problem;
use std::{cmp::Reverse, collections::{binary_heap::BinaryHeap, HashMap, HashSet}};

pub struct Problem10;

impl Problem for Problem10 {
    fn get_input(&self) -> String {
        "input10".into()
    }

    fn get_title(&self) -> String {
        "Problem 10: Cyclops Chaos".into()
    }

    fn part_one(&self, input: &str) -> String {
        let grid = parse_input(input);

        format!(
            "{}",
            grid.iter()
                .map(|row| row.iter().sum::<usize>())
                .min()
                .unwrap()
                .min(
                    (0..grid[0].len())
                        .map(|i| (0..grid.len()).map(|j| grid[j][i]).sum::<usize>())
                        .min()
                        .unwrap()
                )
        )
    }

    fn part_two(&self, input: &str) -> String {
        let grid = &parse_input(input);
        format!("{}", dijkstra(grid, (14,14)).unwrap())
    }

    fn part_three(&self, input: &str) -> String {
        let grid = &parse_input(input);
        format!("{}", dijkstra(grid, (grid.len()-1,grid[0].len()-1 )).unwrap())
    }
}

type Grid = Vec<Vec<usize>>;


fn dijkstra(grid: &Grid, target: (usize, usize)) -> Option<usize> {
    let mut queue = BinaryHeap::from([(Reverse(grid[0][0]), (0usize, 0usize))]);
    let mut visited = HashSet::new();
    let mut lengths = HashMap::<(usize, usize), usize>::new();

    while let Some((Reverse(length), pos)) = queue.pop() {
        if pos == target { return Some(length)}

        if visited.insert(pos) {
            for dir in [(1,0), (0,1)] {
                let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len() { continue}
                
                let length = length + grid[next_pos.0][next_pos.1];
                if !visited.contains(&next_pos) {
                    let entry = lengths.entry(pos).or_insert(usize::MAX);
                    if length < *entry {
                        *entry = length;
                    }
                }

                queue.push((Reverse(length), next_pos));
            }
        }
    }

    None
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3 3 1 7 8 4 1 3 1 7 7 6 7 8 7 8 2 7 7 1
9 9 7 6 3 6 9 4 9 2 6 4 5 7 3 9 3 7 5 6
8 9 7 6 7 7 3 2 2 7 8 9 7 1 5 3 1 2 4 4
9 2 8 2 3 5 9 2 6 5 7 8 1 6 7 3 6 7 9 6
4 1 7 5 2 2 7 6 8 7 2 3 9 2 2 1 6 2 7 5
2 9 1 2 9 9 1 2 2 9 3 7 4 5 3 3 7 1 9 4
9 9 5 2 6 6 2 3 1 8 3 3 3 6 7 9 8 3 1 5
8 4 8 7 2 1 7 9 8 7 3 7 9 1 8 5 2 5 2 8
6 8 9 6 6 4 2 2 7 7 7 8 1 2 6 2 6 1 6 7
3 8 8 6 9 9 2 7 8 5 4 1 8 8 5 8 3 5 6 6
2 8 7 2 6 8 4 7 1 7 6 8 9 4 3 1 2 8 9 8
6 2 9 7 7 2 8 7 9 5 6 6 8 2 8 4 4 8 2 2
3 1 2 8 8 4 6 8 9 1 4 3 9 1 4 2 2 1 5 4
5 2 6 7 2 7 3 9 2 1 7 6 1 2 4 2 1 1 5 9
3 6 8 9 4 4 7 7 3 3 4 8 6 1 2 9 7 2 9 6
9 4 5 5 7 4 1 7 7 1 3 2 3 8 1 7 6 3 1 9
5 3 8 3 1 1 5 3 1 5 9 2 3 6 6 4 4 8 5 3
6 3 8 2 9 7 3 6 4 3 2 8 6 9 8 1 2 7 1 5
4 1 2 4 8 7 7 1 8 7 4 4 5 7 2 3 3 8 3 3
1 5 7 3 3 5 1 5 4 1 1 1 9 2 1 4 6 5 6 3";

    #[test]
    fn part_one_returns_correct_output() {
        let problem = Problem10;
        assert_eq!(problem.part_one(INPUT), "73");
    }

    #[test]
    fn part_two_returns_correct_output() {
        let problem = Problem10;
        assert_eq!(problem.part_two(INPUT), "94");
    }

    #[test]
    fn part_three_returns_correct_output() {
        let problem = Problem10;
        assert_eq!(problem.part_three(INPUT), "120");
    }
}
