use std::fs;

type C = usize;
type Hand = [C; 5];
pub type T = usize;
pub type ParsedData = (T, T);

static ORDER: [u8; 13] = [
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A',
];

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut lookup = [0usize; 128];
    for (i, &c) in ORDER.iter().enumerate() {
        lookup[c as usize] = i;
    }

    let cap = 1000;
    let mut part1 = Vec::with_capacity(cap);
    let mut part2 = Vec::with_capacity(cap);
    for line in contents.trim_end().lines() {
        let mut parts = line.split(" ");
        let mut hand1: Hand = [0; 5];
        parts.next().unwrap().as_bytes()[0..5]
            .iter()
            .map(|&b| lookup[b as usize])
            .enumerate()
            .for_each(|(i, v)| hand1[i] = v);
        let bid = parts.next().unwrap().parse::<T>().unwrap();
        let mut hand2 = hand1.clone();
        for i in 0..5 {
            if hand2[i] == 9 {
                hand2[i] = 0;
            } else if hand2[i] < 9 {
                hand2[i] += 1;
            }
        }
        part1.push((hand_type(hand1), hand1, bid));
        part2.push((best_type(hand2), hand2, bid));
    }
    part1.sort_unstable();
    part2.sort_unstable();

    (
        part1.iter().enumerate().map(|(i, h)| (i + 1) * h.2).sum(),
        part2.iter().enumerate().map(|(i, h)| (i + 1) * h.2).sum(),
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

fn score_count(freq: [usize; 13]) -> [T; 2] {
    let mut res = [0; 2];
    for &c in freq.iter().filter(|&c| *c > 1) {
        if c >= res[0] {
            res[1] = res[0];
            res[0] = c;
        } else if c >= res[1] {
            res[1] = c;
        }
    }
    res
}

fn hand_type(hand: Hand) -> [T; 2] {
    let mut freq = [0 as T; 13];
    for card in hand {
        freq[card] += 1;
    }
    score_count(freq)
}

pub fn part1(hands: &ParsedData) -> usize {
    hands.0
}

fn best_type(hand: Hand) -> [T; 2] {
    if !hand.iter().any(|&c| c == 0) {
        return hand_type(hand);
    }

    let mut freq = [0 as T; 13];
    let (mut max_val, mut max_pos) = (0, 0);
    for c in hand {
        freq[c] += 1;
        if freq[c] > max_val && c != 0 {
            (max_val, max_pos) = (freq[c], c);
        }
    }

    let js = freq[0];
    freq[0] = 0;
    freq[max_pos] += js;

    score_count(freq)
}

pub fn part2(hands: &ParsedData) -> usize {
    hands.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part1(&parse_str(str_input)), 6440);
        assert_eq!(part2(&parse_str(str_input)), 5905);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/07.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 251806792);
        assert_eq!(part2(&input), 252113488);
    }
}
