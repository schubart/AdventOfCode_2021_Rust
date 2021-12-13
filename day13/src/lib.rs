#![cfg(test)]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

type Number = i32;
type Point = (Number, Number);

enum Fold {
    X(Number),
    Y(Number),
}

fn parse() -> (HashSet<Point>, Vec<Fold>) {
    let mut lines = include_str!("input.txt").lines();

    let points = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let folds = lines
        .map(|line| match line.split_once('=').unwrap() {
            ("fold along x", x) => Fold::X(x.parse().unwrap()),
            ("fold along y", y) => Fold::Y(y.parse().unwrap()),
            _ => panic!(),
        })
        .collect();

    (points, folds)
}

fn fold_points(points: &mut HashSet<Point>, fold: &Fold) {
    fn flip(value: Number, axis: Number) -> Number {
        if value > axis {
            axis - (value - axis)
        } else {
            value
        }
    }

    *points = match *fold {
        Fold::X(axis) => points.iter().map(|&(x, y)| (flip(x, axis), y)).collect(),
        Fold::Y(axis) => points.iter().map(|&(x, y)| (x, flip(y, axis))).collect(),
    }
}

fn part1() -> usize {
    let (mut points, folds) = parse();

    folds
        .iter()
        .take(1)
        .for_each(|fold| fold_points(&mut points, fold));

    points.len()
}

fn part2() -> String {
    let (mut points, folds) = parse();

    folds.iter().for_each(|fold| fold_points(&mut points, fold));

    let min_x = points.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = points.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = points.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = points.iter().map(|&(_, y)| y).max().unwrap();

    let mut result = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            result += if points.contains(&(x, y)) { "#" } else { "." };
        }
        result += "\n";
    }

    result
}

#[test]
fn test_part1() {
    assert_eq!(745, part1());
}

#[test]
fn test_part2() {
    assert_eq!(include_str!("output.txt"), part2());
}
