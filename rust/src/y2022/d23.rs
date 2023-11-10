use hashbrown::{HashMap, HashSet};
use std::{collections::VecDeque, fs};

type I = isize;
type Rules = [((I, I), [(I, I); 3]); 4];
type Delta = (I, I);
type ParsedData = (HashSet<(I, I)>, Rules, Vec<Delta>);

const DX: [I; 12] = [-1, 0, 1, -1, 0, 1, -1, -1, -1, 1, 1, 1];
const DY: [I; 12] = [-1, -1, -1, 1, 1, 1, -1, 0, 1, -1, 0, 1];

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = HashSet::with_capacity(3000);
    contents
        .trim_end()
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .for_each(|(x, _)| {
                    res.insert((x as I, y as I));
                })
        });
    let mut ds = [(0isize, 0isize); 12];
    DX.iter()
        .zip(DY)
        .enumerate()
        .for_each(|(i, f)| ds[i] = (*f.0, f.1));
    let mut deltas = HashSet::with_capacity(8);
    ds.iter().for_each(|(x, y)| {
        deltas.insert((*x, *y));
    });
    let deltas: Vec<Delta> = deltas.drain().collect();
    let mut rules: Rules = [
        ((0, -1), [(0, 0); 3]),
        ((0, 1), [(0, 0); 3]),
        ((-1, 0), [(0, 0); 3]),
        ((1, 0), [(0, 0); 3]),
    ];

    rules[0].1.copy_from_slice(&ds[..3]);
    rules[1].1.copy_from_slice(&ds[3..6]);
    rules[2].1.copy_from_slice(&ds[6..9]);
    rules[3].1.copy_from_slice(&ds[9..]);

    (res, rules, deltas)
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

const C: usize = 50;
const R: usize = 200;
const CAP: usize = 3000;
const ITER_OFFSET: u16 = 1;

// part2 noparsing 100ms
// put grid into a 2d array, do everything in place
pub fn solve(data: &ParsedData, limit: usize) -> usize {
    let mut grid = [[0; R]; R];
    let mut points = Vec::with_capacity(2 * CAP);
    for &(x, y) in &data.0 {
        points.push((x + C as I, y + C as I));
        grid[x as usize + C][y as usize + C] = ITER_OFFSET;
    }
    let mut last_inserted = points.len();
    let mut rules: VecDeque<_> = data.1.iter().collect();
    let deltas = &data.2;
    let mut iter = ITER_OFFSET;
    loop {
        iter += 1;
        let mut attempts = HashMap::new();
        let mut moved = false;

        for i in 0..last_inserted {
            let (x, y) = points[i];
            if !deltas
                .iter()
                .any(|&(dx, dy)| grid[(x + dx) as usize][(y + dy) as usize] >= iter - 1)
            {
                points.push((x, y));
                continue;
            }
            'label: {
                for ((dest_dx, dest_dy), delta) in &rules {
                    if !delta
                        .iter()
                        .any(|(dx, dy)| grid[(x + dx) as usize][(y + dy) as usize] >= iter - 1)
                    {
                        attempts
                            .entry((x + dest_dx, y + dest_dy))
                            .or_insert(vec![])
                            .push((x, y));
                        break 'label;
                    }
                }
                points.push((x, y));
            }
        }
        for (dest, sources) in &attempts {
            if sources.len() == 1 {
                moved = true;
                points.push(*dest);
            } else {
                sources.iter().for_each(|&i| {
                    points.push(i);
                });
            }
        }

        let new_points = points.len() - last_inserted;
        for i in 0..last_inserted.min(new_points) {
            points[i] = points[last_inserted + new_points - i - 1];
        }
        points.truncate(new_points);

        for &(x, y) in points.iter() {
            grid[x as usize][y as usize] = iter;
        }
        rules.rotate_left(1);
        if !moved {
            return (iter - ITER_OFFSET) as usize;
        }
        if limit > 0 && (iter - ITER_OFFSET) as usize == limit {
            let (mut minx, mut miny) = (I::MAX, I::MAX);
            let (mut maxx, mut maxy) = (I::MIN, I::MIN);
            for &(x, y) in points.iter() {
                minx = minx.min(x - C as I);
                maxx = maxx.max(x - C as I);
                miny = miny.min(y - C as I);
                maxy = maxy.max(y - C as I);
            }
            return (maxy - miny + 1) as usize * (maxx - minx + 1) as usize - new_points;
        }
        last_inserted = new_points;
    }
}

pub fn part1(data: &ParsedData) -> usize {
    solve(data, 10)
}

pub fn part2(data: &ParsedData) -> usize {
    solve(data, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        assert_eq!(part1(&parse_str(str_input)), 110);
        assert_eq!(part2(&parse_str(str_input)), 20);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/23.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 3800);
        assert_eq!(part2(&input), 916);
    }
}

// more naive approach, grid = HashSet, new set each iteration
// part2 noparsing 250ms
pub fn _solve(data: &ParsedData, limit: usize) -> usize {
    let mut grid = data.0.clone();
    let mut rules: VecDeque<_> = data.1.iter().collect();
    let deltas = &data.2;
    let mut iter = 0;
    loop {
        iter += 1;
        let mut attempts = HashMap::new();
        let mut new_grid = HashSet::with_capacity(500); // TODO?
        let mut moved = false;
        for &(x, y) in &grid {
            if !deltas
                .iter()
                .any(|(dx, dy)| grid.contains(&(x + dx, y + dy)))
            {
                new_grid.insert((x, y));
                continue;
            }
            'label: {
                for ((dest_dx, dest_dy), delta) in &rules {
                    if !delta
                        .iter()
                        .any(|(dx, dy)| grid.contains(&(x + dx, y + dy)))
                    {
                        attempts
                            .entry((x + dest_dx, y + dest_dy))
                            .or_insert(vec![])
                            .push((x, y));
                        break 'label;
                    }
                }
                new_grid.insert((x, y));
            }
        }
        for (dest, sources) in &attempts {
            if sources.len() == 1 {
                moved = true;
                new_grid.insert(*dest);
            } else {
                sources.iter().for_each(|&i| {
                    new_grid.insert(i);
                });
            }
        }
        grid = new_grid;
        rules.rotate_left(1);
        if !moved {
            return iter;
        }
        if limit > 0 && iter == limit {
            let (mut minx, mut miny) = (I::MAX, I::MAX);
            let (mut maxx, mut maxy) = (I::MIN, I::MIN);
            for &(x, y) in grid.iter() {
                minx = minx.min(x);
                maxx = maxx.max(x);
                miny = miny.min(y);
                maxy = maxy.max(y);
            }
            return (maxy - miny + 1) as usize * (maxx - minx + 1) as usize - grid.len();
        }
    }
}
