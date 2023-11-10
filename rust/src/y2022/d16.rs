use hashbrown::HashMap;
use sscanf::sscanf;
use std::{fs, sync::Mutex};

const SIZE: usize = 61;
type Enc = u128;
type Flows = [usize; SIZE];
type Dist = [[usize; SIZE]; SIZE];
type CacheKey = (usize, usize, Enc, bool);
type Cache = Box<Mutex<HashMap<CacheKey, usize>>>;
pub type ParsedData = (Enc, Dist, Flows, Enc, Vec<Enc>, Cache);

// y2022 d16 parsing       time:   [531.84 µs 540.95 µs 548.69 µs]
#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut valves_order = HashMap::new();
    let mut estart = Enc::MAX;
    let mut dist = [[SIZE; SIZE]; SIZE];
    let mut flows: Flows = [0; SIZE];
    let mut closed: Enc = 0;
    contents.trim().lines().for_each(|line| {
        let (src, rate, _, _, others) = sscanf!(
            line,
            "Valve {String} has flow rate={usize}; tunnel{String} to valv{String} {String}"
        )
        .unwrap();
        let src_id = {
            let nxt: Enc = valves_order.len() as Enc;
            let entry = valves_order.entry(src.to_string()).or_insert(nxt);
            *entry
        };
        if &src == "AA" {
            estart = src_id;
        }
        dist[src_id as usize][src_id as usize] = 0;
        others.split(", ").for_each(|s| {
            let nxt: Enc = valves_order.len() as Enc;
            let entry = valves_order.entry(s.to_string()).or_insert(nxt);
            dist[src_id as usize][*entry as usize] = 1;
        });
        flows[src_id as usize] = rate;
        if rate > 0 {
            closed |= 1 << src_id;
        }
    });
    let total = valves_order.len();
    for i in 0..total {
        for j in 0..total {
            for k in 0..total {
                dist[j][k] = dist[j][k].min(dist[j][i] + dist[i][k]);
            }
        }
    }
    let cache = Box::new(Mutex::new(HashMap::with_capacity(0)));
    let good_valves = (0..flows.len() as Enc)
        .filter(|&i| flows[i as usize] > 0)
        .collect();
    (estart, dist, flows, closed, good_valves, cache)
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn maxpressure(pos: usize, minutes_left: usize, closed: Enc, pd: &ParsedData) -> usize {
    let (_, dist, flows, _, good_valves, _cache) = pd;
    if closed == 0 {
        return 0;
    }
    // let key: CacheKey = (pos, minutes_left, closed, false);
    // if let Some(&v) = cache.lock().unwrap().get(&key) {
    //     return v;
    // }
    let mut pressure = 0;
    // for dst_id in 0..=SIZE {
    for &dst_id in good_valves.iter() {
        let gv = 1 << dst_id;
        let dst_id = dst_id as usize;
        if gv & closed == 0 {
            continue;
        }
        let distance = dist[pos][dst_id];
        if distance + 1 > minutes_left {
            continue;
        }
        let tick = minutes_left - distance - 1;
        pressure = pressure.max(flows[dst_id] * tick + maxpressure(dst_id, tick, closed - gv, pd))
    }
    // cache.lock().unwrap().insert(key, pressure);
    pressure
}

fn subsets(v: &[Enc], length: usize) -> Box<dyn Iterator<Item = Enc>> {
    if v.len() < length || length == 0 {
        return Box::new(std::iter::empty());
    }
    if v.len() == length {
        return Box::new(std::iter::once(v.iter().map(|&i| 1 << i).sum()));
    }
    let item = 1 << v[0];
    Box::new(
        subsets(&v[1..], length - 1)
            .map(move |rest| rest | item)
            .chain(subsets(&v[1..], length)),
    )
}

pub fn part1(pd: &ParsedData) -> usize {
    let (pos, _d, _f, initially_closed, _gv, _c) = pd;
    maxpressure(*pos as usize, 30, *initially_closed, pd)
}

pub fn part2(pd: &ParsedData) -> usize {
    let (pos, _d, _f, initially_closed, good_valves, _c) = pd;
    let mut best = 0;
    for (n, subset) in subsets(good_valves, good_valves.len() / 2).enumerate() {
        if n > 300 {
            // ~ half of 15 chose 5, why?
            // There's 15c7 (== 15c8) splits of 15 elements in two groups of 7 + 8 elements. ()
            break;
        }
        let a = maxpressure(*pos as usize, 26, initially_closed - subset, pd);
        let b = maxpressure(*pos as usize, 26, subset, pd);
        best = best.max(a + b);
    }
    //maxpressure(*pos as usize, 26, *initially_closed, pd)
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(part1(&parse_str(str_input)), 1651);
        assert_eq!(part2(&parse_str(str_input)), 1707);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/16.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 1850);
        assert_eq!(part2(&input), 2306);
    }
}

use criterion::Criterion;
pub fn bench(c: &mut Criterion) {
    let path = &"../inputs/2022/16.input".to_string();
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    let parse = || parse_str(&contents);
    let parsed = parse();
    c.bench_function("y2022 d16 parsing", |b| b.iter(|| parse_str(&contents)));
    c.bench_function("y2022 d16 part1 noparsing", |b| b.iter(|| part1(&parsed)));
    c.bench_function("y2022 d16 part1 full", |b| b.iter(|| part1(&parse())));
    c.bench_function("y2022 d16 part2 noparsing", |b| b.iter(|| part2(&parsed)));
    c.bench_function("y2022 d16 part2 full", |b| b.iter(|| part2(&parse())));
}
