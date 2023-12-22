use std::{collections::VecDeque, fs, sync::Mutex};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Size = (usize, usize);
type Block = (Size, Size, Size);
pub type ParsedData = (usize, usize);

const N: usize = 12;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut blocks: Vec<Block> = vec![];
    for line in contents.trim_end().lines() {
        let mut parts = line.split("~");

        let f: Block = parts
            .next()
            .unwrap()
            .split(",")
            .map(|i| i.parse::<usize>().unwrap())
            .zip(
                parts
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|i| i.parse::<usize>().unwrap() + 1),
            )
            .collect_tuple()
            .unwrap();
        blocks.push(f);
    }
    blocks.sort_by_key(|&b| b.2 .1);

    let mut maxz_at = [[(0usize, 0usize); N]; N];
    let mut above = HashMap::new();
    let mut below = HashMap::new();
    for (ii, &(xr, yr, zr)) in blocks.iter().enumerate() {
        let i = ii + 1;
        let mut zmax = 0;
        let support = below.entry(i).or_insert_with(|| HashSet::new());
        for x in xr.0..xr.1 {
            for y in yr.0..yr.1 {
                let (z, piece) = maxz_at[y][x];
                if z == zmax {
                    support.insert(piece);
                } else if z > zmax {
                    support.clear();
                    zmax = z;
                    support.insert(piece);
                }
            }
        }
        above.entry(i).or_insert_with(|| HashSet::new());
        for &j in support.iter() {
            let a = above.entry(j).or_insert_with(|| HashSet::new());
            a.insert(i);
        }
        for x in xr.0..xr.1 {
            for y in yr.0..yr.1 {
                maxz_at[y][x] = (zmax + zr.1 - zr.0, i);
            }
        }
    }
    let p1 = above
        .iter()
        .filter(|&(&b, top)| {
            b > 0
                && top
                    .iter()
                    .all(|&t| below.get(&t).map(|q| q.len() > 1).unwrap_or(false))
        })
        .count();

    // This takes 30x as much as the next
    // let p2 = (1..blocks.len() + 1)
    //     .map(|i| fall_from(i, &above, &below))
    //     .sum::<usize>();
    let sops = Box::new(Mutex::new(HashMap::new()));
    let p2 = (1..blocks.len() + 1)
        .map(|i| single_points_of_failure(i, &below, &sops).len())
        .sum::<usize>();

    (p1, p2)
}

fn single_points_of_failure<'a>(
    ib: usize,
    below: &'a HashMap<usize, HashSet<usize>>,
    sops: &Box<Mutex<HashMap<usize, Box<HashSet<usize>>>>>,
) -> Box<HashSet<usize>> {
    if let Some(v) = sops.lock().unwrap().get(&ib) {
        return v.clone();
    }
    let mut bn = below.get(&ib).unwrap().clone();
    bn.remove(&0);
    let result = Box::new(match bn.len() {
        0 => bn,
        1 => {
            bn.extend(single_points_of_failure(*bn.iter().next().unwrap(), below, sops).iter());
            bn
        }
        _ => {
            let mut sops = bn
                .iter()
                .map(|i| single_points_of_failure(*i, below, sops))
                .collect_vec();
            let mut base = *sops.pop().unwrap().clone();
            while let Some(o) = sops.pop() {
                base.retain(|k| o.contains(k));
            }
            base
        }
    });
    sops.lock().unwrap().insert(ib, result.clone());
    result
}

fn fall_from(
    start: usize,
    above: &HashMap<usize, HashSet<usize>>,
    below: &HashMap<usize, HashSet<usize>>,
) -> usize {
    let mut fell = HashSet::new();
    fell.insert(start);
    let mut fringe = VecDeque::new();
    fringe.extend(above.get(&start).unwrap());
    while let Some(cur) = fringe.pop_front() {
        let b = below.get(cur).unwrap();
        if fell.is_superset(b) {
            fell.insert(*cur);
            fringe.extend(above.get(cur).unwrap());
        }
    }
    fell.len() - 1
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

pub fn part1(data: &ParsedData) -> usize {
    data.0
}

pub fn part2(data: &ParsedData) -> usize {
    data.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(part1(&parse_str(str_input)), 5);
        assert_eq!(part2(&parse_str(str_input)), 7);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/22.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 454);
        assert_eq!(part2(&input), 74287);
    }
}
