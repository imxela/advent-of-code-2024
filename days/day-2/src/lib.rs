#![allow(clippy::cargo_common_metadata)]
#![feature(test, iter_map_windows)]

extern crate test;

use std::{cmp::Ordering, collections::BTreeMap};

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

#[must_use]
pub fn day_2_part_1(input: &str) -> usize {
    let input = parse_values(input);

    input
        .iter()
        .filter(|levels| match levels[0].cmp(&levels[1]) {
            Ordering::Greater => validate_decrementing(levels),
            Ordering::Less => validate_incrementing(levels),
            Ordering::Equal => false,
        })
        .count()
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

#[must_use]
pub fn day_2_part_2(input: &str) -> usize {
    let input = parse_values(input);

    input
        .into_iter()
        .filter(|levels| validate_levels(levels, true))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_part_1_example() {
        assert!(day_2_part_1(&common::read_example_input(1)) == 2);
    }

    #[test]
    fn test_day_2_part_1() {
        assert!(day_2_part_1(&common::read_input()) == 502);
    }

    #[bench]
    fn bench_day_2_part_1(bencher: &mut test::Bencher) {
        let input = common::read_input();
        bencher.iter(|| day_2_part_1(&input));
    }

    #[test]
    fn test_day_2_part_2_example() {
        let result = day_2_part_2(&common::read_example_input(2));
        assert!(result == 4);
    }

    #[test]
    fn test_day_2_part_2() {
        assert!(day_2_part_2(&common::read_input()) == 544);
    }

    #[bench]
    fn bench_day_2_part_2(bencher: &mut test::Bencher) {
        let input = common::read_input();
        bencher.iter(|| day_2_part_2(&input));
    }
}
