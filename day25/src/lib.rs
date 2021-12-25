#![cfg(test)]
#![warn(clippy::pedantic)]

fn part1() -> usize {
    let mut map: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let width = map[0].len();
    let height = map.len();

    for iteration in 1.. {
        let mut moved = false;
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
        map = new_map;

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

        if !moved {
            return iteration;
        }

        map = new_map;

//        println!();
//        for row in &map {
//            println!("{}", row.iter().collect::<String>());
//        }
    }

    unreachable!();
}

#[test]
fn it_works() {
    assert_eq!(513, part1());
}
