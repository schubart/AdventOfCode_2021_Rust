#![cfg(test)]
#![warn(clippy::pedantic)]

type Number = i32;

enum Command {
    Forward(Number),
    Down(Number),
    Up(Number),
}

use Command::{Down, Forward, Up};

fn parse(line: &str) -> Command {
    let mut split = line.split(' ');
    let command = split.next().unwrap();
    let arg = split.next().unwrap().parse::<Number>().unwrap();

    match command {
        "forward" => Forward(arg),
        "down" => Down(arg),
        "up" => Up(arg),
        _ => panic!("Unexpected input: {line}"),
    }
}

fn part1() -> Number {
    let (horizontal, depth) = include_str!("input.txt").lines().map(parse).fold(
        (0, 0),
        |(horizontal, depth), command| match command {
            Forward(arg) => (horizontal + arg, depth),
            Down(arg) => (horizontal, depth + arg),
            Up(arg) => (horizontal, depth - arg),
        },
    );

    horizontal * depth
}

fn part2() -> Number {
    let (horizontal, depth, _aim) = include_str!("input.txt").lines().map(parse).fold(
        (0, 0, 0),
        |(horizontal, depth, aim), command| match command {
            Forward(arg) => (horizontal + arg, depth + aim * arg, aim),
            Down(arg) => (horizontal, depth, aim + arg),
            Up(arg) => (horizontal, depth, aim - arg),
        },
    );

    horizontal * depth
}

#[test]
fn test_part1() {
    assert_eq!(1_882_980, part1());
}

#[test]
fn test_part2() {
    assert_eq!(1_971_232_560, part2());
}
