use std::fs;

use hashbrown::HashMap;

type Coord = isize;
type Val = char;

#[derive(Debug)]
pub enum Dir {
    Straight(Coord),
    L,
    R,
}

pub type ParsedData = (
    [[Val; SIDE]; SIDE],
    Vec<Dir>,
    HashMap<Coord, (Coord, Coord)>,
    HashMap<Coord, (Coord, Coord)>,
);

const SIDE: usize = 201;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let (sgrid, letter) = contents.trim_end().split_once("\n\n").unwrap();
    let mut grid = [[' '; SIDE]; SIDE];
    let mut xrange = HashMap::new();
    let mut yrange = HashMap::new();
    for (y, row) in sgrid.lines().enumerate() {
        let y = y as Coord;
        let (mut x_min, mut x_max) = (Coord::MAX, Coord::MIN);
        for (x, val) in row.chars().enumerate() {
            if val == ' ' {
                continue;
            }
            let x = x as Coord;
            x_min = x_min.min(x);
            x_max = x_max.max(x);
            let cur = (y, y + 1);
            let e = yrange.entry(x).or_insert(cur);
            e.0 = e.0.min(cur.0);
            e.1 = e.1.max(cur.1);
            grid[y as usize][x as usize] = val;
        }
        if x_min != Coord::MAX {
            xrange.insert(y, (x_min, x_max + 1));
        }
    }

    let mut path = vec![];
    let mut letter = letter.trim().chars().peekable();

    let mut q = 0;
    while let Some(_) = letter.peek() {
        q = 0;
        while let Some(c) = letter.peek() {
            if let Some(d) = c.to_digit(10) {
                q = q * 10 + d as Coord;
                letter.next();
            } else {
                path.push(Dir::Straight(q as Coord));
                path.push(match c {
                    'R' => Dir::R,
                    'L' => Dir::L,
                    _ => unreachable!(),
                });
                letter.next();
                break;
            }
        }
    }
    path.push(Dir::Straight(q));
    (grid, path, xrange, yrange)
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn inrange(rng: (Coord, Coord), i: Coord) -> Coord {
    let (a, b) = rng;
    if i < a {
        return b - 1;
    };
    if i >= b {
        a
    } else {
        i
    }
}

pub fn part1(data: &ParsedData) -> usize {
    let (grid, letter, x_ranges, y_ranges) = data;
    let (mut y, mut x) = (0 as Coord, x_ranges.get(&0).unwrap().0 as Coord);
    let (mut dy, mut dx) = (0, 1);
    for d in letter {
        match d {
            Dir::Straight(steps) => {
                for _ in 0..*steps {
                    let (mut ny, mut nx) = (y + dy, x + dx);
                    if ny < 0 || nx < 0 || grid[ny as usize][nx as usize] == ' ' {
                        let nyv = y_ranges.get(&nx);
                        if dy != 0 && nyv.is_some() {
                            ny = inrange(*nyv.unwrap(), ny);
                        }
                        let nxv = x_ranges.get(&ny);
                        if dx != 0 && nxv.is_some() {
                            nx = inrange(*nxv.unwrap(), nx);
                        }
                    }
                    if nx >= 0 && ny >= 0 && '.' == grid[ny as usize][nx as usize] {
                        (y, x) = (ny, nx);
                    } else {
                        break;
                    }
                }
            }
            Dir::R => (dy, dx) = (dx, -dy),
            Dir::L => (dy, dx) = (-dx, dy),
        }
    }

    (1000 * (y + 1)
        + 4 * (x + 1)
        + match (dy, dx) {
            (0, 1) => 0,
            (1, 0) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            _ => unreachable!(),
        }) as usize
}

const FACE: Coord = 50;

pub fn part2(data: &ParsedData) -> usize {
    let (grid, letter, x_ranges, _yr) = data;
    let (mut y, mut x) = (0 as Coord, x_ranges.get(&0).unwrap().0 as Coord);
    let (mut dy, mut dx) = (0, 1);
    for d in letter {
        match d {
            Dir::Straight(steps) => {
                for _ in 0..*steps {
                    let original_delta = (dy, dx);
                    let (mut ny, mut nx) = (y + dy, x + dx);
                    if ny < 0 || nx < 0 || grid[ny as usize][nx as usize] == ' ' {
                        let fy = y / FACE;
                        let fx = x / FACE;
                        match ((fy, fx), (dy, dx)) {
                            // (0, 1)
                            ((0, 1), (0, -1)) => (ny, nx, dy, dx) = (3 * FACE - y - 1, 0, 0, 1),
                            ((0, 1), (-1, 0)) => (ny, nx, dy, dx) = (3 * FACE + x % FACE, 0, 0, 1),
                            // (0, 2)
                            ((0, 2), (-1, 0)) => (ny, nx, dy, dx) = (4 * FACE - 1, x % FACE, -1, 0),
                            ((0, 2), (0, 1)) => {
                                (ny, nx, dy, dx) = (3 * FACE - y - 1, 2 * FACE - 1, 0, -1)
                            }
                            ((0, 2), (1, 0)) => {
                                (ny, nx, dy, dx) = (FACE + x % FACE, 2 * FACE - 1, 0, -1)
                            }
                            // (1,1)
                            ((1, 1), (0, -1)) => (ny, nx, dy, dx) = (2 * FACE, y % FACE, 1, 0),
                            ((1, 1), (0, 1)) => {
                                (ny, nx, dy, dx) = (FACE - 1, 2 * FACE + y % FACE, -1, 0);
                                assert_eq!((ny / FACE, nx / FACE), (0, 2), "{:?}", (ny, nx));
                            }
                            // (2,0)
                            ((2, 0), (-1, 0)) => (ny, nx, dy, dx) = (FACE + x % FACE, FACE, 0, 1),
                            ((2, 0), (0, -1)) => {
                                (ny, nx, dy, dx) = (FACE - (y % FACE) - 1, FACE, 0, 1)
                            }
                            // (2,1)
                            ((2, 1), (0, 1)) => {
                                (ny, nx, dy, dx) = (FACE - (y % FACE) - 1, 3 * FACE - 1, 0, -1)
                            }
                            ((2, 1), (1, 0)) => {
                                (ny, nx, dy, dx) = (3 * FACE + (x % FACE), FACE - 1, 0, -1)
                            }
                            // (3, 0)
                            ((3, 0), (0, -1)) => (ny, nx, dy, dx) = (0, FACE + y % FACE, 1, 0),
                            ((3, 0), (0, 1)) => {
                                (ny, nx, dy, dx) = (3 * FACE - 1, FACE + y % FACE, -1, 0)
                            }
                            ((3, 0), (1, 0)) => (ny, nx, dy, dx) = (0, 2 * FACE + x % FACE, 1, 0),
                            _ => unimplemented!("WTF? {:?} {:?}", (fy, fx), (dy, dx)),
                        }
                    }
                    if nx >= 0 && ny >= 0 && '.' == grid[ny as usize][nx as usize] {
                        (y, x) = (ny, nx);
                    } else {
                        (dy, dx) = original_delta;
                        break;
                    }
                }
            }
            Dir::R => (dy, dx) = (dx, -dy),
            Dir::L => (dy, dx) = (-dx, dy),
        }
    }

    (1000 * (y + 1)
        + 4 * (x + 1)
        + match (dy, dx) {
            (0, 1) => 0,
            (1, 0) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            _ => unreachable!(),
        }) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";
        assert_eq!(part1(&parse_str(str_input)), 6032);
        // assert_eq!(part2(&parse_str(str_input)), 5031);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/22.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 65368);
        assert_eq!(part2(&input), 156166);
    }
}
