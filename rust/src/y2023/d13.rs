use std::fs;

use itertools::Itertools;

type Pattern = Vec<Vec<bool>>;
pub type ParsedData = Vec<Pattern>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    contents
        .trim_end()
        .split("\n\n")
        .map(|spat| {
            spat.lines()
                .map(|l| l.as_bytes().iter().map(|&b| b == b'#').collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

fn score(pat: &Pattern, part2: bool) -> usize {
    let (h, w) = (pat.len(), pat[0].len());
    let threshold = if part2 { 1 } else { 0 };
    for x in 1..w {
        if (0..x)
            .rev()
            .zip(x..w)
            .map(|(xa, xb)| (0..h).filter(|&y| pat[y][xa] == pat[y][xb]).count())
            .sum::<usize>()
            .abs_diff(h * x.min(w - x))
            == threshold
        {
            return x;
        }
    }
    for y in 1..h {
        if (0..y)
            .rev()
            .zip(y..h)
            .map(|(ya, yb)| (0..w).filter(|&x| pat[ya][x] == pat[yb][x]).count())
            .sum::<usize>()
            .abs_diff(w * y.min(h - y))
            == threshold
        {
            return y * 100;
        }
    }
    0
}

pub fn part1(patterns: &ParsedData) -> usize {
    patterns.iter().map(|p| score(p, false)).sum()
}

pub fn part2(patterns: &ParsedData) -> usize {
    patterns.iter().map(|p| score(p, true)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part1(&parse_str(str_input)), 405);
        assert_eq!(part2(&parse_str(str_input)), 400);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/13.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 33520);
        assert_eq!(part2(&input), 34824);
    }
}
