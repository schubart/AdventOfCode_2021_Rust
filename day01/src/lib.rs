#![warn(clippy::pedantic)]

fn count_increasing_windows(window_size: usize) -> usize {
    let numbers: Vec<i32> = include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let sums: Vec<i32> = numbers
        .windows(window_size)
        .map(|window| window.iter().sum())
        .collect();

    sums.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

/// # Panics
///
/// Will panic if the input is malformed.
///
/// ```
/// assert_eq!(1_602, day01::part1());
/// ```
#[must_use]
pub fn part1() -> usize {
    count_increasing_windows(1)
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
    count_increasing_windows(3)
}
