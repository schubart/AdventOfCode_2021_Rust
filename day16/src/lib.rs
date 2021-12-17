#![cfg(test)]
#![warn(clippy::pedantic)]

type Bit = usize;
type Version = usize;
type Value = usize;
type Output = (Version, Value);

struct Bits<I> {
    iterator: I,
    remaining: usize,
}

impl<I> Bits<I>
where
    I: Iterator<Item = Bit>,
{
    pub fn parse_packet(&mut self) -> Output {
        let this_version = self.take(3);
        let this_type = self.take(3);

        let (version, value) = match this_type {
            4 => self.parse_literal(),
            0 => self.parse_operator(|x, y| x + y),
            1 => self.parse_operator(|x, y| x * y),
            2 => self.parse_operator(std::cmp::min),
            3 => self.parse_operator(std::cmp::max),
            5 => self.parse_operator(|x, y| (x > y) as Value),
            6 => self.parse_operator(|x, y| (x < y) as Value),
            7 => self.parse_operator(|x, y| (x == y) as Value),
            _ => panic!(),
        };

        (this_version + version, value)
    }

    fn parse_literal(&mut self) -> Output {
        let mut value = 0;
        let mut has_more = true;
        while has_more {
            has_more = self.take(1) == 1;

            value = value * 16 + self.take(4);
        }
        (0, value)
    }

    fn parse_operator<F>(&mut self, function: F) -> Output
    where
        F: FnMut(Value, Value) -> Value,
    {
        let length_type = self.take(1);

        let mut sub_packets = Vec::new();
        if length_type == 0 {
            let length = self.take(15);
            let old_remaining = self.remaining;
            self.remaining = length;

            while self.remaining > 0 {
                sub_packets.push(self.parse_packet());
            }

            self.remaining = old_remaining - length;
        } else {
            let count = self.take(11);

            for _ in 0..count {
                sub_packets.push(self.parse_packet());
            }
        }

        assert!(!sub_packets.is_empty());
        let version = sub_packets.iter().map(|pair| pair.0).sum();
        let values = sub_packets.iter().map(|pair| pair.1).collect::<Vec<_>>();
        let result = values.iter().copied().reduce(function).unwrap();

        (version, result)
    }

    fn take(&mut self, count: usize) -> Value {
        assert!(count <= self.remaining);
        self.remaining -= count;

        (0..count).fold(0, |result, _| result * 2 + self.iterator.next().unwrap())
    }
}

fn parse() -> Output {
    let mut bits = Bits {
        iterator: include_str!("input.txt").trim().chars().flat_map(|c| {
            // Turn hex digit to number and then to four digit binary.
            format!("{:04b}", c.to_digit(16).unwrap())
                .chars()
                .map(|c| c.to_digit(2).unwrap() as Value)
                .collect::<Vec<_>>()
        }),
        remaining: usize::MAX,
    };

    bits.parse_packet()
}

#[test]
fn test_part1() {
    assert_eq!(969, parse().0);
}

#[test]
fn test_part2() {
    assert_eq!(124_921_618_408, parse().1);
}
