use sscanf::sscanf;
use std::{collections::HashMap, fs};

type Pos = (isize, isize);
pub type ParsedData = HashMap<Pos, isize>;

#[inline]
fn dist(a: Pos, b: Pos) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = HashMap::with_capacity(100);
    contents.trim().lines().for_each(|line| {
        let (sx, sy, bx, by) = sscanf!(
            line,
            "Sensor at x={isize}, y={isize}: closest beacon is at x={isize}, y={isize}"
        )
        .unwrap();
        let sensor = (sx, sy);
        res.insert(sensor, dist(sensor, (bx, by)));
    });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

const MAX_SIDE: isize = 4_000_000;

pub fn part1(by_radius: &ParsedData) -> usize {
    let mut intervals = Vec::with_capacity(50);
    by_radius.iter().for_each(|((x, y), radius)| {
        let arm = radius - (MAX_SIDE / 2 - y).abs();
        if arm >= 0 {
            intervals.push((x - arm, x + arm));
        }
    });
    intervals.sort_unstable();
    (intervals.iter().map(|v| v.1).max().unwrap() - intervals[0].0) as usize
}

pub fn part2(by_radius: &ParsedData) -> usize {
    let mut positive = HashMap::with_capacity(100);
    let mut negative = HashMap::with_capacity(100);
    by_radius.iter().for_each(|((x, y), r)| {
        positive
            .entry(y - x + r + 1)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        positive
            .entry(y - x - r - 1)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        negative
            .entry(y + x + r + 1)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        negative
            .entry(y + x - r - 1)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });
    for (a, _) in positive.iter().filter(|(_, v)| **v > 1) {
        for (b, _) in negative.iter().filter(|(_, nv)| **nv > 1) {
            let pos = ((b - a) / 2, (a + b) / 2);
            if 0 < pos.0
                && pos.0 < MAX_SIDE
                && 0 < pos.1
                && pos.1 < MAX_SIDE
                && by_radius
                    .iter()
                    .all(|(scanner, radius)| dist(pos, *scanner) > *radius)
            {
                return (MAX_SIDE * pos.0 + pos.1) as usize;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual() {
        let path = &"../inputs/2022/15.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 4737567);
        assert_eq!(part2(&input), 13267474686239);
    }
}
