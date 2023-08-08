use std::{collections::VecDeque, fs};

#[inline]
fn actual_parse(contents: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut res = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    contents.trim().lines().enumerate().for_each(|(y, line)| {
        let mut row = vec![];
        line.chars().enumerate().for_each(|(x, c)| {
            row.push(match c {
                'S' => {
                    start = (y, x);
                    0
                }
                'E' => {
                    end = (y, x);
                    b'z' - b'a'
                }
                d => d as u8 - b'a',
            });
        });
        res.push(row);
    });
    (res, start, end)
}

const COLS: usize = 115;
const ROWS: usize = 50;
pub type ParsedData = ([[usize; COLS]; ROWS], (usize, usize), Vec<Vec<u8>>);

#[inline]
fn neighbors(grid: &Vec<Vec<u8>>, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;
    let (y, x) = pos;

    [(0 as isize, -1 as isize), (0, 1), (-1, 0), (1, 0)]
        .iter()
        .filter(move |(dy, dx)| {
            (*dy >= 0 || y > 0)
                && (*dx >= 0 || x > 0)
                && (y as isize + *dy < h)
                && (x as isize + *dx < w)
        })
        .map(move |(dy, dx)| ((y as isize + dy) as usize, (x as isize + dx) as usize))
}

fn distances(grid: &Vec<Vec<u8>>, end: (usize, usize)) -> [[usize; COLS]; ROWS] {
    let mut dists = [[usize::MAX; COLS]; ROWS];
    dists[end.0][end.1] = 0;
    let mut fringe = VecDeque::new();
    let mut seen = [[false; COLS]; ROWS];
    fringe.push_back(end);
    while let Some(cur) = fringe.pop_back() {
        let (y, x) = cur;
        if seen[y][x] {
            continue;
        }
        seen[y][x] = true;
        for (ny, nx) in &mut neighbors(&grid, cur) {
            if seen[ny][nx] || grid[ny][nx] + 1 < grid[y][x] {
                continue;
            }
            dists[ny][nx] = dists[y][x] + 1;
            fringe.push_front((ny, nx));
        }
    }
    dists
}

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let (grid, start, end) = actual_parse(contents);
    (distances(&grid, end), start, grid)
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

// y2022 d12 part1 full    time:   [71.971 µs 72.216 µs 72.489 µs]
pub fn part1(data: &ParsedData) -> usize {
    let (dists, (y, x), _) = data;
    dists[*y][*x]
}

// y2022 d12 part2 full    time:   [77.131 µs 77.520 µs 77.937 µs]
pub fn part2(data: &ParsedData) -> usize {
    let (dists, _, grid) = data;
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| ((y, x), v)))
        .filter(|(_, v)| **v == 0)
        .map(|((y, x), _)| dists[y][x])
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(part1(&parse_str(str_input)), 31);
        assert_eq!(part2(&parse_str(str_input)), 29);
    }

    #[test]
    fn actual() {
        let path = &"inputs/2022/12.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 380);
        assert_eq!(part2(&input), 375);
    }
}
