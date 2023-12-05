use std::fs;

use hashbrown::HashMap;
use itertools::Itertools;

type T = usize;
pub type ParsedData = (T, T);

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let lines = contents.trim().lines().collect_vec();
    let dim = (lines.len(), lines[0].len());
    let mut part1 = 0;
    let mut part2 = HashMap::new();
    for (y, &line) in lines.iter().enumerate() {
        let mut x = 0;
        while x < dim.1 {
            let digits = line[x..]
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(dim.1 - x);
            if digits > 0 {
                let parsed = line[x..x + digits].parse::<T>().unwrap();

                let mut near_symbol = false;
                for ny in (if y > 0 { y - 1 } else { y })..(y + 2).min(dim.0) {
                    for nx in (if x > 0 { x - 1 } else { x })..(x + digits + 1).min(dim.1) {
                        let c = lines[ny].as_bytes()[nx];
                        if c == b'.' || c.is_ascii_digit() {
                            continue;
                        }
                        near_symbol = true;
                        if c == b'*' {
                            let e = part2.entry((ny, nx)).or_insert((0, 1));
                            e.0 += 1;
                            e.1 *= parsed;
                        }
                    }
                }
                if near_symbol {
                    part1 += parsed;
                }
            }
            x += digits + 1;
        }
    }
    (
        part1,
        part2.values().filter(|(c, _)| *c > 1).map(|(_, p)| p).sum(),
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

pub fn part1(data: &ParsedData) -> usize {
    data.0
}

pub fn part2(data: &ParsedData) -> usize {
    data.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(&parse_str(str_input)), 4361);
        assert_eq!(part2(&parse_str(str_input)), 467835);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/03.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 521515);
        assert_eq!(part2(&input), 69527306);
    }
}
