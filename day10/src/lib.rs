#![cfg(test)]
#![warn(clippy::pedantic)]

fn illegal(input: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return Some(c);
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return Some(c);
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return Some(c);
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return Some(c);
                }
            }
            _ => panic!(),
        }
    }

    None
}

fn part1() -> usize {
    include_str!("input.txt")
        .lines()
        .filter_map(illegal)
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1_197,
            '>' => 25_137,
            _ => panic!(),
        })
        .sum()
}

fn missing(input: &str) -> Option<usize> {
    let mut stack = Vec::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return None;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return None;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return None;
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return None;
                }
            }
            _ => panic!(),
        }
    }

    Some(stack.iter().rev().fold(0, |result, c| {
        result * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            }
    }))
}

fn part2() -> usize {
    let mut scores = include_str!("input.txt")
        .lines()
        .filter_map(missing)
        .collect::<Vec<_>>();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

#[test]
fn test_part1() {
    assert_eq!(392_367, part1());
}

#[test]
fn test_part2() {
    assert_eq!(2_192_104_158, part2());
}
