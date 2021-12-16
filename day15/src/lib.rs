#![cfg(test)]
#![warn(clippy::pedantic)]

fn part1() -> usize {
    let matrix: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    
    0
}

#[test]
fn test_part1() {
    assert_eq!(3_009, part1());
}
