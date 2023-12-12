use std::fs;

use itertools::Itertools;

pub type ParsedData = Vec<(Vec<u8>, Vec<usize>)>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    for line in contents.trim_end().lines() {
        let mut half = line.split_ascii_whitespace();
        let lhs = half
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .cloned()
            .collect_vec();
        let rhs = half
            .next()
            .unwrap()
            .split_terminator(',')
            .map(|i| i.parse::<usize>().unwrap())
            .collect_vec();
        res.push((lhs, rhs));
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

const ROW: usize = 31;
const COL: usize = 106;
type Cache = [[(u16, usize); COL]; ROW];
fn arrangements(
    record: &[u8],
    conf: &[usize],
    ir: usize,
    ic: usize,
    version: u16,
    cache: &mut Cache,
) -> usize {
    if ic >= conf.len() {
        return if ir > record.len() || record[ir..].iter().all(|&i| i != b'#') {
            1
        } else {
            0
        };
    }
    if cache[ic][ir].0 == version {
        return cache[ic][ir].1;
    }
    let b = conf[ic];
    let mut result = 0;
    if ir < record.len() {
        if ir + b <= record.len()
            && record[ir..]
                .iter()
                .enumerate()
                .take(b + 1)
                .all(|(i, &e)| (i < b && e != b'.') || (i == b && e != b'#'))
        {
            result += arrangements(record, conf, ir + b + 1, ic + 1, version, cache);
        }
        if record[ir] != b'#' {
            result += arrangements(record, conf, ir + 1, ic, version, cache)
        }
    }
    cache[ic][ir] = (version, result);
    result
}

pub fn part1(data: &ParsedData) -> usize {
    let mut res = 0;
    let mut cache = [[(0, 0); COL]; ROW];
    for (v, (lhs, rhs)) in data.iter().enumerate() {
        res += arrangements(&lhs, &rhs, 0, 0, 1 + v as u16, &mut cache);
    }
    res
}

pub fn part2(data: &ParsedData) -> usize {
    let mut p2v: ParsedData = Vec::with_capacity(data.len());
    for (lhs, rhs) in data {
        let mut nlhs = Vec::with_capacity(lhs.len() * 5 + 5);
        let mut nrhs: Vec<usize> = Vec::with_capacity(rhs.len() * 5);
        nlhs.extend(lhs);
        nrhs.extend(rhs);
        for _ in 1usize..5 {
            nlhs.push(b'?');
            nlhs.extend(lhs);
            nrhs.extend(rhs);
        }
        p2v.push((nlhs, nrhs));
    }
    part1(&p2v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part1(&parse_str(str_input)), 21);
        assert_eq!(part2(&parse_str(str_input)), 525152);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/12.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 6852);
        assert_eq!(part2(&input), 8475948826693);
    }
}
