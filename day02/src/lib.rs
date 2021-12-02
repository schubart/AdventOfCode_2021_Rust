#![cfg(test)]
#![warn(clippy::pedantic)]

fn part1() -> i32 {
    let mut h = 0;
    let mut d = 0;
    include_str!("input.txt").lines().for_each(|line| {
        let mut split = line.split(' ');
        let command = split.next().unwrap();
        let arg = split.next().unwrap().parse::<i32>().unwrap();

        match command {
            "forward" => h += arg,
            "down" => d += arg,
            "up" => d -= arg,
            _ => panic!(),
        }
    });

    h * d
}

#[test]
fn test_part1() {
    assert_eq!(1_882_980, part1());
}

fn part2() -> i32 {
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;
    include_str!("input.txt").lines().for_each(|line| {
        let mut split = line.split(' ');
        let command = split.next().unwrap();
        let arg = split.next().unwrap().parse::<i32>().unwrap();

        match command {
            "forward" => {
                h += arg;
                d += aim * arg;
            }
            "down" => aim += arg,
            "up" => aim -= arg,
            _ => panic!(),
        }
    });

    h * d
}

#[test]
fn test_part2() {
    assert_eq!(1_971_232_560, part2());
}
