use std::{collections::VecDeque, fs};

pub type ParsedData = String;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    contents.to_string()
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn solve(text: String, size: usize) -> usize {
    let mut counter = [false; 256];
    let mut prev = VecDeque::with_capacity(size);
    for (n, c) in text.chars().enumerate() {
        while counter[c as usize] {
            counter[prev.pop_front().unwrap() as usize] = false;
        }
        counter[c as usize] = true;
        prev.push_back(c);
        if prev.len() == size {
            return n + 1;
        }
    }
    panic!("Should never get here");
}

// y2022 d06 part1 full    time:   [24.339 µs 24.431 µs 24.522 µs]
pub fn part1(x: &ParsedData) -> usize {
    solve(x.clone(), 4)
}

// y2022 d06 part2 full    time:   [30.503 µs 30.591 µs 30.676 µs]
pub fn part2(x: &ParsedData) -> usize {
    solve(x.clone(), 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part1(&parse_str(str_input)), 5);
        assert_eq!(part2(&parse_str(str_input)), 23);
    }

    #[test]
    fn actual() {
        let path = &"inputs/2022/06.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 1929);
        assert_eq!(part2(&input), 3298);
    }
}
