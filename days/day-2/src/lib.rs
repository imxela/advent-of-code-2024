#![allow(clippy::cargo_common_metadata)]
#![feature(test, iter_map_windows)]

extern crate test;

use std::{cmp::Ordering, collections::BTreeMap};

use common::{AdventSolver, AdventSolverMut};

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn parse_values(input_string: &str) -> Vec<Vec<usize>> {
    let numbers = input_string
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| str::parse::<usize>(num).expect("failed to parse usize"))
                .collect()
        })
        .collect();

    numbers
}

fn validate_incrementing(levels: &[usize]) -> bool {
    levels.windows(2).all(|window| {
        let current = window[0];
        let next = window[1];

        let diff = current.abs_diff(next);

        current < next && diff > 0 && diff < 4
    })
}

fn validate_decrementing(levels: &[usize]) -> bool {
    levels.windows(2).all(|window| {
        let current = window[0];
        let next = window[1];

        let diff = current.abs_diff(next);

        current > next && diff > 0 && diff < 4
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SequenceType {
    Incrementing,
    Decrementing,
    Equal,
}

// Finds out if the levels are more frequently incrementing, decrementing or equal.
fn find_sequence_type(levels: &[usize]) -> SequenceType {
    let mut counts = BTreeMap::new();
    for sequence in levels
        .windows(2)
        .map(|window| match window[0].cmp(&window[1]) {
            Ordering::Less => SequenceType::Incrementing,
            Ordering::Equal => SequenceType::Equal,
            Ordering::Greater => SequenceType::Decrementing,
        })
    {
        *counts.entry(sequence).or_insert(0) += 1;
    }

    let (most_common, _) = counts.into_iter().max_by_key(|&(_, count)| count).unwrap();

    most_common
}

fn validate_levels(levels: &[usize], filter: bool) -> bool {
    let sequence_type = find_sequence_type(levels);

    for idx in 0..(levels.len() - 1) {
        let diff = levels[idx].abs_diff(levels[idx + 1]);

        let result = diff > 0
            && diff < 4
            && match sequence_type {
                SequenceType::Incrementing => levels[idx] < levels[idx + 1],
                SequenceType::Decrementing => levels[idx] > levels[idx + 1],
                SequenceType::Equal => return false,
            };

        if !result && filter {
            // Try without current index, and then without next index
            for i in 0..=2 {
                let filtered_levels =
                    [&levels[0..idx + i], &levels[idx + i + 1..levels.len()]].concat();

                if validate_levels(&filtered_levels, false) {
                    return true;
                }
            }

            return false;
        } else if !result && !filter {
            return false;
        }
    }

    true
}

struct Day2Part1Solver {
    data: Vec<Vec<usize>>,
}

impl AdventSolver<usize, Vec<Vec<usize>>> for Day2Part1Solver {
    fn parse(input: &str) -> Self {
        Self {
            data: parse_values(input),
        }
    }

    fn solve(&self) -> common::AdventSolution<usize> {
        self.data
            .iter()
            .filter(|levels| match levels[0].cmp(&levels[1]) {
                Ordering::Greater => validate_decrementing(levels),
                Ordering::Less => validate_incrementing(levels),
                Ordering::Equal => false,
            })
            .count()
            .into()
    }

    fn data(&self) -> &Vec<Vec<usize>> {
        &self.data
    }
}

struct Day2Part2Solver {
    data: Vec<Vec<usize>>,
}

impl AdventSolverMut<usize, Vec<Vec<usize>>> for Day2Part2Solver {
    fn parse(input: &str) -> Self {
        Self {
            data: parse_values(input),
        }
    }

    fn solve(&mut self) -> common::AdventSolution<usize> {
        self.data
            .iter_mut()
            .filter(|levels| validate_levels(levels, true))
            .count()
            .into()
    }

    fn data(&self) -> &Vec<Vec<usize>> {
        &self.data
    }
}

#[must_use]
pub fn day_2_part_2(input: &mut [Vec<usize>]) -> usize {
    input
        .iter_mut()
        .filter(|levels| validate_levels(levels, true))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_part_1_example() {
        let solver = Day2Part1Solver::parse(&common::read_example_input(1));
        let solution = solver.solve();
        assert!(solution.prove(&2));
    }

    #[test]
    fn test_day_2_part_1() {
        let solver = Day2Part1Solver::parse(&common::read_input());
        let solution = solver.solve();
        assert!(solution.prove(&502));
    }

    #[bench]
    fn bench_day_2_part_1(bencher: &mut test::Bencher) {
        let solver = Day2Part1Solver::parse(&common::read_input());
        bencher.iter(|| {
            let solution = solver.solve();
            assert!(solution.prove(&502));
        });
    }

    #[test]
    fn test_day_2_part_2_example() {
        let mut solver = Day2Part2Solver::parse(&common::read_example_input(2));
        let solution = solver.solve();
        assert!(solution.prove(&4));
    }

    #[test]
    fn test_day_2_part_2() {
        let mut solver = Day2Part2Solver::parse(&common::read_input());
        let solution = solver.solve();
        assert!(solution.prove(&544));
    }

    #[bench]
    fn bench_day_2_part_2(bencher: &mut test::Bencher) {
        let mut solver = Day2Part2Solver::parse(&common::read_input());
        bencher.iter(|| {
            let solution = solver.solve();
            assert!(solution.prove(&544));
        });
    }
}
