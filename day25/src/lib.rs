#![cfg(test)]
#![warn(clippy::pedantic)]

type Map = Vec<Vec<char>>;

fn apply(map: &mut Map) -> bool {
    let mut moved = false;
    let width = map[0].len();
    let height = map.len();

    let mut new_map = map.clone();
    for y in 0..height {
        let mut x = 0;
        loop {
            if map[y][x] == '>' && map[y][(x + 1) % width] == '.' {
                new_map[y][x] = '.';
                new_map[y][(x + 1) % width] = '>';
                x += 2;
                moved = true;
            } else {
                x += 1;
            }
            if x >= width {
                break;
            }
        }
    }
    *map = new_map;

    let mut new_map = map.clone();
    for x in 0..width {
        let mut y = 0;
        loop {
            if map[y][x] == 'v' && map[(y + 1) % height][x] == '.' {
                new_map[y][x] = '.';
                new_map[(y + 1) % height][x] = 'v';
                y += 2;
                moved = true;
            } else {
                y += 1;
            }
            if y >= height {
                break;
            }
        }
    }
    *map = new_map;

    moved
}

fn part1() -> usize {
    let mut map: Map = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for iteration in 1.. {
        if !apply(&mut map) {
            return iteration;
        }
    }

    unreachable!();
}

#[test]
fn it_works() {
    assert_eq!(513, part1());
}
