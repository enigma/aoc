use std::fs;

type Step = Vec<u8>;
pub type ParsedData = Vec<Step>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    contents
        .trim_end()
        .split(',')
        .map(|s| s.as_bytes().to_vec())
        .collect()
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

fn hash(part: &[u8]) -> usize {
    let mut res = 0;
    for &c in part {
        res = ((res + c as usize) * 17) % 256;
    }
    res
}

pub fn part1(steps: &ParsedData) -> usize {
    steps.iter().map(|s| hash(s)).sum::<usize>()
}

pub fn part2(steps: &ParsedData) -> usize {
    let mut hashmap: Vec<Vec<(&[u8], usize)>> = vec![];
    for _ in 0..256 {
        hashmap.push(Vec::with_capacity(200));
    }
    for step in steps {
        if step.ends_with(b"-") {
            let lab = &step[..step.len() - 1];
            let ibox = hash(lab);
            if let Some(p) = hashmap[ibox].iter().position(|(l, _v)| lab.starts_with(l)) {
                hashmap[ibox].remove(p);
            }
        }
        if let Some(eqi) = step.iter().position(|&c| c == b'=') {
            let lab = &step[..eqi];
            let ibox = hash(lab);
            let val = (&step[eqi + 1] - b'0') as usize;
            let mut found = false;
            for e in hashmap[ibox].iter_mut() {
                if lab == e.0 {
                    found = true;
                    e.1 = val;
                    break;
                }
            }
            if !found {
                hashmap[ibox].push((lab, val));
            }
        }
    }
    hashmap
        .iter()
        .enumerate()
        .map(|(ibox, entries)| {
            entries
                .iter()
                .enumerate()
                .map(|(slot, &(_, l))| (ibox + 1) * (slot + 1) * l)
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(&parse_str(str_input)), 1320);
        assert_eq!(part2(&parse_str(str_input)), 145);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/15.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 504036);
        assert_eq!(part2(&input), 295719);
    }
}
