#![cfg(test)]
#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::collections::HashSet;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn is_small(string: &str) -> bool {
    !string.chars().all(char::is_uppercase)
}

fn parse() -> Graph<'static> {
    let mut graph = HashMap::new();
    include_str!("input.txt").lines().for_each(|l| {
        let (from, to) = l.split_once('-').unwrap();

        assert!(is_small(from) || is_small(to));

        graph.entry(from).or_insert_with(Vec::new).push(to);
        graph.entry(to).or_insert_with(Vec::new).push(from);
    });

    graph
}

fn count_paths<'a>(
    current: &'a str,
    visited: &mut HashSet<&'a str>,
    graph: &Graph<'a>,
    may_visit_twice: bool,
) -> usize {
    if current == "end" {
        return 1;
    }

    let inserted = visited.insert(current);

    let mut result = 0;
    for next in &graph[current] {
        if *next == "start" {
            continue;
        }

        let is_second_visit = is_small(next) && visited.contains(next);

        if is_second_visit {
            if may_visit_twice {
                result += count_paths(next, visited, graph, false);
            }
        } else {
            result += count_paths(next, visited, graph, may_visit_twice);
        }
    }

    if inserted {
        visited.remove(current);
    }

    result
}

#[test]
fn test_part1() {
    assert_eq!(
        5_333,
        count_paths("start", &mut HashSet::new(), &parse(), false)
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        146_553,
        count_paths("start", &mut HashSet::new(), &parse(), true)
    );
}
