#![cfg(test)]
#![warn(clippy::pedantic)]

use std::cmp::max;
use std::collections::HashMap;

type Counts = (usize, usize); // Player 1 win count, player 2 win count.
type Cache = HashMap<State, Counts>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    positions: [usize; 2],
    scores: [usize; 2],
    next: usize, // Index of next player to move; 0 or 1.
}

impl State {
    fn new(p1: usize, p2: usize) -> Self {
        State {
            positions: [p1, p2],
            scores: [0, 0],
            next: 0,
        }
    }

    fn apply(&self, steps: usize) -> Self {
        let mut result = *self;

        // Move position.
        let mut position = result.positions[result.next];
        position += steps;
        position = ((position - 1) % 10) + 1; // 1..=10
        result.positions[result.next] = position;
        // Increment score.
        result.scores[result.next] += position;
        // Switch whose turn it is.
        result.next = (result.next + 1) % 2;

        result
    }
}

fn part1(mut state: State) -> usize {
    let mut die = (1..=100).cycle();

    for round in 1.. {
        state = state.apply(die.by_ref().take(3).sum());
        // Previous (aka next) player won?
        if state.scores[(state.next + 1) % 2] >= 1000 {
            // Current player is the loser.
            return round * 3 * state.scores[state.next];
        }
    }

    unreachable!()
}

fn part2(cache: &mut Cache, state: State) -> Counts {
    // Plater 1 won?
    if state.scores[0] >= 21 {
        return (1, 0);
    }
    // PLayer 2 won?
    if state.scores[1] >= 21 {
        return (0, 1);
    }

    if let Some(result) = cache.get(&state) {
        // Cahce hit.
        return *result;
    }

    // Cache miss. Do the work.
    let mut p1_sum = 0;
    let mut p2_sum = 0;
    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                let next = state.apply(d1 + d2 + d3);
                let (p1, p2) = part2(cache, next);
                p1_sum += p1;
                p2_sum += p2;
            }
        }
    }

    // Store in cache.
    let result = (p1_sum, p2_sum);
    cache.insert(state, result);
    result
}

#[test]
fn test_part1() {
    assert_eq!(757_770, part1(State::new(6, 8)));
}

#[test]
fn test_part2() {
    let (p1, p2) = part2(&mut Cache::new(), State::new(6, 8));
    assert_eq!(712_381_680_443_927, max(p1, p2));
}
