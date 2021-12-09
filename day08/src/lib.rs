#![cfg(test)]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

type Signal = HashSet<char>;
type Lookup = [Signal; 10];

fn part1() -> usize {
    include_str!("input.txt")
        .lines()
        .flat_map(|l| l.split(' ').skip(11))
        .map(str::len)
        .filter(|&l| l == 2 || l == 3 || l == 4 || l == 7)
        .count()
}

// If there is exactly one matching element, remove and return it. Else panic.
fn remove_only<T, F>(input: &mut Vec<T>, predicate: F) -> T
where
    T: Clone,
    F: Fn(&&T) -> bool + Copy,
{
    let mut results = input.iter().filter(predicate);
    let result = results.next().expect("no element found").clone();
    assert!(results.next().is_none(), "multiple elements found");

    // Vec::drain_filter would be useful here, but don't want to depend on nighly.
    input.retain(|x| !predicate(&x));

    result
}

fn decode(input: &mut Vec<Signal>) -> Lookup {
    // Easy cases.
    let n1 = remove_only(input, |x| x.len() == 2);
    let n4 = remove_only(input, |x| x.len() == 4);
    let n7 = remove_only(input, |x| x.len() == 3);
    let n8 = remove_only(input, |x| x.len() == 7);

    // 3 is the only 5-segment digit that shares 2 segments with digit 1.
    let n3 = remove_only(input, |x| x.len() == 5 && (*x & &n1).len() == 2);
    let n2 = remove_only(input, |x| x.len() == 5 && (*x & &n4).len() == 2);
    // 5 is the only remaining 5-segment digit.
    let n5 = remove_only(input, |x| x.len() == 5);

    // And so on.
    let n6 = remove_only(input, |x| x.len() == 6 && (*x & &n1).len() == 1);
    let n9 = remove_only(input, |x| x.len() == 6 && (*x & &n4).len() == 4);
    let n0 = remove_only(input, |x| x.len() == 6);

    assert!(input.is_empty());

    [n0, n1, n2, n3, n4, n5, n6, n7, n8, n9]
}

fn apply(lookup: &Lookup, output: &[Signal]) -> usize {
    output.iter().fold(0, |result, x| {
        result * 10
            + lookup
                .iter()
                .enumerate()
                .find(|(_, y)| x == *y)
                .map(|(index, _)| index)
                .unwrap()
    })
}

fn part2() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut output: Vec<Signal> = line.split(' ').map(|x| x.chars().collect()).collect();
            let mut input = output.drain(0..10).collect();
            output.remove(0); // Remove | separator.

            let lookup = decode(&mut input);
            apply(&lookup, &output)
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(452, part1());
}

#[test]
fn test_part2() {
    assert_eq!(1_096_964, part2());
}
