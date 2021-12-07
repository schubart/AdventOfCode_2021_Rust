#![cfg(test)]
#![warn(clippy::pedantic)]

type Number = i32;

fn minimize<F>(cost: F) -> Number
where
    F: Fn(Number) -> Number,
{
    let positions: Vec<Number> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .map(|p1| positions.iter().map(|&p2| cost((p1 - p2).abs())).sum())
        .min()
        .unwrap()
}

fn part1_cost(distance: Number) -> Number {
    distance
}

fn part2_cost(distance: Number) -> Number {
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
