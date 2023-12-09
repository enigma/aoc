use std::fs;

use itertools::Itertools;

type T = i64;
pub type ParsedData = Vec<Vec<T>>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    for line in contents.trim_end().lines() {
        res.push(
            line.split_ascii_whitespace()
                .map(|i| i.parse::<T>().unwrap())
                .collect_vec(),
        );
    }
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

fn extrapolate(ns: &Vec<T>) -> T {
    let mut res = 0;
    let mut last = ns.clone();
    let mut nxt;
    while !last.iter().all(|&i| i == 0) {
        let mut delta = last[0];
        nxt = vec![];
        for &n in last.iter().skip(1) {
            nxt.push(n - delta);
            delta = n;
        }
        res += delta;
        last = nxt;
    }
    res
}

pub fn part1(nss: &ParsedData) -> usize {
    nss.iter().map(|ns| extrapolate(ns)).sum::<T>() as usize
}

pub fn part2(nss: &ParsedData) -> usize {
    nss.iter()
        .map(|ns| extrapolate(&ns.iter().cloned().rev().collect_vec()))
        .sum::<T>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part1(&parse_str(str_input)), 114);
        assert_eq!(part2(&parse_str(str_input)), 2);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/09.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 1641934234);
        assert_eq!(part2(&input), 975);
    }
}
