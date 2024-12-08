#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::collapsible_if)] // For clarity
#![feature(test)]

extern crate test;

use common::AdventSolver;

struct Day4Part1Solver {
    data: String,
}

impl AdventSolver<usize, String> for Day4Part1Solver {
    fn parse(input: &str) -> Self {
        Day4Part1Solver {
            data: input.to_owned(),
        }
    }

    fn solve(&self) -> common::AdventSolution<usize> {
        static WORD: [char; 4] = ['X', 'M', 'A', 'S'];

        let lines: Vec<Vec<char>> = self
            .data
            .lines()
            .map(|elem| elem.chars().collect::<Vec<char>>())
            .collect();

        let mut count = 0;

        for line_idx in 0..lines.len() {
            let current_line: &Vec<char> = &lines[line_idx];

            for char_idx in 0..current_line.len() {
                count += count_matches(&WORD, &lines, line_idx, char_idx);
            }
        }

        count.into()
    }

    fn data(&self) -> &String {
        &self.data
    }
}

struct Day4Part2Solver {
    data: String,
}

impl AdventSolver<usize, String> for Day4Part2Solver {
    fn parse(input: &str) -> Self {
        Day4Part2Solver {
            data: input.to_owned(),
        }
    }

    fn solve(&self) -> common::AdventSolution<usize> {
        static WORD: [char; 3] = ['M', 'A', 'S'];

        let lines: Vec<Vec<char>> = self
            .data
            .lines()
            .map(|elem| elem.chars().collect::<Vec<char>>())
            .collect();

        let mut count = 0;

        // North, East, South and West are not valid directions for a cross.
        // We cherry-pick the valid ones.
        let valid_directions = [
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthWest,
            Direction::SouthEast,
        ];

        // Skip first line since we're matching the 'A' om the center!
        for line_idx in 1..lines.len() {
            let current_line: &Vec<char> = &lines[line_idx];

            // Skip first char since we're matching the 'A' in the center!
            for (char_idx, char) in current_line.iter().enumerate().skip(1) {
                if char == &'A' {
                    if match_crossed_word(&lines, line_idx, char_idx, &WORD, &valid_directions, 0) {
                        count += 1;
                    }
                }
            }
        }

        count.into()
    }

    fn data(&self) -> &String {
        &self.data
    }
}

// Matches the word MAX in a cross starting at the 'A' in the center.
#[must_use]
fn match_crossed_word(
    lines: &Vec<Vec<char>>,
    line_idx: usize,
    char_idx: usize,
    word: &[char],
    valid_directions: &[Direction],
    match_count: usize,
) -> bool {
    // Check all valid direction in clockwise order
    for direction in valid_directions {
        // We start one step in the opposite of the direction
        // since we match the character 'A' in the center of the cross.
        // E.g., if direction is SouthEast:
        // M . . <-- 2. And go to the 'M' here so we can match the entire word
        // . A . <-- 1. We start here
        // . . S
        let (line_delta, char_delta) = direction.inverse().traversal_delta();

        // The word can't fit if there is underflow
        let Some(line_start) = line_idx.checked_add_signed(line_delta) else {
            continue;
        };

        // –– // ––
        let Some(char_start) = char_idx.checked_add_signed(char_delta) else {
            continue;
        };

        // `line_start` and `char_start` represent the coordinate to start
        // checking for a word match, so we can feed them into the function
        // from part 1 along with the direction to check in.
        if !match_word_in_direction(lines, line_start, char_start, word, 0, *direction) {
            // Not a match, we can keep looking in the next direction.
            continue;
        }

        // We matched one part of the cross in the previous recursion
        // so we can safely assume we found the entire cross now and return.
        if match_count > 0 {
            return true;
        }

        // If we get here it means we have an initial match of the word MAS
        // in some diagonal direction. However, we still need it to match
        // a 2nd non-opposite diagonal.

        // The next match has to be in a flipped position, and possibly opposite.
        // Both this:
        // . . M
        // . A .
        // S . .
        // and this:
        // . . S
        // . A .
        // M . .
        // are possible candidates for a full match, so we need to try both.

        // When we try flipping the direction:
        // *
        //   o
        //     o
        // Becomes:
        //     *
        //   o
        // o

        // And if we then try inversing that direction:
        //     *
        //   o
        // o
        // Becomes:
        //     o
        //   o
        // *

        // Maybe not very efficient but oh well
        let valid_directions = valid_directions
            .iter()
            .filter(|elem| *elem != direction)
            .copied()
            .collect::<Vec<Direction>>();

        // If we find a 2nd match, we have a full cross -- return true.
        if match_crossed_word(
            lines,
            line_idx,
            char_idx,
            word,
            &valid_directions,
            match_count + 1,
        ) {
            return true;
        }
    }

    // If all directions have been checked and none gave a match we return false.
    false
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl Direction {
    pub fn iter() -> std::slice::Iter<'static, Direction> {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ]
        .iter()
    }

    /// The change in line index and character index respectively
    /// required for a treversal in the `self` direction.
    pub fn traversal_delta(self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
            Direction::NorthEast => (-1, 1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (1, -1),
            Direction::NorthWest => (-1, -1),
        }
    }

    //
    pub fn inverse(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::SouthEast,
            Direction::SouthWest => Direction::NorthEast,
            Direction::SouthEast => Direction::NorthWest,
        }
    }

    pub fn flip_around_north(self) -> Direction {
        match self {
            Direction::North => Direction::North,
            Direction::South => Direction::South,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::NorthWest,
            Direction::NorthWest => Direction::NorthEast,
            Direction::SouthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::SouthEast,
        }
    }
}

