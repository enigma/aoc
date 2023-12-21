use std::{collections::VecDeque, fs};

const SIDE: usize = 131;
type Pos = (isize, isize);
type Grid = [[u8; SIDE]; SIDE];
pub type ParsedData = Grid;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut grid = [[0u8; SIDE]; SIDE];
    for (y, line) in contents.trim_end().lines().enumerate() {
        for (x, &v) in line.as_bytes().iter().enumerate() {
            grid[y][x] = if v == b'S' { b'.' } else { v }
        }
    }
    grid
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

fn count_from(grid: &Grid, start: Pos, goal: usize) -> usize {
    let side = grid.len() as isize;
    let (mut y, mut x) = start;
    y = (y + side * 202300) % side;
    x = (x + side * 202300) % side;
    let mut steps = [0, 0];
    let mut fringe = VecDeque::new();
    fringe.push_front((0, (y, x)));
    let mut seen = [[0u8; SIDE]; SIDE];
    while let Some((d, cur)) = fringe.pop_back() {
        let s = &mut seen[cur.0 as usize][cur.1 as usize];
        if *s > 0 {
            continue;
        } else {
            *s = 1;
        }
        if d > goal {
            continue;
        }
        let (y, x) = cur;
        steps[d % 2] += 1;
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let ny = y + dy;
            let nx = x + dx;
            if 0 <= ny
                && ny < side
                && 0 <= nx
                && nx < side
                && grid[ny as usize][nx as usize] != b'#'
            {
                fringe.push_front((d + 1, (ny, nx)));
            }
        }
    }
    steps[goal % 2]
}

pub fn part1(grid: &ParsedData) -> usize {
    let s = (grid.len() / 2) as isize;
    count_from(grid, (s, s), 64)
}

pub fn part2(grid: &ParsedData) -> usize {
    let s = (grid.len() / 2) as isize;
    let side = grid.len();
    let is = side as isize;

    let p2steps = 26501365;
    let gw = p2steps / side - 1;

    let mut p2 = 0;
    p2 += (((gw + 1) / 2) * 2).pow(2) * count_from(grid, (s, s), 2 * side);
    p2 += ((gw / 2) * 2 + 1).pow(2) * count_from(grid, (s, s), 2 * side + 1);
    p2 += count_from(grid, (s, 0), side - 1);
    p2 += count_from(grid, (s, is - 1), side - 1);
    p2 += count_from(grid, (0, s), side - 1);
    p2 += count_from(grid, (is - 1, s), side - 1);
    p2 += gw * count_from(grid, (is - 1, 0), 3 * side / 2 - 1);
    p2 += gw * count_from(grid, (is - 1, is - 1), 3 * side / 2 - 1);
    p2 += gw * count_from(grid, (0, 0), 3 * side / 2 - 1);
    p2 += gw * count_from(grid, (0, is - 1), 3 * side / 2 - 1);
    p2 += (gw + 1) * count_from(grid, (is - 1, 0), side / 2 - 1);
    p2 += (gw + 1) * count_from(grid, (is - 1, is - 1), side / 2 - 1);
    p2 += (gw + 1) * count_from(grid, (0, 0), side / 2 - 1);
    p2 += (gw + 1) * count_from(grid, (0, is - 1), side / 2 - 1);
    p2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual() {
        let path = &"../inputs/2023/21.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 3651);
        assert_eq!(part2(&input), 607334325965751);
    }
}
