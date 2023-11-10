use std::fs;

pub type ParsedData = Vec<((usize, usize), (usize, usize))>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    contents.trim().lines().for_each(|l| {
        let elves = l.split(",");
        let mut entry = [(0, 0); 2];
        let mut i = 0;
        elves.for_each(|e| {
            let mut parts = e.split("-");
            entry[i] = (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            );
            i += 1;
        });
        entry.sort();
        res.push((entry[0], entry[1]));
    });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

// y2022 d04 part1 full    time:   [99.940 µs 100.30 µs 100.69 µs]
pub fn part1(lines: &ParsedData) -> usize {
    let mut res = 0;
    for (r1, r2) in lines.iter() {
        let inner = (r1.0.min(r2.0), r1.1.max(r2.1));
        if inner == *r1 || inner == *r2 {
            res += 1
        }
    }
    res
}

// y2022 d04 part2 full    time:   [100.85 µs 101.10 µs 101.38 µs]
pub fn part2(lines: &ParsedData) -> usize {
    let mut res = 0;
    for (r1, r2) in lines.iter() {
        if r1.0 <= r2.0 && r2.0 <= r1.1 {
            res += 1
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(part1(&parse_str(str_input)), 2);
        assert_eq!(part2(&parse_str(str_input)), 4);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/04.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 500);
        assert_eq!(part2(&input), 815);
    }
}
