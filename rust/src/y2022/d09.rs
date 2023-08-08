use std::fs;

pub type ParsedData = Vec<((isize, isize), isize)>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    contents.trim().lines().for_each(|line| {
        let mut split = line.split(" ");
        res.push((
            match split.next().unwrap() {
                "R" => (1 as isize, 0 as isize),
                "U" => (0, 1),
                "L" => (-1, 0),
                _ => (0, -1),
            },
            split.next().unwrap().parse::<isize>().unwrap(),
        ));
    });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

type Pos = (isize, isize);

#[inline]
fn next_tail(tail: Pos, head: Pos) -> Pos {
    let (nx, ny) = (head.0 - tail.0, head.1 - tail.1);
    if nx.abs() < 2 && ny.abs() < 2 {
        return tail;
    }
    let mut next = tail;
    if nx.abs() > 0 {
        next.0 += nx / nx.abs();
    }
    if ny.abs() > 0 {
        next.1 += ny / ny.abs();
    }
    next
}

const SIZE: usize = 600;
const DELTA: isize = 300;

// y2022 d09 part1 full    time:   [137.20 µs 137.68 µs 138.24 µs]
pub fn part1(instructions: &ParsedData) -> usize {
    let mut seen = [[false; SIZE]; SIZE];
    let mut tail: Pos = (0, 0);
    let mut head: Pos = (0, 0);
    seen[DELTA as usize][DELTA as usize] = true;
    for ((dx, dy), steps) in instructions {
        for _ in 0..*steps {
            head.0 += dx;
            head.1 += dy;
            tail = next_tail(tail, head);
            seen[(DELTA + tail.1) as usize][(DELTA + tail.0) as usize] = true;
        }
    }
    seen.iter()
        .map(|row| {
            row.iter()
                .map(|saw| if *saw { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

// y2022 d09 part2 full    time:   [229.46 µs 230.11 µs 230.79 µs]
pub fn part2(instructions: &ParsedData) -> usize {
    let mut seen = [[false; SIZE]; SIZE];
    let mut tails: [Pos; 10] = [(0, 0); 10];
    seen[DELTA as usize][DELTA as usize] = true;
    for ((dx, dy), steps) in instructions {
        for _ in 0..*steps {
            tails[0].0 += dx;
            tails[0].1 += dy;
            for knot in 1..10 {
                tails[knot] = next_tail(tails[knot], tails[knot - 1]);
            }
            let tail = tails[9];
            seen[(DELTA + tail.1) as usize][(DELTA + tail.0) as usize] = true;
        }
    }
    seen.iter()
        .map(|row| {
            row.iter()
                .map(|saw| if *saw { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(part1(&parse_str(str_input)), 13);
        assert_eq!(part2(&parse_str(str_input)), 1);
    }

    #[test]
    fn actual() {
        let path = &"inputs/2022/09.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 6384);
        assert_eq!(part2(&input), 2734);
    }
}
