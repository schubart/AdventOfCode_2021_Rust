#![cfg(test)]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

type Number = i32;
type Board = Vec<Vec<Number>>;

fn parse() -> (Vec<Number>, Vec<Board>) {
    let mut lines = include_str!("input.txt").lines();

    // Parse first line (numbers).
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    // Expect empty line between numbers and boards.
    assert_eq!(Some(""), lines.next());

    // Parse boards.
    let mut boards = Vec::new();
    loop {
        let board: Board = (0..5)
            .map(|_| {
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();
        boards.push(board);

        if lines.next().is_none() {
            break;
        }
    }

    (numbers, boards)
}

fn is_solution(board: &[Vec<Number>], numbers: &HashSet<Number>) -> bool {
    let size = board.len();
    let vert = (0..size).any(|x| (0..size).all(|y| numbers.contains(&board[x][y])));
    let hori = (0..size).any(|x| (0..size).all(|y| numbers.contains(&board[y][x])));
    vert || hori
}

fn checksum(board: &[Vec<Number>], numbers: &HashSet<Number>) -> Number {
    board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|n| !numbers.contains(n))
        .sum()
}

fn part1() -> Number {
    let (numbers, boards) = parse();

    let mut current = HashSet::new();
    for n in numbers {
        current.insert(n);

        let mut solutions = boards.iter().filter(|b| is_solution(b, &current));
        if let Some(board) = solutions.next() {
            // Make sure only one solution was found.
            assert!(solutions.next().is_none());
            return checksum(board, &current) * n;
        }
    }

    panic!("No solution was found");
}

fn part2() -> Number {
    let (numbers, mut boards) = parse();

    let mut current = HashSet::new();
    for n in numbers {
        current.insert(n);

        if boards.len() == 1 {
            // End game mode: Check if last board is now solved, without
            // deleting it by calling retain().
            let only = boards.first().unwrap();
            if is_solution(only, &current) {
                return checksum(only, &current) * n;
            }
        } else {
            boards.retain(|b| !is_solution(b, &current));
        }
    }

    panic!("No solution was found");
}

#[test]
fn test_part1() {
    assert_eq!(49_860, part1());
}

#[test]
fn test_part2() {
    assert_eq!(24_628, part2());
}