fn count_matches(word: &[char], lines: &Vec<Vec<char>>, line_idx: usize, char_idx: usize) -> usize {
    let mut count = 0;

    for direction in Direction::iter() {
        if match_word_in_direction(lines, line_idx, char_idx, word, 0, *direction) {
            count += 1;
        }
    }

    count
}

fn match_word_in_direction(
    lines: &Vec<Vec<char>>,
    line_idx: usize,
    char_idx: usize,

    word: &[char],

    // Number of chars matched the word in the previous recursions
    match_word_idx: usize,

    direction: Direction,
) -> bool {
    // Ensure a line exists at next_line_idx and a char exists at char_idx
    if line_idx >= lines.len() || char_idx >= lines[line_idx].len() {
        return false;
    }

    // Ensure the character matches the next one in the word sequence
    if lines[line_idx][char_idx] != word[match_word_idx] {
        return false;
    }

    let next_match_word_idx = match_word_idx + 1;

    // If out of bounds of the word means we have matched the complete word
    if next_match_word_idx == word.len() {
        return true;
    }

    // Traverse for next recursion
    let (line_delta, char_delta) = direction.traversal_delta();

    let new_line_idx = TryInto::<isize>::try_into(line_idx).unwrap() + line_delta;
    let new_char_idx = TryInto::<isize>::try_into(char_idx).unwrap() + char_delta;

    // Ensure the indices are not out of lower bounds
    // If they are, it's definitely not a match
    if new_line_idx < 0 || new_char_idx < 0 {
        return false;
    }

    match_word_in_direction(
        lines,
        new_line_idx.try_into().unwrap(),
        new_char_idx.try_into().unwrap(),
        word,
        next_match_word_idx,
        direction,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_4_part_1_example() {
        // let input = common::read_example_input(1);
        let input = include_str!("../res/inputs/example_input_p1");
        let solver = Day4Part1Solver::parse(input);
        let solution = solver.solve();
        solution.prove(&18);
    }

    #[test]
    fn test_day_4_part_1() {
        let input = common::read_input();
        let solver = Day4Part1Solver::parse(&input);
        let solution = solver.solve();
        solution.prove(&2517);
    }

    #[bench]
    fn bench_day_4_part_1(bencher: &mut test::Bencher) {
        let input = common::read_input();
        let solver = Day4Part1Solver::parse(&input);
        bencher.iter(|| {
            let solution = solver.solve();
            solution.prove(&2517);
        });
    }

    #[test]
    fn test_day_4_part_2_example() {
        let input = include_str!("../res/inputs/example_input_p2");
        let solver = Day4Part2Solver::parse(input);
        let solution = solver.solve();
        solution.prove(&9);
    }

    #[test]
    fn test_day_4_part_2() {
        let input = common::read_input();
        let solver = Day4Part2Solver::parse(&input);
        let solution = solver.solve();
        solution.prove(&1960);
    }

    #[bench]
    fn bench_day_4_part_2(bencher: &mut test::Bencher) {
        let input = common::read_input();
        let solver = Day4Part2Solver::parse(&input);
        bencher.iter(|| {
            let solution = solver.solve();
            solution.prove(&1960);
        });
    }
}
