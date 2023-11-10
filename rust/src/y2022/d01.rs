use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

pub type ParsedData = Vec<usize>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut numbers: Vec<usize> = Vec::new();
    let mut acc = 0;
    for x in contents.lines() {
        if x.is_empty() {
            numbers.push(acc);
            acc = 0;
        } else {
            acc += x.parse::<usize>().unwrap();
        }
    }
    numbers.push(acc);
    numbers
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

// y2022 d01 part1         time:   [37.797 µs 37.953 µs 38.143 µs]
pub fn part1(numbers: &ParsedData) -> usize {
    *numbers.iter().max().unwrap()
}

// y2022 d01 part2         time:   [40.109 µs 40.265 µs 40.462 µs]
pub fn part2(numbers: &ParsedData) -> usize {
    let mut heap = BinaryHeap::new();
    numbers.iter().for_each(|k| {
        heap.push(Reverse(k));
        if heap.len() > 3 {
            heap.pop();
        }
    });

    heap.iter().map(|v| *v.0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(part1(&parse_str(str_input)), 24000);
        assert_eq!(part2(&parse_str(str_input)), 45000);
    }

    #[test]
    fn actual() {
        let numbers = parse(&"../inputs/2022/01.input".to_string());
        assert_eq!(part1(&numbers), 71124);
        assert_eq!(part2(&numbers), 204639);
    }
}
