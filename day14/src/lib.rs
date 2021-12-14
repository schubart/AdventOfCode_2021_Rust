#![cfg(test)]
#![warn(clippy::pedantic)]

use std::collections::HashMap;

type Element = char;
type Pair = (Element, Element);

fn simulate(steps: usize) -> usize {
    // Parse input and rules.
    let mut lines = include_str!("input.txt").lines();
    let input: Vec<Element> = lines.next().unwrap().chars().collect();
    let rules: HashMap<Pair, Vec<Pair>> = lines
        .skip(1)
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(from, to)| {
            // Turn "AB -> X" into "AB -> [AX, XB]"
            let a = from.chars().next().unwrap();
            let b = from.chars().nth(1).unwrap();
            let x = to.chars().next().unwrap();

            ((a, b), vec![(a, x), (x, b)])
        })
        .collect();

    // Count initial pairs.
    let mut pair_counts: HashMap<Pair, usize> = HashMap::new();
    input
        .windows(2)
        .map(|window| (window[0], window[1])) // Turn vector of length 2 into pair.
        .for_each(|pair| *pair_counts.entry(pair).or_default() += 1);

    // Run simulation.
    for _ in 0..steps {
        pair_counts = pair_counts
            .iter()
            .fold(HashMap::new(), |mut counts, (pair, count)| {
                // AB occurs n times and maps to AX and XB -> Increase counts of AX and of XB by n.
                rules[pair]
                    .iter()
                    .for_each(|pair2| *counts.entry(*pair2).or_default() += count);
                counts
            });
    }

    // Count the first elements of each pair.
    let mut element_counts: HashMap<Element, usize> = HashMap::new();
    for (pair, count) in &pair_counts {
        *element_counts.entry(pair.0).or_default() += count;
    }
    // Count the last element of the initial (and final) input.
    let last = input.last().unwrap();
    *element_counts.entry(*last).or_default() += 1;

    // Calculate result as per problem statement.
    let min = element_counts.values().min().unwrap();
    let max = element_counts.values().max().unwrap();
    max - min
}

#[test]
fn test_part1() {
    assert_eq!(3_009, simulate(10));
}

#[test]
fn test_part2() {
    assert_eq!(3_459_822_539_451, simulate(40));
}
