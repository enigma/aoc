use std::{collections::VecDeque, fs};

type ParsedData = Vec<isize>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    contents
        .trim_end()
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect()
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

// My solve: part1+part2   time:   [117.58 ms 117.86 ms 118.16 ms]
fn solve(data: &ParsedData, factor: isize, repetitions: usize) -> usize {
    let order: Vec<(usize, isize)> = data
        .iter()
        .enumerate()
        .map(|(i, d)| (i, d * factor))
        .collect();
    let mut mixing = VecDeque::from(order.clone());
    for _ in 0..repetitions {
        for cur in 0..order.len() {
            let idx = mixing.iter().position(|&el| el == order[cur]).unwrap();
            mixing.remove(idx);
            let (rotation, size) = ((idx as isize) + order[cur].1, data.len() - 1);
            if rotation < 0 {
                mixing.rotate_right((-rotation) as usize % size)
            } else {
                mixing.rotate_left(rotation as usize % size);
            }
            mixing.push_front((cur, order[cur].1))
        }
    }
    let zero = order.iter().find(|&(_, v)| *v == 0).unwrap();
    mixing.rotate_left(mixing.iter().position(|&v| v == *zero).unwrap());
    [1000, 2000, 3000]
        .iter()
        .map(|&i| mixing[i % data.len()].1)
        .sum::<isize>() as usize
}

// y2022 d20 part1 full    time:   [10.729 ms 10.764 ms 10.802 ms]
pub fn part1(data: &ParsedData) -> usize {
    solve(data, 1, 1)
}

// y2022 d20 part2 full    time:   [107.27 ms 107.61 ms 107.96 ms]
pub fn part2(data: &ParsedData) -> usize {
    solve(data, 811589153, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "1
2
-3
3
-2
0
4";
        assert_eq!(part1(&parse_str(str_input)), 4 - 3 + 2);
        assert_eq!(part2(&parse_str(str_input)), 1623178306);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/20.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 2827);
        assert_eq!(part2(&input), 7834270093909);
    }
}
