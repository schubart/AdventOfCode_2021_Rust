#![cfg(test)]
#![warn(clippy::pedantic)]

fn part1() -> i32 {
    let (horizontal, depth) =
        include_str!("input.txt")
            .lines()
            .fold((0, 0), |(horizontal, depth), line| {
                let mut split = line.split(' ');
                let command = split.next().unwrap();
                let arg = split.next().unwrap().parse::<i32>().unwrap();

                match command {
                    "forward" => (horizontal + arg, depth),
                    "down" => (horizontal, depth + arg),
                    "up" => (horizontal, depth - arg),
                    _ => panic!("Unexpected input: {}", line),
                }
            });

    horizontal * depth
}

#[test]
fn test_part1() {
    assert_eq!(1_882_980, part1());
}

fn part2() -> i32 {
    let (horizontal, depth, _aim) =
        include_str!("input.txt")
            .lines()
            .fold((0, 0, 0), |(horizontal, depth, aim), line| {
                let mut split = line.split(' ');
                let command = split.next().unwrap();
                let arg = split.next().unwrap().parse::<i32>().unwrap();

                match command {
                    "forward" => (horizontal + arg, depth + aim * arg, aim),
                    "down" => (horizontal, depth, aim + arg),
                    "up" => (horizontal, depth, aim - arg),
                    _ => panic!("Unexpected input: {}", line),
                }
            });

    horizontal * depth
}

#[test]
fn test_part2() {
    assert_eq!(1_971_232_560, part2());
}
