#![allow(clippy::cargo_common_metadata)]
#![feature(test, iter_next_chunk)]

extern crate test;

use common::AdventSolver;

struct Day3Part1Solver {
    data: Vec<char>,
}

struct Mul {
    left: usize,
    right: usize,
}

impl AdventSolver<usize, Vec<char>> for Day3Part1Solver {
    fn parse(input: &str) -> Self {
        Day3Part1Solver {
            data: input
                .as_bytes()
                .iter()
                .map(|byte| char::from(*byte))
                .collect::<Vec<char>>(),
        }
    }

    fn solve(&self) -> common::AdventSolution<usize> {
        let mut muls: Vec<Mul> = Vec::new();
        let mut idx = 0; // Current parser index
        let data = &self.data;

        loop {
            if data[idx] == 'm' {
                if let Some(mul) = parse_op_mul(&data[idx..]) {
                    muls.push(mul);
                }
            }

            idx += 1;
            if data.len() == idx {
                break;
            }
        }

        muls.iter()
            .map(|mul| mul.left * mul.right)
            .sum::<usize>()
            .into()
    }

    fn data(&self) -> &Vec<char> {
        &self.data
    }
}

struct Day3Part2Solver {
    data: Vec<char>,
}

impl AdventSolver<usize, Vec<char>> for Day3Part2Solver {
    fn parse(input: &str) -> Self {
        Day3Part2Solver {
            data: input
                .as_bytes()
                .iter()
                .map(|byte| char::from(*byte))
                .collect::<Vec<char>>(),
        }
    }

    fn solve(&self) -> common::AdventSolution<usize> {
        let mut muls: Vec<Mul> = Vec::new();
        let mut idx = 0; // Current parser index
        let mut ignore_muls = false;
        let data = &self.data;

        loop {
            if data[idx] == 'm' && !ignore_muls {
                if let Some(mul) = parse_op_mul(&data[idx..]) {
                    muls.push(mul);
                }
            }

            if data[idx] == 'd' {
                // Can be do or don't, check don't first and then redo with do
                if parse_op_dont(&data[idx..]) {
                    ignore_muls = true;
                } else if parse_op_do(&data[idx..]) {
                    ignore_muls = false;
                }
            }

            idx += 1;
            if data.len() == idx {
                break;
            }
        }

        muls.iter()
            .map(|mul| mul.left * mul.right)
            .sum::<usize>()
            .into()
    }

    fn data(&self) -> &Vec<char> {
        &self.data
    }
}

// I am so sorry for writing this :D
// Todo: Separate number parsing into its own function and call it
//       for left and right numbers
#[allow(clippy::collapsible_if)]
fn parse_op_mul(data: &[char]) -> Option<Mul> {
    if let Ok(chunk) = data.iter().next_chunk::<4>() {
        if chunk == [&'m', &'u', &'l', &'('] {
            let left_digits = data[4..]
                .iter()
                .take_while(|elem| elem.is_ascii_digit())
                .collect::<String>();

            if let Ok(left) = left_digits.parse::<usize>() {
                // Anything above is considered invalid
                if left < 999 {
                    if data.len() >= 4 + left_digits.len() {
                        if data[4 + left_digits.len()] == ',' {
                            let right_digits = data[4 + left_digits.len()..]
                                .iter()
                                .skip(1)
                                .take_while(|elem| elem.is_ascii_digit())
                                .collect::<String>();

                            if let Ok(right) = right_digits.parse::<usize>() {
                                if right < 999 {
                                    if let Some(elem) = data
                                        [4 + left_digits.len() + 1 + right_digits.len()..]
                                        .iter()
                                        .next()
                                        .take()
                                    {
                                        if elem == &')' {
                                            return Some(Mul { left, right });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn parse_op_dont(data: &[char]) -> bool {
    if let Ok(chunk) = data.iter().next_chunk::<7>() {
        chunk == [&'d', &'o', &'n', &'\'', &'t', &'(', &')']
    } else {
        false
    }
}

fn parse_op_do(data: &[char]) -> bool {
    if let Ok(chunk) = data.iter().next_chunk::<4>() {
        chunk == [&'d', &'o', &'(', &')']
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_part_1_example() {
        let input = common::read_example_input(1);
        let solver = Day3Part1Solver::parse(&input);
        let solution = solver.solve();
        solution.prove(&161);
    }

    #[test]
    fn test_day_3_part_1() {
        let input = common::read_input();
        let solver = Day3Part1Solver::parse(&input);
        let solution = solver.solve();
        solution.prove(&153_469_856);
    }

    #[bench]
    fn bench_day_3_part_1(bencher: &mut test::Bencher) {
        let input = common::read_input();
        let solver = Day3Part1Solver::parse(&input);
        bencher.iter(|| {
            let solution = solver.solve();
            solution.prove(&153_469_856);
        });
    }

    #[test]
    fn test_day_3_part_2_example() {
        let input = common::read_example_input(2);
        let solver = Day3Part2Solver::parse(&input);
        let solution = solver.solve();
        solution.prove(&48);
    }

    #[test]
    fn test_day_3_part_2() {
        let input = common::read_input();
        let solver = Day3Part2Solver::parse(&input);
        let solution = solver.solve();
        solution.prove(&77_055_967);
    }

    #[bench]
    fn bench_day_3_part_2(bencher: &mut test::Bencher) {
        let input = common::read_input();
        let solver = Day3Part2Solver::parse(&input);
        bencher.iter(|| {
            let solution = solver.solve();
            solution.prove(&77_055_967);
        });
    }
}
