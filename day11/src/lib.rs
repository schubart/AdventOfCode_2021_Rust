#![cfg(test)]
#![warn(clippy::pedantic)]

fn part1() -> usize {
    let mut values: Vec<Vec<usize>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| (c as usize - '0' as usize)).collect())
        .collect();

    for _ in 0..3 {
        for x in 0..values.len() {
            for y in 0..values[0].len() {
                print!("{}", values[x][y]);
            }
            println!();
        }
        println!();

        for x in 0..values.len() {
            for y in 0..values[0].len() {
                values[x][y] = (values[x][y] + 1) % 10
            }
        }

    }

    0
}

#[test]
fn test_part1() {
    assert_eq!(42, part1());
}
