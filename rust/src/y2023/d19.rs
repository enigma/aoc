use std::fs;

use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Cond {
    Lt(usize, usize),
    Gt(usize, usize),
    True,
}

type Rule = Vec<(Cond, String)>;
pub type ParsedData = (HashMap<String, Rule>, Vec<Vec<usize>>);

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut rules = HashMap::new();
    let mut iparts = contents.trim_end().split("\n\n");
    for line in iparts.next().unwrap().lines() {
        let mut rparts = line[..line.len() - 1].split("{");
        rules.insert(
            rparts.next().unwrap().to_string(),
            rparts
                .next()
                .unwrap()
                .split(",")
                .map(|cond| {
                    if let Some(p) = cond.as_bytes().iter().position(|&x| x == b':') {
                        let rulep = &cond.as_bytes()[..p];
                        let srci = match rulep[0] {
                            b'x' => 0,
                            b'm' => 1,
                            b'a' => 2,
                            _ => 3,
                        };
                        let mut v = 0;
                        for &i in rulep[2..].iter() {
                            v = (v * 10) + (i - b'0') as usize;
                        }
                        let dst = std::str::from_utf8(&cond.as_bytes()[p + 1..])
                            .unwrap()
                            .to_string();
                        match rulep[1] {
                            b'>' => (Cond::Gt(srci, v), dst),
                            _ => (Cond::Lt(srci, v), dst),
                        }
                    } else {
                        (Cond::True, cond.to_string())
                    }
                })
                .collect_vec(),
        );
    }
    let pieces = iparts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line[1..line.len() - 1]
                .split(",")
                .map(|comp| comp[2..].parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    (rules, pieces)
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

pub fn part1((rules, parts): &ParsedData) -> usize {
    parts
        .iter()
        .map(|part| {
            let mut rule = "in";
            while let Some(&ref v) = &rules.get(rule) {
                for (cond, dst) in v {
                    match *cond {
                        Cond::Lt(i, v) => {
                            if part[i] < v {
                                rule = dst;
                                break;
                            }
                        }
                        Cond::Gt(i, v) => {
                            if part[i] > v {
                                rule = dst;
                                break;
                            }
                        }
                        Cond::True => {
                            rule = dst;
                            break;
                        }
                    }
                }
            }
            if rule == "A" {
                part.iter().sum::<usize>()
            } else {
                0
            }
        })
        .sum::<usize>()
}

pub fn part2((rules, _): &ParsedData) -> usize {
    let mut res = 0;
    let mut fringe = vec![];
    fringe.push(("in", [(1usize, 4001usize); 4]));
    while let Some((cur, startingr)) = fringe.pop() {
        if cur == "A" {
            res += startingr.iter().map(|&(a, b)| b - a).product::<usize>();
            continue;
        }
        if cur == "R" {
            continue;
        }
        let mut ranges = startingr;
        for (cond, dst) in rules.get(cur).unwrap() {
            match *cond {
                Cond::True => {
                    fringe.push((dst, ranges.clone()));
                    continue;
                }
                Cond::Gt(pi, v) => {
                    let (lo, hi) = ranges[pi];
                    if v + 1 < hi {
                        let middle = lo.max(v + 1);
                        ranges[pi] = (middle, hi);
                        fringe.push((dst, ranges.clone()));
                        ranges[pi] = (lo, middle);
                    }
                }
                Cond::Lt(pi, v) => {
                    let (lo, hi) = ranges[pi];
                    if lo < v {
                        let middle = hi.min(v);
                        ranges[pi] = (lo, middle);
                        fringe.push((dst, ranges.clone()));
                        ranges[pi] = (middle, hi);
                    }
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(part1(&parse_str(str_input)), 19114);
        assert_eq!(part2(&parse_str(str_input)), 167409079868000);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/19.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 395382);
        assert_eq!(part2(&input), 103557657654583);
    }
}
