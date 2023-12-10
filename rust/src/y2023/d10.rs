use std::fs;

use itertools::Itertools;

type Pos = (usize, usize);
type Delta = (isize, isize);
pub type ParsedData = (usize, usize);

const LX: usize = 141;
const LY: usize = 141;
type Pipe = [[bool; LX]; LY];

fn expand(grid: &Vec<Vec<u8>>, pos: Pos) -> impl Iterator<Item = Delta> + '_ {
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;
    let (y, x) = (pos.0 as isize, pos.1 as isize);

    [(0isize, -1isize), (0, 1), (-1, 0), (1, 0)]
        .iter()
        .cloned()
        .filter(move |&(dy, dx)| {
            (dy >= 0 || y > 0) && (dx >= 0 || x > 0) && (y + dy < h) && (x + dx < w)
        })
}

fn neigh(grid: &Vec<Vec<u8>>, pos: Pos) -> impl Iterator<Item = Delta> + '_ {
    expand(grid, pos).filter(move |&(dy, dx)| match (grid[pos.0][pos.1], dy, dx) {
        (b'.', _, _) => true,
        (b'-', 0, _) => true,
        (b'|', _, 0) => true,
        (b'L', -1, 0) | (b'L', 0, 1) => true,
        (b'J', -1, 0) | (b'J', 0, -1) => true,
        (b'F', 1, 0) | (b'F', 0, 1) => true,
        (b'7', 1, 0) | (b'7', 0, -1) => true,
        (b'S', _, _) => true,
        _ => false,
    })
    //.map(move |&(dy, dx)| ((y + dy) as usize, (x + dx) as usize))
}

fn track(raw: &Vec<Vec<u8>>, cur: Pos, d: Delta, left: &mut Pipe, right: &mut Pipe) -> isize {
    let (y, x) = (cur.0 as isize, cur.1 as isize);
    let (dy, dx) = d;
    match raw[cur.0][cur.1] {
        b'|' => {
            if x + dy > 0 {
                left[y as usize][(x + dy) as usize] = true;
            }

            if x - dy > 0 {
                right[y as usize][(x - dy) as usize] = true;
            }
        }
        b'-' => {
            if y - dx > 0 {
                left[(y - dx) as usize][x as usize] = true;
            }
            if y + dx > 0 {
                right[(y + dx) as usize][x as usize] = true;
            }
        }
        b'J' => {
            let side = if dy == 0 { left } else { right };
            side[y as usize][(x + 1) as usize] = true;
            side[(y + 1) as usize][x as usize] = true;
            return if dy == 0 { 1 } else { -1 };
        }
        b'L' => {
            let side = if dy == -1 { left } else { right };
            if x > 0 {
                side[y as usize][(x - 1) as usize] = true;
            }
            side[(y + 1) as usize][x as usize] = true;
            return if dx == 1 { 1 } else { -1 };
        }
        b'7' => {
            let side = if dy == 1 { left } else { right };
            if y > 0 {
                side[(y - 1) as usize][x as usize] = true;
            }
            side[y as usize][(x + 1) as usize] = true;
            return if dy == 1 { 1 } else { -1 };
        }
        b'F' => {
            let side = if dy == 0 { left } else { right };
            side[y as usize][(x + 1) as usize] = true;
            if y > 0 {
                side[(y - 1) as usize][x as usize] = true;
            }
            return if dx == 1 { 1 } else { -1 };
        }
        b'S' => {}
        _ => panic!("WTF?"),
    }
    0
}

fn fill(parsed: &Vec<Vec<u8>>, pipe: &Pipe, f: &Pipe) -> usize {
    let mut seen = [[false; LX]; LY];
    let mut fringe = (0..parsed.len())
        .cartesian_product(0..parsed[0].len())
        .filter(|&(y, x)| f[y][x])
        .collect_vec();
    let mut res = 0;
    while let Some(p) = fringe.pop() {
        if p.0 >= parsed.len() || p.1 >= parsed[0].len() {
            continue;
        }
        if seen[p.0][p.1] || pipe[p.0][p.1] {
            continue;
        }
        seen[p.0][p.1] = true;
        res += 1;
        for (dy, dx) in expand(parsed, p) {
            fringe.push(((p.0 as isize + dy) as usize, (p.1 as isize + dx) as usize));
        }
    }
    res
}

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut parsed = vec![];
    let mut start = (0, 0);
    for (y, row) in contents.trim_end().lines().enumerate() {
        let prow = row.as_bytes().iter().cloned().collect_vec();
        if let Some(x) = prow.iter().position(|&p| p == b'S') {
            start = (y, x);
        }
        parsed.push(prow);
    }
    let mut pipe = [[false; LX]; LY];
    let mut cur = start;
    let mut d = neigh(&parsed, cur)
        .filter(|&(ndy, ndx)| {
            let n = (
                (cur.0 as isize + ndy) as usize,
                (cur.1 as isize + ndx) as usize,
            );
            neigh(&parsed, n).any(|m| m == (-ndy, -ndx))
        })
        .next()
        .unwrap();
    let mut left = [[false; LX]; LY];
    let mut right = [[false; LX]; LY];
    let mut part1 = 0;
    let mut turns = 0;
    while !pipe[cur.0][cur.1] {
        pipe[cur.0][cur.1] = true;
        part1 += 1;
        turns += track(&parsed, cur, d, &mut left, &mut right);
        cur.0 = (cur.0 as isize + d.0) as usize;
        cur.1 = (cur.1 as isize + d.1) as usize;
        d = neigh(&parsed, cur)
            .filter(|&i| i != (-d.0, -d.1))
            .next()
            .unwrap();
    }
    (
        part1 / 2,
        fill(
            &parsed,
            &pipe,
            if turns < 0 { &mut right } else { &mut left },
        ),
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

pub fn part1(x: &ParsedData) -> usize {
    x.0
}

pub fn part2(x: &ParsedData) -> usize {
    x.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(part1(&parse_str(str_input)), 8);
        // assert_eq!(part2(&parse_str(str_input)), 2);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/10.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 7086);
        assert_eq!(part2(&input), 317);
    }
}
