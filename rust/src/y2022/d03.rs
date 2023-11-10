use std::collections::HashSet;
use std::fs;

pub type ParsedData = Vec<String>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    contents.lines().for_each(|x| {
        if !x.is_empty() {
            res.push(x.to_string());
        }
    });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        (c as u8 - b'a') as usize + 1
    } else if c.is_ascii_uppercase() {
        (c as u8 - b'A') as usize + 27
    } else {
        0
    }
}

// y2022 d03 part1         time:   [300.41 µs 301.67 µs 303.34 µs]
pub fn part1(rucksacks: &ParsedData) -> usize {
    let mut res = 0;
    for r in rucksacks.iter() {
        let a: HashSet<char> = r[..r.len() / 2].chars().collect();
        let b: HashSet<char> = r[r.len() / 2..].chars().collect();
        let c: HashSet<char> = a.intersection(&b).cloned().collect();
        if let Some(x) = c.iter().next() {
            res += priority(*x);
        }
    }
    res
}

// y2022 d03 part2         time:   [322.50 µs 323.32 µs 324.15 µs]
pub fn part2(rucksacks: &ParsedData) -> usize {
    let mut res = 0;
    let mut i = 0;
    while i < rucksacks.len() {
        let a: HashSet<char> = rucksacks[i + 0].chars().collect();
        let b: HashSet<char> = rucksacks[i + 1].chars().collect();
        let c: HashSet<char> = rucksacks[i + 2].chars().collect();
        let common: HashSet<char> = a.intersection(&b).cloned().collect();
        let common: HashSet<char> = common.intersection(&c).cloned().collect();
        if let Some(x) = common.iter().next() {
            res += priority(*x);
        }
        i += 3;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(part1(&parse_str(str_input)), 157);
        assert_eq!(part2(&parse_str(str_input)), 70);
    }

    #[test]
    fn actual() {
        let numbers = parse(&"../inputs/2022/03.input".to_string());
        assert_eq!(part1(&numbers), 7795);
        assert_eq!(part2(&numbers), 2703);
    }
}
