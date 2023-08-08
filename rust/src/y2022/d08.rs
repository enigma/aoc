use std::fs;

pub type ParsedData = Vec<Vec<usize>>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    let mut row = vec![];
    contents.chars().for_each(|c| {
        if c == '\n' {
            res.push(row.clone());
            row = vec![];
        } else {
            row.push((c as u8 - b'0') as usize);
        }
    });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

type Pos = (i32, i32);
type Dir = (i32, i32);

#[inline]
fn walk(pos: Pos, dx: i32, dy: i32, height: i32, width: i32) -> impl Iterator<Item = Pos> {
    let (mut x, mut y) = pos;
    (0..)
        .map(move |_| {
            x += dx;
            y += dy;
            (x - dx, y - dy)
        })
        .take_while(move |(x, y)| 0 <= *x && *x < width && 0 <= *y && *y < height)
}

// y2022 d08 part1 full    time:   [107.98 µs 108.27 µs 108.57 µs]
pub fn part1(forest: &ParsedData) -> usize {
    let height = forest.len() as i32;
    let width = forest[0].len() as i32;
    let mut seen = [[false; 100]; 100];
    let dir_start: [(Dir, Vec<Pos>); 4] = [
        ((1, 0), (0..height).map(|y| (0, y)).collect()),
        ((-1, 0), (0..height).map(|y| (width - 1, y)).collect()),
        ((0, 1), (0..width).map(|x| (x, 0)).collect()),
        ((0, -1), (0..width).map(|x| (x, height - 1)).collect()),
    ];
    for (dir, start) in dir_start.iter() {
        for starting in start.iter() {
            let mut cur_max: isize = -1;
            for (x, y) in &mut walk(*starting, dir.0, dir.1, height, width) {
                let tree = forest[y as usize][x as usize] as isize;
                if tree > cur_max {
                    cur_max = tree;
                    seen[y as usize][x as usize] = true;
                }
            }
        }
    }
    seen.iter()
        .map(|row| row.iter().filter(|e| **e).count())
        .sum()
}

// y2022 d08 part2 full    time:   [294.78 µs 297.59 µs 300.52 µs]
pub fn part2(forest: &ParsedData) -> usize {
    let directions: [Dir; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let height = forest.len() as i32;
    let width = forest[0].len() as i32;
    let mut res = 0;
    for (sy, row) in forest.iter().enumerate() {
        for (sx, el) in row.iter().enumerate() {
            let mut score = 1;
            for (dx, dy) in directions.iter() {
                let mut steps = 0;
                for (x, y) in &mut walk((sx as i32 + *dx, sy as i32 + *dy), *dx, *dy, height, width)
                {
                    steps += 1;
                    if forest[y as usize][x as usize] >= *el {
                        break;
                    }
                }
                score *= steps;
            }
            res = res.max(score as usize);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "30373
25512
65332
33549
35390
";
        assert_eq!(part1(&parse_str(str_input)), 21);
        assert_eq!(part2(&parse_str(str_input)), 8);
    }

    #[test]
    fn actual() {
        let path = &"inputs/2022/08.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 1715);
        assert_eq!(part2(&input), 374400);
    }
}
