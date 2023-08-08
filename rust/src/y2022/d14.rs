use sscanf::sscanf;
use std::fs;

pub const WIDTH: usize = 700;
pub const HEIGHT: usize = 200;

pub type ParsedData = (Vec<Vec<bool>>, usize);

#[inline]
fn way_iter(from: (isize, isize), to: (isize, isize)) -> impl Iterator<Item = (usize, usize)> {
    let dx = to.0.cmp(&from.0) as isize;
    let dy = to.1.cmp(&from.1) as isize;

    let steps = 1 + (to.0 - from.0).abs().max((to.1 - from.1).abs()) as usize;

    (0..)
        .map(move |i| (from.0 + i * dx, from.1 + i * dy))
        .map(move |(x, y)| (x as usize, y as usize))
        .take(steps)
}

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut grid = vec![vec![false; WIDTH]; HEIGHT];
    let mut max_y = 0;
    contents.trim().lines().for_each(|line| {
        let mut waypoints = Vec::with_capacity(30);
        line.split(" -> ")
            .for_each(|piece| waypoints.push(sscanf!(piece, "{isize},{isize}").unwrap()));
        for i in 0..waypoints.len() - 1 {
            for (x, y) in &mut way_iter(waypoints[i], waypoints[i + 1]) {
                grid[y][x] = true;
                max_y = max_y.max(y);
            }
        }
    });
    (grid, max_y)
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn solve((grid, max_y): &ParsedData, hard_bottom: bool) -> usize {
    let mut grid = grid.clone();
    let mut dropped = 0;
    let source = (500isize, 0isize);
    let dirs = [(0isize, 1isize), (-1, 1), (1, 1)].as_slice();
    let mut stack = Vec::with_capacity(200);
    stack.push((source, dirs));
    while let Some(((x, y), attempt_left)) = stack.pop() {
        if y as usize > *max_y {
            if hard_bottom {
                continue;
            } else {
                return dropped;
            }
        }
        if grid[y as usize][x as usize] {
            continue;
        }
        if let Some((dx, dy)) = attempt_left.first() {
            stack.push(((x, y), &attempt_left[1..]));
            stack.push(((x + dx, y + dy), &dirs));
        } else {
            grid[y as usize][x as usize] = true;
            dropped += 1;
            if (x, y) == source {
                return dropped;
            }
        }
    }
    dropped
}

// y2022 d14 part1 full    time:   [355.54 µs 356.55 µs 357.60 µs]
pub fn part1(data: &ParsedData) -> usize {
    solve(data, false)
}

// y2022 d14 part2 full    time:   [900.36 µs 903.62 µs 906.80 µs]
pub fn part2((grid, max_y): &ParsedData) -> usize {
    let param: ParsedData = (grid.to_vec(), max_y + 1);
    solve(&param, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waypoints() {
        let res: Vec<(usize, usize)> = way_iter((2, 0), (0, 0)).collect();
        let mut exp = vec![];
        exp.push((2usize, 0usize));
        exp.push((1usize, 0usize));
        exp.push((0usize, 0usize));
        assert_eq!(res, exp);

        let res: Vec<(usize, usize)> = way_iter((529, 113), (529, 110)).collect();
        let mut exp = vec![];
        exp.push((529usize, 113usize));
        exp.push((529usize, 112usize));
        exp.push((529usize, 111usize));
        exp.push((529usize, 110usize));
        assert_eq!(res, exp);
    }

    #[test]
    fn example() {
        let str_input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(part1(&parse_str(str_input)), 24);
        assert_eq!(part2(&parse_str(str_input)), 93);
    }

    #[test]
    fn actual() {
        let path = &"inputs/2022/14.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 779);
        assert_eq!(part2(&input), 27426);
    }
}
