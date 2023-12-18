use std::fs;

type Dir = (isize, isize);
pub type ParsedData = (Vec<Dir>, Vec<Dir>);

const DIRS: [Dir; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let (mut p1, mut p2) = (vec![], vec![]);
    for line in contents.trim_end().lines() {
        let mut parts = line.split_ascii_whitespace();
        let d = parts.next().unwrap();
        let step = parts.next().unwrap().parse::<isize>().unwrap();

        let (dy, dx) = match d.as_bytes().first().unwrap() {
            b'R' => DIRS[0],
            b'D' => DIRS[1],
            b'L' => DIRS[2],
            _ => DIRS[3],
        };
        p1.push((dy * step, dx * step));

        let color = parts.next().unwrap();
        let step = u64::from_str_radix(&color[2..color.len() - 2], 16).unwrap() as isize;
        let (dy, dx) = DIRS[(color.as_bytes()[7] - b'0') as usize];
        p2.push((dy * step, dx * step));
    }
    (p1, p2)
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

fn total_area(instrs: &Vec<Dir>) -> usize {
    let (mut y, mut x) = (0, 0);
    let mut vertex = vec![];
    for &(dy, dx) in instrs {
        y += dy;
        x += dx;
        vertex.push((y, x));
    }
    let iter = vertex.iter();
    let mut shoelace = 0isize;
    for (&(y1, x1), &(y2, x2)) in iter.clone().zip(iter.skip(1).chain(vertex.iter().take(1))) {
        shoelace += x1 * y2 - x2 * y1;
        shoelace += (x1.abs_diff(x2) + y1.abs_diff(y2)) as isize;
    }
    (shoelace / 2 + 1) as usize
}

pub fn part1(data: &ParsedData) -> usize {
    total_area(&data.0)
}

pub fn part2(data: &ParsedData) -> usize {
    total_area(&data.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(part1(&parse_str(str_input)), 62);
        assert_eq!(part2(&parse_str(str_input)), 952408144115);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/18.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 47527);
        assert_eq!(part2(&input), 52240187443190);
    }
}
