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

#[must_use]
pub fn day_1_part_1() -> usize {
    let input = common::read_input();
    let (mut left, mut right) = parse_values(&input);

    left.sort_unstable();
    right.sort_unstable();

    zip(left, right)
        .map(|(left, right)| {
            if left > right {
                left - right
            } else {
                right - left
            }
        })
        .sum()
}

#[must_use]
pub fn day_1_part_2() -> usize {
    let input = common::read_input();
    let (left, right) = parse_values(&input);

    let sum = left
        .iter()
        .map(|left_number| {
            let count = right
                .iter()
                .filter(|right_number| left_number == *right_number)
                .count();

            left_number * count
        })
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_part_1() {
        assert!(day_1_part_1() == 765_748);
    }

    #[bench]
    fn bench_day_1_part_1(bencher: &mut test::Bencher) {
        bencher.iter(day_1_part_1);
    }

    #[test]
    fn test_day_1_part_2() {
        assert!(day_1_part_2() == 27_732_508);
    }

    #[bench]
    fn bench_day_1_part_2(bencher: &mut test::Bencher) {
        bencher.iter(day_1_part_2);
    }
}
