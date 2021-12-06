#![cfg(test)]
#![warn(clippy::pedantic)]

fn simulate(generations: usize) -> usize {
    let mut counters = [0; 9];
    include_str!("input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| counters[n] += 1);

    for _ in 0..generations {
        let spawn_count = counters[0];
        counters.rotate_left(1);
        counters[6] += spawn_count; // Parents restart at 6.
        counters[8] = spawn_count; // Children start at 8.
    }

    counters.iter().sum()
}

#[test]
fn test_part1() {
    assert_eq!(352_151, simulate(80));
}

#[test]
fn test_part2() {
    assert_eq!(1_601_616_884_019, simulate(256));
}
