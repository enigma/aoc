use itertools::Itertools;
use std::fs;

type T = usize;
type Mapping = (T, T, T);

pub type ParsedData = (Vec<T>, Vec<Vec<Mapping>>);

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut sections = contents.trim_end().split("\n\n");

    let seeds = sections.next().unwrap()[6..]
        .split_whitespace()
        .map(|d| d.parse::<T>().unwrap())
        .collect_vec();

    let mut mappings = vec![];
    while let Some(section) = sections.next() {
        let mut lines = section.lines();
        lines.next();
        let mut layer: Vec<Mapping> = vec![];
        while let Some(line) = lines.next() {
            layer.push(
                line.split_whitespace()
                    .map(|c| c.parse::<T>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        }
        layer.sort_by_key(|v| (v.1 + v.2, v.1));
        mappings.push(layer);
    }

    (seeds, mappings)
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

fn find(seed: T, mappings: &Vec<Vec<Mapping>>) -> T {
    let mut current = seed;
    for mapping in mappings {
        for &(dst, src, step) in mapping {
            if src <= current && current < src + step {
                current = dst + current - src;
                break;
            }
        }
    }
    current
}

pub fn part1(data: &ParsedData) -> usize {
    let (seeds, mappings) = data;
    seeds.iter().map(|&s| find(s, mappings)).min().unwrap()
}

fn find_range(rng: (T, T), mappings: &Vec<Vec<Mapping>>) -> T {
    let total_seeds = rng.1 - rng.0;
    let mut ranges = vec![rng];
    for mapping in mappings {
        let mapped_seeds: T = ranges.iter().map(|r| r.1 - r.0).sum();
        assert_eq!(total_seeds, mapped_seeds);
        let mut next_ranges = vec![];
        let mut mapping_left = mapping.iter().rev().collect_vec();
        let mut to_map = ranges.iter().cloned().rev().collect_vec();
        while let Some(current) = to_map.pop() {
            let (mut cur_start, mut cur_end) = current;
            while let Some(&&(_dst, src, step)) = mapping_left.last() {
                if cur_start >= src + step {
                    mapping_left.pop();
                    continue;
                } else {
                    break;
                }
            }
            if mapping_left.is_empty() {
                next_ranges.push(current);
                continue;
            }
            let &&(dst, src, step) = mapping_left.last().unwrap();
            if cur_end < src {
                next_ranges.push(current);
                continue;
            }
            if cur_start < src {
                next_ranges.push((cur_start, src));
                cur_start = src;
            }
            if cur_end >= src + step {
                to_map.push((src + step, cur_end));
                cur_end = src + step;
            }
            assert!(
                src <= cur_start && cur_start < src + step,
                "{:?}",
                (src, cur_start, src + step)
            );
            assert!(
                src < cur_end && cur_end <= src + step,
                "{:?}",
                (src, cur_end, src + step)
            );
            let nxt_start = dst + cur_start - src;
            let nxt_end = dst + cur_end - src;
            let x = (nxt_start, nxt_end);
            next_ranges.push(x);
        }
        next_ranges.sort_by_key(|&v| (v.1, v.0));
        ranges = next_ranges;
    }
    ranges.iter().map(|v| v.0).min().unwrap()
}

pub fn part2(data: &ParsedData) -> usize {
    let (seeds, mappings) = data;
    (0..seeds.len())
        .step_by(2)
        .map(|i| find_range((seeds[i], seeds[i] + seeds[i + 1]), mappings))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part1(&parse_str(str_input)), 35);
        assert_eq!(part2(&parse_str(str_input)), 46);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/05.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 309796150);
        assert_eq!(part2(&input), 50716416);
    }
}
