use itertools::Itertools;
use std::{cmp::Reverse, collections::BinaryHeap, fs};
pub type ParsedData = Vec<Vec<u16>>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    for i in contents.trim_end().lines() {
        res.push(
            i.as_bytes()
                .iter()
                .map(|&b| (b - b'0') as u16)
                .collect_vec(),
        );
    }
    res
}

type Dir = (isize, isize);
static DIRS: [Dir; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

fn min_loss(grid: &ParsedData, mmin: isize, mmax: isize) -> usize {
    let (h, w) = (grid.len() as isize, grid[0].len() as isize);
    let data = (h * w) as usize;
    let mut cache = vec![usize::MAX; 2 * data];
    let end = (h - 1, w - 1);
    let mut fringe = BinaryHeap::with_capacity(if mmin == 1 { 1000 } else { 50000 });
    fringe.push(Reverse((0, 0, (0, 0), 0b1111 as u8)));
    let mut seen = [[0u8; 142]; 142];
    while let Some(Reverse((_heu, lost, (y, x), viable))) = fringe.pop() {
        if (y, x) == end {
            return lost;
        }
        let was = seen[y as usize][x as usize];
        if was & viable == viable {
            continue;
        }
        seen[y as usize][x as usize] |= viable;
        for ii in 0..4 {
            if (1 << ii) & viable == 0 || (1 << ii) & was > 0 {
                continue;
            }
            let (dx, dy) = DIRS[ii];
            let nxt_viable = 0b1111 - (1 << ii) - (1 << ((ii + 2) % 4));
            let mut nxt_lost = lost;
            for i in 1..=mmax {
                let ny = y + i * dy;
                let nx = x + i * dx;
                if 0 <= ny && ny < h && 0 <= nx && nx < w {
                    nxt_lost += grid[ny as usize][nx as usize] as usize;
                    if i < mmin {
                        continue;
                    }
                    let ci = (2 * (ny * h + nx) + ii as isize % 2) as usize;
                    if cache[ci] <= nxt_lost {
                        continue;
                    }
                    cache[ci] = nxt_lost;
                    let nxt_heu = nxt_lost + (h - ny + w - nx) as usize;
                    fringe.push(Reverse((nxt_heu, nxt_lost, (ny, nx), nxt_viable)));
                }
            }
        }
    }
    usize::MAX
}

pub fn part1(grid: &ParsedData) -> usize {
    min_loss(grid, 1, 3)
}

pub fn part2(grid: &ParsedData) -> usize {
    min_loss(grid, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(part1(&parse_str(str_input)), 102);
        assert_eq!(part2(&parse_str(str_input)), 94);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/17.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 785);
        assert_eq!(part2(&input), 922);
    }
}
