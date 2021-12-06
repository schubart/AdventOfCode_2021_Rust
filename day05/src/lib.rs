#![cfg(test)]
#![warn(clippy::pedantic)]

use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

fn part1() -> usize {
    let lines = include_str!("input.txt")
        .lines()
        .map(|l| {
            let split = l.split(&[' ', ','][..]).collect::<Vec<_>>();
            (
                split[0].parse::<i32>().unwrap(),
                split[1].parse::<i32>().unwrap(),
                split[3].parse::<i32>().unwrap(),
                split[4].parse::<i32>().unwrap(),
            )
        })
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
        .collect::<Vec<_>>();

    let mut counts = HashMap::new();
    for (x1, y1, x2, y2) in lines {
        for x in min(x1, x2)..=max(x1, x2) {
            for y in min(y1, y2)..=max(y1, y2) {
                *counts.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    counts.values().filter(|c| **c > 1).count()
}

#[test]
fn test_part1() {
    assert_eq!(4_728, part1());
}
