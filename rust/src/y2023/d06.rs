use std::fs;

use itertools::Itertools;

type T = usize;
type Race = (T, T);
pub type ParsedData = (Vec<Race>, Race);

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut lines = contents.trim_end().lines();
    let times = lines.next().unwrap().strip_prefix("Time: ").unwrap();
    let dists = lines.next().unwrap().strip_prefix("Distance: ").unwrap();
    let p1t = times
        .split_whitespace()
        .map(|t| t.parse::<T>().unwrap())
        .collect_vec();
    let p1d = dists
        .split_whitespace()
        .map(|t| t.parse::<T>().unwrap())
        .collect_vec();

    (
        p1t.iter().cloned().zip(p1d.iter().cloned()).collect_vec(),
        (
            times.replace(" ", "").parse::<T>().unwrap(),
            dists.replace(" ", "").parse::<T>().unwrap(),
        ),
    )
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

fn count_wins(time: T, dist: T) -> T {
    let t = time as f64;
    let sq = ((time.pow(2) - 4 * dist) as f64).sqrt();
    let t1 = ((t - sq) / 2.0).ceil() as T;
    let t2 = ((t + sq) / 2.0).floor() as T;
    let mut res = t2 - t1;
    if (sq as T) as f64 == sq {
        res -= 1;
    } else {
        res += 1;
    };
    res
}

pub fn part1(data: &ParsedData) -> usize {
    let (races, _) = data;
    races.iter().map(|&(t, d)| count_wins(t, d)).product()
}

pub fn part2(data: &ParsedData) -> usize {
    let &(_, (t, d)) = data;
    count_wins(t, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(&parse_str(str_input)), 288);
        assert_eq!(part2(&parse_str(str_input)), 71503);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/06.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 1084752);
        assert_eq!(part2(&input), 28228952);
    }
}
