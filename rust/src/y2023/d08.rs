use std::fs;

use hashbrown::HashMap;
use itertools::Itertools;

const MAPPINGS: usize = 800;
type Key = usize;
type Dir = u8;
type Mapping = (Key, Key);
type Mappings = [Mapping; MAPPINGS];
pub type ParsedData = (Vec<Dir>, Mappings, (Key, Key), (Vec<Key>, Vec<Key>));

fn keyed<'a>(i: &mut usize, k: &'a str, km: &mut HashMap<&'a str, usize>) -> Key {
    let v = km.entry(k).or_default();
    if *v == 0 {
        *v = *i;
        *i += 1;
    }
    *v
}

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut parts = contents.split("\n\n");
    let dirs = parts
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .cloned()
        .collect_vec();

    let mut p1s = 0;
    let mut p1e = 0;
    let mut p2s = vec![];
    let mut p2e = vec![];
    let mut mapping = [(0, 0); MAPPINGS];
    let mut km = HashMap::new();
    let smap = parts.next().unwrap();
    let mut i = 1usize;
    for line in smap.lines() {
        let src = keyed(&mut i, &line[0..3], &mut km);
        let ld = keyed(&mut i, &line[7..10], &mut km);
        let rd = keyed(&mut i, &line[12..15], &mut km);
        match &line[0..3] {
            "AAA" => p1s = src,
            "ZZZ" => p1e = src,
            &_ => {}
        };
        match &line[2..3] {
            "A" => p2s.push(src),
            "Z" => p2e.push(src),
            &_ => {}
        };
        mapping[src] = (ld, rd);
    }

    (dirs, mapping, (p1s, p1e), (p2s, p2e))
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

#[inline]
fn step(cur: &mut usize, dir: Dir, mapping: &Mappings) {
    match dir {
        b'L' => *cur = mapping[*cur].0,
        b'R' => *cur = mapping[*cur].1,
        x => panic!("WTF {:?}", x),
    }
}

pub fn part1(data: &ParsedData) -> usize {
    let (dirs, mapping, (start, end), _) = data;
    let mut count = 0;
    let mut cur = *start;
    while cur != *end {
        step(&mut cur, dirs[count % dirs.len()], mapping);
        count += 1;
    }
    count
}

pub fn part2(data: &ParsedData) -> usize {
    let (dirs, mapping, _, (starts, ends)) = data;
    let mut res = 1;
    for start in starts {
        let mut count = 0;
        let mut cur = *start;
        while ends.iter().all(|&e| e != cur) {
            step(&mut cur, dirs[count % dirs.len()], mapping);
            count += 1;
        }
        res = num_integer::lcm(res, count);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&parse_str(str_input)), 6);
        // assert_eq!(part2(&parse_str(str_input)), 2);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/08.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 18727);
        assert_eq!(part2(&input), 18024643846273);
    }
}
