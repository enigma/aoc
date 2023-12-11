use std::fs;

use hashbrown::HashSet;
use itertools::Itertools;

type T = usize;
type Pos = (T, T);
type Missing = Vec<T>;
pub type ParsedData = (Vec<Pos>, Missing, Missing);

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, line) in contents.trim_end().lines().enumerate() {
        for (x, b) in line.as_bytes().iter().enumerate() {
            if b'#' == *b {
                res.push((y, x));
                rows.insert(y);
                cols.insert(x);
                max_y = max_y.max(y);
                max_x = max_x.max(x);
            }
        }
    }
    (
        res,
        (0..max_y).filter(|&i| !rows.contains(&i)).collect_vec(),
        (0..max_x).filter(|&i| !cols.contains(&i)).collect_vec(),
    )
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

fn solve(data: &ParsedData, factor: usize) -> usize {
    let (grid, mrows, mcols) = data;
    let egrid = grid
        .iter()
        .map(|&(y, x)| {
            (
                y + mrows.iter().take_while(|&&i| i < y).count() * (factor - 1),
                x + mcols.iter().take_while(|&&i| i < x).count() * (factor - 1),
            )
        })
        .collect_vec();
    let mut res = 0;
    for (i, &(y1, x1)) in egrid.iter().enumerate() {
        for &(y2, x2) in egrid.iter().skip(i + 1) {
            res += y1.abs_diff(y2) + x1.abs_diff(x2);
        }
    }
    res
}

pub fn part1(data: &ParsedData) -> usize {
    solve(data, 2)
}

pub fn part2(data: &ParsedData) -> usize {
    solve(data, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part1(&parse_str(str_input)), 374);
        // assert_eq!(part2(&parse_str(str_input)), 2);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/11.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 10494813);
        assert_eq!(part2(&input), 840988812853);
    }
}
