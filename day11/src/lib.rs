#![cfg(test)]
#![warn(clippy::pedantic)]

fn part1() -> usize {
    let values: Vec<Vec<usize>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| (c as usize - '0' as usize)).collect())
        .collect();
    dbg!(values).len()
}

#[test]
fn test_part1() {
    assert_eq!(42, part1());
}
