#![cfg(test)]
//#![warn(clippy::pedantic)]

use std::collections::HashSet;    
    
fn get(matrix: &[Vec<u8>], x: isize, y: isize) -> &u8 {
    if y < 0 || y >= (matrix.len() as isize) {
        return &u8::MAX;
    }
    let row = &matrix[y as usize];

    if x < 0 || x >= (row.len() as isize) {
        return &u8::MAX;
    }

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
            let x = x as isize;
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
                let point = (x as isize, y as isize);
                points.insert(point);
            }
        }
    }

    while let Some(&point) = points.iter().next() {
        points.remove(&point);
    }
}

fn fill(points: &mut HashSet<Point>, point: Point) {
    points.remove(&point);
}
