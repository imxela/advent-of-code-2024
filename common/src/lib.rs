#![allow(clippy::cargo_common_metadata)]

use std::fmt::Debug;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn read_input() -> String {
    let input_content: String =
        std::fs::read_to_string(std::env::current_dir().unwrap().join("res/inputs/input"))
            .expect("failed to read AoC input file!");

    input_content
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn read_example_input(part: usize) -> String {
    let input_content: String = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(format!("res/inputs/example_input_p{part}")),
    )
    .expect("failed to read AoC example input file!");

    input_content
}

/// Represents a solution to an Advent of Code problem.
/// The solution should have a single implementation for both steps in a day.
/// Use `AdventSolution<T>::solve()` to test your solution.
pub struct AdventSolution<T: PartialEq + Debug> {
    solution: T,
}

impl<T: PartialEq + Debug> AdventSolution<T> {
    pub fn new(solution: T) -> Self {
        Self { solution }
    }

    /// # Panics
    ///
    /// Will panic if `other` is not equal to the solution produced by the solver.
    pub fn prove(&self, other: &T) -> bool {
        self.solution == *other
    }
}

pub trait AdventSolver<T: PartialEq + Debug, D> {
    /// Parses the given input data into an appropriate structure an
    /// instance of its associated solver structure.
    fn parse(input: &str) -> Self;

    /// Attempts to solve the puzzle immutably using the parsed input data.
    fn solve(&self) -> AdventSolution<T>;

    /// Return the parsed puzzle input.
    fn data(&self) -> &D;
}

pub trait AdventSolverMut<T: PartialEq + Debug, D> {
    /// Parses the given input data into an appropriate structure an
    /// instance of its associated solver structure.
    fn parse(input: &str) -> Self;

    /// Attempts to solve the puzzle mutably using the parsed input data.
    fn solve(&mut self) -> AdventSolution<T>;

    /// Return the parsed puzzle input.
    fn data(&self) -> &D;
}

impl<T: PartialEq + Debug> std::fmt::Display for AdventSolution<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.solution)
    }
}

impl<T: PartialEq + Debug> From<T> for AdventSolution<T> {
    fn from(value: T) -> Self {
        AdventSolution { solution: value }
    }
}
