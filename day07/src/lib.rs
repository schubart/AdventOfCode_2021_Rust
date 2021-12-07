#![cfg(test)]
#![warn(clippy::pedantic)]

fn minimize<F>(cost: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    let positions: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .map(|p1| positions.iter().map(|&p2| cost(p1, p2)).sum())
        .min()
        .unwrap()
}

fn part1_cost(p1: i32, p2: i32) -> i32 {
    (p1 - p2).abs()
}

fn part2_cost(p1: i32, p2: i32) -> i32 {
    let distance = (p1 - p2).abs();

    // Gauss formula for sum of first n numbers.
    distance * (distance + 1) / 2
}

#[test]
fn test_part1() {
    assert_eq!(336_131, minimize(part1_cost));
}

#[test]
fn test_part2() {
    assert_eq!(92_676_646, minimize(part2_cost));
}
