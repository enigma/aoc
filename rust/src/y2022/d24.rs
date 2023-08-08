use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::{
    cmp::{Ordering, Reverse},
    fs,
};

use hashbrown::HashSet;

type D = u16;
type Grid = [[char; MAX]; MAX];
type ParsedData = (Grid, (D, D), RefCell<Option<D>>);

const MAX: usize = 200;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut maxx = D::MIN;
    let mut maxy = D::MIN;
    let mut grid = [[' '; MAX]; MAX];
    contents.trim().lines().enumerate().for_each(|(y, line)| {
        for (x, val) in line.chars().enumerate() {
            if val == '#' {
                continue;
            }
            maxx = maxx.max(x as D);
            maxy = maxy.max(y as D);
            grid[y][x] = val;
        }
    });
    (grid, (maxy, maxx), RefCell::new(None))
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

#[inline]
fn neighbors(from: (D, D), maxy: D, maxx: D) -> Vec<(D, D)> {
    let mut res = Vec::with_capacity(5);
    res.push(from);

    let (y, x) = from;
    if y > 0 {
        if y > 1 || x == 1 {
            res.push((y - 1, x));
        }
    }
    if y < maxy {
        if y < maxy - 1 || x == maxx {
            res.push((y + 1, x));
        }
    }
    if x > 1 && y < maxy {
        res.push((y, x - 1));
    }
    if x < maxx && y > 0 && y < maxy {
        res.push((y, x + 1));
    }
    res
}

#[inline]
fn is_free(grid: Grid, pos: (D, D), mx: (D, D), epoch: D) -> bool {
    let tl = (1, 1);
    let br = ((mx.0 - 1) as usize, mx.1 as usize);
    let width = 1 + br.1 - tl.1;
    let height = 1 + br.0 - tl.0;
    let epoch = epoch as usize;

    let y = pos.0 as usize;
    let x = pos.1 as usize;

    let k = 50;

    let a0 = grid[y][tl.1 + ((width * k + x - tl.1 + epoch) % width)] != '<';
    let a1 = grid[y][tl.1 + ((width * k + x - tl.1 - epoch) % width)] != '>';
    let a2 = grid[tl.0 + ((height * k + y - tl.0 + epoch) % height)][x] != '^';
    let a3 = grid[tl.0 + ((height * k + y - tl.0 - epoch) % height)][x] != 'v';
    a0 && a1 && a2 && a3
}

#[inline]
fn heu(src: (D, D), dst: (D, D)) -> D {
    src.0.abs_diff(dst.0) + src.1.abs_diff(dst.1)
}

#[inline]
fn shortest_path(grid: &Grid, mx: (D, D), start: (D, D), goal: (D, D), epoch: D) -> D {
    let (maxy, maxx) = mx;
    let mut fringe = BinaryHeap::with_capacity(3_000);
    fringe.push(Priority(Reverse(heu(start, goal)), (epoch, start)));
    let mut seen = HashSet::with_capacity(80_000);
    while let Some(Priority(_, (elapsed, pos))) = fringe.pop() {
        if !seen.insert((elapsed, pos)) {
            continue;
        }
        if pos == goal {
            return elapsed;
        }
        for nxt in neighbors(pos, maxy, maxx) {
            if nxt == start || nxt == goal || is_free(*grid, nxt, mx, elapsed + 1) {
                let heu = elapsed + 1 + heu(nxt, goal);
                fringe.push(Priority(Reverse(heu), (elapsed + 1, nxt)));
            }
        }
    }
    unreachable!("Could not find a solution!");
}

pub fn part1(data: &ParsedData) -> usize {
    let &(grid, (maxy, maxx), _) = data;
    let result = shortest_path(&grid, (maxy, maxx), (0, 1), (maxy, maxx), 0);

    // Cache for part2
    let mut cache = data.2.borrow_mut();
    *cache.get_or_insert(result) as usize
}

pub fn part2(data: &ParsedData) -> usize {
    let &(grid, (maxy, maxx), _) = data;
    let part1 = &data.2.borrow();

    let start = (0, 1);
    let goal = (maxy, maxx);
    let d1 = part1.unwrap_or_else(|| shortest_path(&grid, (maxy, maxx), start, goal, 0));
    let d2 = shortest_path(&grid, (maxy, maxx), goal, start, d1);
    shortest_path(&grid, (maxy, maxx), start, goal, d2) as usize
}

#[derive(Debug, Copy, Clone)]
struct Priority<P, T>(P, T);

impl<P: Ord + Eq, T> Ord for Priority<P, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<P: Ord + Eq, T> PartialOrd for Priority<P, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: Eq, T> PartialEq for Priority<P, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P: Eq, T> Eq for Priority<P, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
        let parsed = &parse_str(str_input);
        assert_eq!(part1(&parsed), 18);
        assert_eq!(part2(&parsed), 54);
    }

    #[test]
    fn actual() {
        let path = &"inputs/2022/24.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 301);
        assert_eq!(part2(&input), 859);
    }
}
