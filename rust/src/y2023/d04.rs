use std::fs;

use itertools::Itertools;

type T = u128;
pub type ParsedData = Vec<T>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    for line in contents.trim_end().lines() {
        let mut cards = line.split(": ");
        cards.next();
        let mut parts = cards.next().unwrap().split(" | ");
        let mut wins = 0u128;
        for winning in parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|p| p.parse::<T>().unwrap())
        {
            wins += 1 << winning;
        }
        let mut winners = 0;
        for played in parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|p| 1 << p.parse::<T>().unwrap())
        {
            if (played & wins) > 0 {
                winners += 1;
            }
        }
        res.push(winners);
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

pub fn part1(x: &ParsedData) -> usize {
    let mut score = 0;
    for &w in x {
        if w > 0 {
            let mut s = 1;
            for _ in 1..w {
                s += s;
            }
            score += s;
        }
    }
    score
}

pub fn part2(x: &ParsedData) -> usize {
    let mut res = 0;
    let mut copies = x.iter().map(|_| 0).collect_vec();
    for (i, matches) in x.iter().enumerate() {
        let extra = 1 + copies[i];
        res += extra;
        for j in i + 1..i + 1 + x.len().min(*matches as usize) {
            copies[j] += extra;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(&parse_str(str_input)), 13);
        assert_eq!(part2(&parse_str(str_input)), 30);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/04.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 21558);
        assert_eq!(part2(&input), 10425665);
    }
}
