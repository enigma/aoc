use std::{
    fs,
    hash::{DefaultHasher, Hasher},
};

use itertools::Itertools;

type Grid = Vec<Vec<u8>>;
type Pos = (isize, isize);
pub type ParsedData = Grid;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    for line in contents.trim_end().lines() {
        res.push(line.as_bytes().iter().cloned().collect_vec());
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

fn roll(grid: &mut Grid, start: Pos, end: Pos, step: Pos) {
    let (mut y, mut x) = start;
    let (mut fy, mut fx) = start;
    let (dy, dx) = step;
    while (y, x) != end {
        let cur = grid[y as usize][x as usize];
        if cur == b'O' {
            grid[y as usize][x as usize] = b'.';
            grid[fy as usize][fx as usize] = b'O';
            fy += dy;
            fx += dx;
        } else if cur == b'#' {
            fy = y + dy;
            fx = x + dx;
        }
        y += dy;
        x += dx;
    }
}

enum Dir {
    North,
    West,
    South,
    East,
}

fn tilt(grid: &mut Grid, dir: Dir) {
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;
    match dir {
        Dir::North => {
            for x in 0..w {
                roll(grid, (0, x), (h, x), (1, 0));
            }
        }
        Dir::West => {
            for y in 0..h {
                roll(grid, (y, 0), (y, w), (0, 1));
            }
        }
        Dir::South => {
            for x in 0..w {
                roll(grid, (h - 1, x), (-1, x), (-1, 0));
            }
        }
        Dir::East => {
            for y in 0..h {
                roll(grid, (y, w - 1), (y, -1), (0, -1));
            }
        }
    }
}

fn cycle(grid: &mut Grid) {
    for dir in [Dir::North, Dir::West, Dir::South, Dir::East] {
        tilt(grid, dir);
    }
}

fn score(grid: &Grid) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&s| s == b'O').count() * (i + 1))
        .sum::<usize>()
}

pub fn part1(grid: &ParsedData) -> usize {
    let mut g = grid.clone();
    tilt(&mut g, Dir::North);
    score(&g)
}

const H: usize = 1024;

pub fn part2(grid: &ParsedData) -> usize {
    let mut cur = grid.clone();
    let mut seen = [0; H];
    let g = 1_000_000_000;
    for i in 0..g {
        let key = {
            let mut hasher = DefaultHasher::new();
            for row in &cur {
                hasher.write(&row);
            }
            hasher.finish() as usize % H
        };
        let e = seen[key];
        if e > 0 {
            let loop_size = e - i;
            for _ in 0..((g - i) % loop_size) % 4 {
                cycle(&mut cur);
            }
            break;
        }
        seen[key] = i;
        cycle(&mut cur);
    }
    score(&cur)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part1(&parse_str(str_input)), 136);
        assert_eq!(part2(&parse_str(str_input)), 64);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/14.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 109654);
        assert_eq!(part2(&input), 94828);
    }
}
