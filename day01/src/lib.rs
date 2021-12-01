#![warn(clippy::pedantic)]

fn count_increasing_windows(window_size: usize) -> usize {
    let numbers: Vec<i32> = include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    // Example: If window size is 3: Comparing sum of
    // A, B, C     with sum of
    //    B, C, D
    // B and C are part of both windows so they can be ignored when
    // comparing sums of windows. Create windows of size 4 and
    // compare only A and D.
    numbers
        .windows(window_size + 1)
        .filter(|window| window[0] < window[window_size])
        .count()
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
