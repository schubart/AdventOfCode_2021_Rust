#![cfg(test)]
#![warn(clippy::pedantic)]

fn part1() -> usize {
    include_str!("input.txt")
        .lines()
        .flat_map(|l| l.split(' ').skip(11))
        .map(str::len)
        .filter(|&l| l == 2 || l == 3 || l == 4 || l == 7)
        .count()
}

#[test]
fn test_part1() {
    assert_eq!(452, part1());
}
