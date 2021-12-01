#![warn(clippy::pedantic)]

use itertools::Itertools;

/// # Panics
///
/// Will panic if the input is malformed.
///
/// ```
/// assert_eq!(1_602, day01::part1());
/// ```
#[must_use]
pub fn part1() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(x, y)| x < y)
        .count()
}

/// # Panics
///
/// Will panic if the input is malformed.
///
/// ```
/// assert_eq!(1_633, day01::part2());
/// ```
#[must_use]
pub fn part2() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(x1, x2, x3)| x1 + x2 + x3)
        .tuple_windows()
        .filter(|(x, y)| x < y)
        .count()
}
