#![cfg(test)]
#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::iter::repeat;

type Number = i32;
type Line = (Number, Number, Number, Number);

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse(text: &str) -> Result<Line> {
    let split = text.split(&[' ', ','][..]).collect::<Vec<_>>();

    Ok((
        split[0].parse()?,
        split[1].parse()?,
        split[3].parse()?,
        split[4].parse()?,
    ))
}

// Iterates from `a` (inclusive) to `b` (inclusive). Counts down if `a` > `b`.
fn iterate(a: Number, b: Number) -> Box<dyn Iterator<Item = Number>> {
    if a < b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

fn boxed_repeat(n: Number) -> Box<dyn Iterator<Item = Number>> {
    Box::new(repeat(n))
}

fn count_overlaps(lines: &[Line]) -> usize {
    let mut counts = HashMap::new();

    lines
        .iter()
        .flat_map(|&(x1, y1, x2, y2)| {
            if x1 == x2 {
                boxed_repeat(x1).zip(iterate(y1, y2)) // Vertical
            } else if y1 == y2 {
                iterate(x1, x2).zip(boxed_repeat(y1)) // Horizontal
            } else {
                iterate(x1, x2).zip(iterate(y1, y2)) // Diagonal
            }
        })
        .for_each(|p| *counts.entry(p).or_insert(0) += 1);

    counts.values().filter(|c| **c > 1).count()
}

fn part1() -> Result<usize> {
    let mut lines: Vec<Line> = include_str!("input.txt")
        .lines()
        .map(parse)
        .collect::<Result<_>>()?;

    // Keep only vertical and horizontal lines.
    lines.retain(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2);

    Ok(count_overlaps(&lines))
}

fn part2() -> Result<usize> {
    let lines: Vec<Line> = include_str!("input.txt")
        .lines()
        .map(parse)
        .collect::<Result<_>>()?;

    Ok(count_overlaps(&lines))
}

#[test]
fn test_part1() {
    assert_eq!(4_728, part1().unwrap());
}

#[test]
fn test_part2() {
    assert_eq!(17_717, part2().unwrap());
}
