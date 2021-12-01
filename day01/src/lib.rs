#![cfg(test)]
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

#[test]
fn part1() {
    assert_eq!(1_602, count_increasing_windows(1));
}

#[test]
fn part2() {
    assert_eq!(1_633, count_increasing_windows(3));
}
