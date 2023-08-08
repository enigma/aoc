use std::{cmp::Ordering, fs};

#[derive(Eq, Clone)]
pub enum Packet {
    Single(u8),
    Batch(Vec<Packet>),
}

pub type ParsedData = Vec<Packet>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    contents
        .trim()
        .lines()
        .filter(|l| !l.is_empty())
        .for_each(|line| {
            let mut stack = vec![];
            let mut batch = vec![];
            let mut single = 0 as u8;
            let mut iter = line[1..line.len() - 1].bytes();

            while let Some(c) = iter.next() {
                match c {
                    b'0'..=b'9' => single = single * 10 + (c - b'0'),
                    b',' => {
                        batch.push(Packet::Single(single));
                        single = 0;
                    }
                    b'[' => {
                        stack.push(batch);
                        batch = vec![];
                        single = 0;
                    }
                    b']' => {
                        if !batch.is_empty() && single > 0 {
                            batch.push(Packet::Single(single));
                            single = 0;
                        }
                        let packet = match single {
                            0 => Packet::Batch(batch),
                            v => Packet::Single(v),
                        };
                        batch = stack.pop().unwrap();
                        batch.push(packet);
                    }
                    _ => panic!("wtf?"),
                }
            }

            if !batch.is_empty() && single > 0 {
                batch.push(Packet::Single(single));
                single = 0;
            }
            let packet = match single {
                0 => Packet::Batch(batch.clone()),
                v => Packet::Single(v),
            };
            res.push(packet);
        });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Single(a), Packet::Single(b)) => a.cmp(b),
            (Packet::Batch(a), Packet::Batch(b)) => a.cmp(b),
            (a @ Packet::Single(_), Packet::Batch(b)) => std::slice::from_ref(a).cmp(b.as_slice()),
            (Packet::Batch(a), b @ Packet::Single(_)) => a.as_slice().cmp(std::slice::from_ref(b)),
        }
    }
}

// y2022 d13 part1 full    time:   [487.81 µs 491.59 µs 496.06 µs]
pub fn part1(packets: &ParsedData) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .filter(|(_, p)| p[0].cmp(&p[1]) != Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum()
}

// y2022 d13 part2 full    time:   [737.65 µs 739.43 µs 741.24 µs]
pub fn part2(packets: &ParsedData) -> usize {
    let mut total = packets.clone();
    let two = Packet::Single(2);
    let six = Packet::Single(6);
    total.push(two.clone());
    total.push(six.clone());
    total.sort_unstable();
    total
        .iter()
        .enumerate()
        .filter(|(_, p)| **p == two || **p == six)
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(part1(&parse_str(str_input)), 13);
        assert_eq!(part2(&parse_str(str_input)), 140);
    }

    #[test]
    fn actual() {
        let path = &"inputs/2022/13.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 5625);
        assert_eq!(part2(&input), 23111);
    }
}
