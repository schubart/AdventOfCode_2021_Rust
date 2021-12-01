#![warn(clippy::pedantic)]

/// # Panics
///
/// Will panic if the input is malformed.
///
/// ```
/// assert_eq!(1_602, day01::part1());
/// ```
#[must_use]
pub fn part1() -> usize {
    let numbers: Vec<i32> = include_str!("input.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    numbers.windows(2).filter(|pair| pair[0] < pair[1]).count()
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
    let numbers: Vec<i32> = include_str!("input.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let sums: Vec<i32> = numbers
        .windows(3)
        .map(|triple| triple.iter().sum())
        .collect();

    sums.windows(2).filter(|pair| pair[0] < pair[1]).count()
}
