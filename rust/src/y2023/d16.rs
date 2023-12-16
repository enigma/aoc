use std::{fs, vec};

use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

pub type ParsedData = Vec<Vec<u8>>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    for i in contents.trim_end().lines() {
        res.push(i.as_bytes().iter().cloned().collect_vec());
    }
    res
}

type Dir = (isize, isize);
type Pos = Dir;
static DIRS: [Dir; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
static EAST: Dir = DIRS[0];
static SOUTH: Dir = DIRS[1];
static WEST: Dir = DIRS[2];
static NORTH: Dir = DIRS[3];

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

const X: usize = 111;
fn energy(grid: &ParsedData, start: Pos, startd: usize) -> usize {
    let (h, w) = (grid.len(), grid[0].len());
    let mut fringe = Vec::with_capacity(h + w);
    let mut seen = [[0usize; X]; X];
    fringe.push((start, startd));
    let mut res = 0;
    while let Some(((y, x), dir)) = fringe.pop() {
        let key = 1usize << dir;
        let v = seen[y as usize][x as usize];
        if v & key > 0 {
            continue;
        }
        if v == 0 {
            res += 1;
        }
        seen[y as usize][x as usize] |= key;
        let mut ndirs = Vec::with_capacity(2);
        match (grid[y as usize][x as usize], DIRS[dir]) {
            (b'.', _) | (b'|', (_, 0)) | (b'-', (0, _)) => ndirs.push(DIRS[dir]),
            (b'|', (0, _)) => ndirs.extend([NORTH, SOUTH]),
            (b'-', (_, 0)) => ndirs.extend([WEST, EAST]),
            (b'\\', (dy, dx)) => ndirs.push((dx, dy)),
            (b'/', (dy, dx)) => ndirs.push((-dx, -dy)),
            _ => panic!("{:?}", (grid[y as usize][x as usize] as char, DIRS[dir])),
        }
        for (dy, dx) in ndirs {
            let ny = y + dy;
            let nx = x + dx;
            if 0 <= ny && ny < h as isize && 0 <= nx && nx < w as isize {
                fringe.push(((ny, nx), DIRS.iter().position(|&p| p == (dy, dx)).unwrap()));
            }
        }
    }
    res
}

pub fn part1(grid: &ParsedData) -> usize {
    energy(grid, (0, 0), 0)
}

pub fn part2(grid: &ParsedData) -> usize {
    let (h, w) = (grid.len() as isize, grid[0].len() as isize);

    (0..h)
        .into_par_iter()
        .map(|y| ((y, 0), 0))
        .chain((0..h).into_par_iter().map(|y| ((y, w - 1), 2)))
        .chain((0..w).into_par_iter().map(|x| ((0, x), 1)))
        .chain((0..w).into_par_iter().map(|x| ((h - 1, x), 3)))
        .into_par_iter()
        .map(|(start, dir)| energy(grid, start, dir))
        .max()
        .unwrap()

    // let ymax = (0..h)
    //     .into_par_iter()
    //     .map(|y| energy(grid, (y, 0), 0).max(energy(grid, (y, w - 1), 2)))
    //     .max()
    //     .unwrap();

    // let xmax = (0..w)
    //     .into_par_iter()
    //     .map(|x| energy(grid, (0, x), 1).max(energy(grid, (h - 1, x), 3)))
    //     .max()
    //     .unwrap();
    // ymax.max(xmax)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(part1(&parse_str(str_input)), 46);
        assert_eq!(part2(&parse_str(str_input)), 51);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/16.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 7434);
        assert_eq!(part2(&input), 8183);
    }
}
