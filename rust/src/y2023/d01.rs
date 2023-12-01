use std::fs;

use itertools::Itertools;

pub type ParsedData = Vec<String>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    contents.trim().lines().map(|s| s.to_string()).collect_vec()
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

static WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

// y2023 d01 complete      time:   [83.082 µs 83.923 µs 84.820 µs]
fn solve(line: &str, include_words: bool) -> usize {
    let mut first = 0;
    let mut last = 0;

    for start in 0..line.len() {
        let prefix = &line[start..];
        let c = prefix.chars().nth(0).unwrap();
        if c > '0' && c <= '9' {
            first = c as usize - '0' as usize;
            break;
        } else if include_words {
            for (i, &word) in WORDS.iter().enumerate() {
                if prefix.starts_with(word) {
                    first = i + 1;
                    break;
                }
            }
            if first > 0 {
                break;
            }
        }
    }
    'label: for start in 0..line.len() {
        let end = line.len() - start;
        let suffix = &line[..end];
        let c = suffix.chars().last().unwrap();
        if c > '0' && c <= '9' {
            last = c as usize - '0' as usize;
            break;
        } else if include_words {
            for (i, &word) in WORDS.iter().enumerate() {
                if suffix.ends_with(word) {
                    last = i + 1;
                    break 'label;
                }
            }
        }
    }

    10 * first + last
}

pub fn part1(data: &ParsedData) -> usize {
    data.iter().map(|line| solve(line, false)).sum()
}

pub fn part2(data: &ParsedData) -> usize {
    data.iter().map(|line| solve(line, true)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "5sixninesixnh";
        assert_eq!(part1(&parse_str(str_input)), 55);
        let str_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(&parse_str(str_input)), 142);
        let str_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(&parse_str(str_input)), 281);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/01.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 55538);
        assert_eq!(part2(&input), 54875);
    }
}
