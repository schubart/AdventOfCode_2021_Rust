#![cfg(test)]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

fn get(matrix: &[Vec<u8>], x: isize, y: isize) -> &u8 {
    #[allow(clippy::cast_possible_wrap)]
    if y < 0 || y >= (matrix.len() as isize) {
        return &u8::MAX;
    }
    #[allow(clippy::cast_sign_loss)]
    let row = &matrix[y as usize];

    #[allow(clippy::cast_possible_wrap)]
    if x < 0 || x >= (row.len() as isize) {
        return &u8::MAX;
    }

    #[allow(clippy::cast_sign_loss)]
    &row[x as usize]
}

fn part1() -> usize {
    let heights: Vec<Vec<u8>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| (c as u8) - b'0').collect())
        .collect();

    let mut result = 0;
    for (y, row) in heights.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            #[allow(clippy::cast_possible_wrap)]
            let x = x as isize;
            #[allow(clippy::cast_possible_wrap)]
            let y = y as isize;

            if height >= get(&heights, x - 1, y) {
                continue;
            }
            if height >= get(&heights, x + 1, y) {
                continue;
            }
            if height >= get(&heights, x, y - 1) {
                continue;
            }
            if height >= get(&heights, x, y + 1) {
                continue;
            }

            result += (*height as usize) + 1;
        }
    }

    result
}

#[test]
fn test_part1() {
    assert_eq!(524, part1());
}

type Point = (isize, isize);

#[test]
fn test_part2() {
    let heights: Vec<Vec<u8>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| (c as u8) - b'0').collect())
        .collect();

    let mut points = HashSet::new();

    for (y, row) in heights.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height != 9 {
                #[allow(clippy::cast_possible_wrap)]
                let point = (x as isize, y as isize);
                points.insert(point);
            }
        }
    }

    let mut pools = vec![];
    while let Some(&point) = points.iter().next() {
        pools.push(fill(&mut points, point));
    }

    pools.sort_unstable();
    let p: usize = pools.iter().rev().take(3).product();
    assert_eq!(1_235_430, p);
}

fn fill(points: &mut HashSet<Point>, point @ (x, y): Point) -> usize {
    if !points.remove(&point) {
        return 0;
    }

    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    return 1 + deltas
        .iter()
        .map(|(dx, dy)| fill(points, (x + dx, y + dy)))
        .sum::<usize>();
}
