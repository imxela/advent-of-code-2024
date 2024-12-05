#![allow(clippy::cargo_common_metadata)]
#![feature(test, iter_map_windows)]

extern crate test;

use std::iter::zip;

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn parse_values(input_string: &str) -> (Vec<usize>, Vec<usize>) {
    let numbers = input_string
        .lines()
        .flat_map(|line| {
            line.split_whitespace().map_windows(|[left, right]| {
                (
                    str::parse::<usize>(left).expect("failed to parse input data to usize"),
                    str::parse::<usize>(right).expect("failed to parse input data to usize"),
                )
            })
        })
        .unzip();

    numbers
}

use common::{AdventSolver, AdventSolverMut};

struct Day1Part1Solver {
    data: (Vec<usize>, Vec<usize>),
}

impl AdventSolverMut<usize, (Vec<usize>, Vec<usize>)> for Day1Part1Solver {
    fn parse(input: &str) -> Self {
        Self {
            data: parse_values(input),
        }
    }

    fn solve(&mut self) -> common::AdventSolution<usize> {
        // This looks a bit awful...
        let &mut (ref mut left, ref mut right) = &mut self.data;

        left.sort_unstable();
        right.sort_unstable();

        let sum: usize = zip(left, right)
            .map(|(left, right)| {
                if left > right {
                    *left - *right
                } else {
                    *right - *left
                }
            })
            .sum();

        sum.into()
    }

    fn data(&self) -> &(Vec<usize>, Vec<usize>) {
        &self.data
    }
}

struct Day1Part2Solver {
    data: (Vec<usize>, Vec<usize>),
}

impl AdventSolver<usize, (Vec<usize>, Vec<usize>)> for Day1Part2Solver {
    fn parse(input: &str) -> Self {
        Self {
            data: parse_values(input),
        }
    }

    fn solve(&self) -> common::AdventSolution<usize> {
        let (left, right) = &self.data;

        let sum: usize = left
            .iter()
            .map(|left_number| {
                let count = right
                    .iter()
                    .filter(|right_number| left_number == *right_number)
                    .count();

                left_number * count
            })
            .sum();

        sum.into()
    }

    fn data(&self) -> &(Vec<usize>, Vec<usize>) {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_part_1_example() {
        let mut solver = Day1Part1Solver::parse(&common::read_example_input(1));
        let solution = solver.solve();
        assert!(solution.prove(&11));
    }

    #[test]
    fn test_day_1_part_1() {
        let mut solver = Day1Part1Solver::parse(&common::read_input());
        let solution = solver.solve();
        assert!(solution.prove(&765_748));
    }

    #[bench]
    fn bench_day_1_part_1(bencher: &mut test::Bencher) {
        let mut solver = Day1Part1Solver::parse(&common::read_input());
        bencher.iter(|| {
            let solution = solver.solve();
            assert!(solution.prove(&765_748));
        });
    }

    #[test]
    fn test_day_1_part_2_example() {
        let solver = Day1Part2Solver::parse(&common::read_example_input(2));
        let solution = solver.solve();
        assert!(solution.prove(&31));
    }

    #[test]
    fn test_day_1_part_2() {
        let solver = Day1Part2Solver::parse(&common::read_input());
        let solution = solver.solve();
        assert!(solution.prove(&27_732_508));
    }

    #[bench]
    fn bench_day_1_part_2(bencher: &mut test::Bencher) {
        let solver = Day1Part2Solver::parse(&common::read_input());
        bencher.iter(|| {
            let solution = solver.solve();
            assert!(solution.prove(&27_732_508));
        });
    }
}
