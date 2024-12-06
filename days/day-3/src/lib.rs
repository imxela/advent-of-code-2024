#![allow(clippy::cargo_common_metadata)]

use common::AdventSolver;

struct Day3Solver {
    data: Vec<u8>,
}

impl AdventSolver<usize, Vec<u8>> for Day3Solver {
    fn parse(input: &str) -> Self {
        Day3Solver {
            data: input.as_bytes().to_vec(),
        }
    }

    fn solve(&self) -> common::AdventSolution<usize> {
        dbg!(&self.data);
        161.into()
    }

    fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_part_1() {
        let input = String::from("3"); // common::read_example_input(3);
        let solver = Day3Solver::parse(&input);
        let solution = solver.solve();
        println!("Solution is {solution}");
        assert!(solution.prove(&161));
    }
}
