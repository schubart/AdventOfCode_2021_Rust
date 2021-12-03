#![cfg(test)]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

type Bit = bool;
type Word = Vec<Bit>;
type Words = HashSet<Word>;
type Number = i32;

fn part1() -> Number {
    let words: Words = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect();
    let width = words.iter().next().unwrap().len();

    let (gamma, epsilon) = (0..width).fold((0, 0), |(gamma, epsilon), position| {
        let ones = words.iter().filter(|word| word[position]).count();
        let zeros = words.len() - ones;
        let majority_value = ones >= zeros;

        (
            (gamma << 1) | majority_value as Number,
            (epsilon << 1) | !majority_value as Number,
        )
    });

    gamma * epsilon
}

fn shrink(mut words: Words, retain_majority: bool) -> Number {
    let width = words.iter().next().unwrap().len();

    for position in 0..width {
        let ones = words.iter().filter(|word| word[position]).count();
        let zeros = words.len() - ones;
        let majority_value = ones >= zeros;
        let retain_value = majority_value == retain_majority;

        words.retain(|word| word[position] == retain_value);

        if words.len() == 1 {
            let word = words.iter().next().unwrap();
            // Convert bits to number.
            return word.iter().fold(0, |x, bit| (x << 1) | *bit as Number);
        }
    }

    panic!();
}

fn part2() -> Number {
    let words: Words = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect();

    let oxygen = shrink(words.clone(), true);
    let co2 = shrink(words, false);

    oxygen * co2
}

#[test]
fn test_part1() {
    assert_eq!(3_882_564, part1());
}

#[test]
fn test_part2() {
    assert_eq!(3_385_170, part2());
}
