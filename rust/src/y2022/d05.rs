use sscanf::sscanf;
use std::fs;

type Move = (usize, usize, usize);
type Stacks = Vec<Vec<char>>;
pub type ParsedData = (Stacks, Vec<Move>);

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut stacks = vec![];
    let mut moves = vec![];
    let mut mode = 0;
    contents.trim_end().lines().for_each(|line| {
        if mode == 0 {
            if line.is_empty() {
                mode = 1;
                return;
            }
            let els = line.chars().skip(1).step_by(4);
            if stacks.is_empty() {
                for _ in els {
                    stacks.push(vec![]);
                }
            }
            let els = line.chars().skip(1).step_by(4);
            for (stack, el) in stacks.iter_mut().zip(els) {
                if el != ' ' {
                    stack.push(el);
                }
            }
        }
        if mode == 1 {
            let (a, b, c) = sscanf!(line, "move {usize} from {usize} to {usize}").unwrap();
            moves.push((a, b - 1, c - 1));
        }
    });
    let mut res = vec![];
    for stack in stacks.iter() {
        let mut v = stack.clone();
        v.pop();
        v.reverse();
        res.push(v);
    }
    (res, moves)
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

// y2022 d05 part1 full    time:   [139.37 µs 139.95 µs 140.64 µs]
pub fn part1(lines: &ParsedData) -> String {
    let (mut stacks, moves) = lines.clone();
    for (cnt, from, to) in moves.iter() {
        for _ in 0..*cnt {
            let moved = stacks[*from].pop().unwrap();
            let _ = &stacks[*to].push(moved);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

// y2022 d05 part2 full    time:   [151.23 µs 152.44 µs 153.74 µs]
pub fn part2(lines: &ParsedData) -> String {
    let (mut stacks, moves) = lines.clone();
    for (cnt, from, to) in moves.iter() {
        let from_stack = &mut stacks[*from];
        let mut suffix = from_stack.drain((from_stack.len() - cnt)..).collect();
        stacks[*to].append(&mut suffix);
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(part1(&parse_str(str_input)), "CMZ");
        assert_eq!(part2(&parse_str(str_input)), "MCD");
    }

    #[test]
    fn actual() {
        let path = &"inputs/2022/05.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), "QPJPLMNNR");
        assert_eq!(part2(&input), "BQDNWJPVJ");
    }
}
