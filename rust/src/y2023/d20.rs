use std::collections::VecDeque;
use std::fs;

use hashbrown::HashMap;

pub type ParsedData = (usize, usize);

fn mint<'a, 'b>(
    mapping: &'a mut HashMap<&'b str, usize>,
    name: &'b str,
    conjs: &mut usize,
    ffs: &mut usize,
) -> usize {
    let mut key = name;
    let isconj = name.starts_with("&");
    let isff = name.starts_with("%");
    if isconj || isff {
        key = &name[1..];
    }
    let res = if let Some(&v) = mapping.get(key) {
        v
    } else {
        let id = mapping.len() + 1;
        mapping.insert(key, id);
        id
    };
    if isconj {
        *conjs |= 1 << res;
    }
    if isff {
        *ffs |= 1 << res;
    }
    res
}

const NODES: usize = 63;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut names = HashMap::new();
    let mut conjs = 0;
    let mut ffs = 0;
    let mut srcs = [0u128; NODES];
    let mut ff = [0u128; NODES];
    let mut dsts = HashMap::new();
    for line in contents.trim_end().lines() {
        let mut parts = line.split(" -> ");
        let lhs = parts.next().unwrap();
        let srci = mint(&mut names, lhs, &mut conjs, &mut ffs);
        let mut dvec = vec![];
        for dstn in parts.next().unwrap().split(", ") {
            let dsti = mint(&mut names, dstn, &mut conjs, &mut ffs);
            srcs[dsti] |= 1 << srci;
            dvec.push(dsti);
        }
        dsts.insert(srci, dvec);
    }

    let mut sigc = [0, 0];
    let sink = (0..NODES)
        .filter(|&i| srcs[i] > 0 && !dsts.contains_key(&i))
        .next()
        .unwrap();
    let second_last = (0..NODES)
        .filter(|&i| srcs[sink] & (1 << i) > 0)
        .next()
        .unwrap();
    let loop_ends = srcs[second_last];
    let loop_ends_cnt = (0..NODES).filter(|&i| (1 << i) & loop_ends > 0).count();
    let mut loops = HashMap::new();
    let mut click = 0;
    let mut res = (0, 0);
    let broadcaster = names["broadcaster"];
    'outer: loop {
        click += 1;
        let mut dq = VecDeque::new();
        dq.push_front((0, broadcaster, broadcaster));
        while let Some((signal, src, dst)) = dq.pop_back() {
            sigc[signal] += 1;
            if dst == broadcaster {
                for &o in dsts.get(&dst).unwrap() {
                    dq.push_front((signal, dst, o));
                }
            } else if ffs & (1 << dst) > 0 {
                if signal > 0 {
                    continue;
                }
                let ns = 1 - ff[dst];
                ff[dst] = ns;
                for &o in dsts.get(&dst).unwrap() {
                    dq.push_front((ns as usize, dst, o));
                }
            } else if conjs & (1 << dst) > 0 {
                ff[src] = signal as u128;
                let emit = 1
                    - (0..NODES)
                        .filter(|&i| (1 << i) & srcs[dst] > 0)
                        .map(|i| ff[i])
                        .all(|i| i > 0) as usize;

                for &o in dsts.get(&dst).unwrap() {
                    dq.push_front((emit, dst, o));
                }
                if signal == 0 && loop_ends & (1 << dst) > 0 && !loops.contains_key(&dst) {
                    loops.insert(dst, click);
                    if loops.len() == loop_ends_cnt {
                        res.1 = loops.values().product();
                        break 'outer;
                    }
                }
            }
        }
        if click == 1000 {
            res.0 = sigc[0] * sigc[1];
        }
    }
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

pub fn part1(d: &ParsedData) -> usize {
    d.0
}

pub fn part2(d: &ParsedData) -> usize {
    d.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual() {
        let path = &"../inputs/2023/20.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 825896364);
        assert_eq!(part2(&input), 243566897206981);
    }
}
