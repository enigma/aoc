// use anyhow::Ok;
// use anyhow::Result;
use sscanf::sscanf;
use std::{cmp::Ordering, collections::BinaryHeap, fs};

type Qty = u16;
type Materials = [Qty; 4];
type Recipe = [Qty; 3];
type Blueprint = (Qty, [Recipe; 4]);

pub type ParsedData = Vec<Blueprint>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    contents.trim().lines().for_each(|line| {
            let (id, ore_ore, clay_ore, ob_ore, ob_clay, g_ore, g_ob) = sscanf!(line, "Blueprint {Qty}: Each ore robot costs {Qty} ore. Each clay robot costs {Qty} ore. Each obsidian robot costs {Qty} ore and {Qty} clay. Each geode robot costs {Qty} ore and {Qty} obsidian.").unwrap();
            res.push(
                (id,
                [
                    [ore_ore, 0, 0],
                    [clay_ore, 0, 0],
                    [ob_ore, ob_clay, 0],
                    [g_ore, 0, g_ob],
                ])
           );
    });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

const GEODE: usize = 3;
const OBSID: usize = 2;
const _CLAY: usize = 1;
const ORE: usize = 0;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct State<'a> {
    minutes_left: Qty,
    robots: Materials,
    material: Materials,
    blueprint: &'a Blueprint,
}

impl State<'_> {
    fn new(minutes_left: Qty, bp: &Blueprint) -> State {
        State {
            minutes_left: minutes_left,
            robots: [1, 0, 0, 0],
            material: [0, 0, 0, 0],
            blueprint: bp,
        }
    }

    fn build(&self, res: usize) -> Option<Self> {
        if res != GEODE && self.robots[res] >= (0..4).map(|i| self.blueprint.1[i][res]).max()? {
            // Have enough...
            return None;
        }
        let bp = self.blueprint.1[res];
        if !(0..3).all(|i| bp[i] == 0 || self.robots[i] > 0) {
            // Can't make ingredients...
            return None;
        }
        let wait = (0..3)
            .map(|i| {
                (self.robots[i] > 0).then(|| {
                    if bp[i] > self.material[i] {
                        (bp[i] - self.material[i]).div_ceil(self.robots[i])
                    } else {
                        0
                    }
                })
            })
            .flatten()
            .max()?
            + 1;
        if wait > self.minutes_left {
            return None;
        }
        let mut result = self.clone();
        result.minutes_left -= wait;
        for i in 0..4 {
            result.material[i] += self.robots[i] * wait;
        }
        for i in 0..3 {
            result.material[i] -= bp[i];
        }
        result.robots[res] += 1;
        return Some(result);
    }

    fn lower_bound(&self) -> Qty {
        let mut robots = self.robots;
        let mut material = self.material;
        let recipe = self.blueprint.1[GEODE];
        for _ in 0..self.minutes_left {
            let will_build =
                (material[ORE] >= recipe[ORE] && material[OBSID] >= recipe[OBSID]) as Qty;
            (0..4).for_each(|m| material[m] += robots[m]);
            material[ORE] -= recipe[ORE] * will_build;
            material[OBSID] -= recipe[OBSID] * will_build;
            robots[GEODE] += will_build;
        }
        material[GEODE]
    }

    fn upper_bound(&self) -> Qty {
        let mut robots = self.robots;
        let mut material = self.material;
        let bp = self.blueprint.1;
        let mut will_build = [0; 4];
        for _ in 0..self.minutes_left {
            (0..4).for_each(|i| will_build[i] = (0..3).all(|j| bp[i][j] <= material[j]) as Qty);
            (0..4).for_each(|i| {
                material[i] += robots[i];
                robots[i] += will_build[i];
            });
        }
        material[GEODE]
    }
}

#[derive(Debug, Copy, Clone)]
struct Priority<P, T>(P, T);

impl<P: Ord + Eq, T> Ord for Priority<P, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<P: Ord + Eq, T> PartialOrd for Priority<P, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: Eq, T> PartialEq for Priority<P, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P: Eq, T> Eq for Priority<P, T> {}

fn max_geode(minutes_left: Qty, blueprint: Blueprint) -> Qty {
    let mut best = 0;
    let mut fringe = BinaryHeap::with_capacity(10_000);
    fringe.push(Priority(1, State::new(minutes_left, &blueprint)));
    while let Some(Priority(upper, state)) = fringe.pop() {
        if upper <= best {
            break;
        }
        for next in (0..4).map(|i| state.build(i)).flatten() {
            let nxt_upper = next.upper_bound();
            if nxt_upper > best {
                best = best.max(next.lower_bound());
                if next.minutes_left > 0 {
                    fringe.push(Priority(nxt_upper, next));
                }
            }
        }
    }
    best
}

pub fn part1(bps: &ParsedData) -> usize {
    bps.iter()
        .map(|&(k, bp)| k * max_geode(24, (k, bp)))
        .sum::<Qty>() as usize
}

pub fn part2(bps: &ParsedData) -> usize {
    let mut prod = 1;
    for &bp in bps.iter().take(3) {
        prod *= max_geode(32, bp) as usize;
    }
    prod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";
        assert_eq!(part1(&parse_str(str_input)), 33);
        assert_eq!(part2(&parse_str(str_input)), 3472);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/19.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 1719);
        assert_eq!(part2(&input), 19530);
    }
}
