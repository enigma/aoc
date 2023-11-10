use std::fs;

use hashbrown::HashMap;

pub type ParsedData = Vec<Jet>;
type Piece = u32;

const PIECES: [Piece; 5] = [
    (0b0011110 << (7 * 3)),
    (0b0001000 << (7 * 3)) + (0b0011100 << (7 * 2)) + (0b0001000 << 7),
    (0b0011100 << (7 * 3)) + (0b0000100 << (7 * 2)) + (0b0000100 << 7),
    (0b0010000 << (7 * 3)) + (0b0010000 << (7 * 2)) + (0b0010000 << 7) + 0b0010000,
    (0b0011000 << (7 * 3)) + (0b0011000 << (7 * 2)),
];
pub enum Jet {
    Left,
    Right,
}

impl Jet {
    const LEFT: u32 = (0b1000000 << 7 * 3) + (0b1000000 << 7 * 2) + (0b1000000 << 7) + 0b1000000;
    const RIGHT: u32 = Jet::LEFT >> 6;
    fn push(&self, p: Piece) -> Option<Piece> {
        match *self {
            Jet::Left => {
                if Jet::LEFT & p > 0 {
                    None
                } else {
                    Some(p << 1)
                }
            }
            Jet::Right => {
                if Jet::RIGHT & p > 0 {
                    None
                } else {
                    Some(p >> 1)
                }
            }
        }
    }
}

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    contents
        .trim()
        .as_bytes()
        .iter()
        .map(|&c| if c == b'>' { Jet::Right } else { Jet::Left })
        .collect()
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn solve(instr: &ParsedData, goal: usize) -> usize {
    let mut jets = instr.iter().cycle();
    let mut placed = [0u32; 5_000];
    let mut highest = 0;
    let mut cache = HashMap::new();
    let mut instr_id = instr.len() - 1;
    for (n, (piece_id, piece)) in PIECES.iter().enumerate().cycle().enumerate() {
        let mut y = highest + 3;
        let mut next_piece = *piece;
        let mut board = 0;
        loop {
            instr_id = (instr_id + 1) % instr.len();

            // Jet pushes left or right
            if let Some(plausible) = jets.next().unwrap().push(next_piece) {
                if board & plausible == 0 {
                    next_piece = plausible;
                }
            }

            // Drop one
            if y > 0 {
                board = (board >> 7) + (placed[y - 1] << (7 * 3));
                if (next_piece & board) == 0 {
                    y -= 1;
                    continue;
                }
            }

            // Stop
            let mask = 0b1111111;
            placed[y + 0] |= (next_piece >> (7 * 3)) & mask;
            placed[y + 1] |= (next_piece >> (7 * 2)) & mask;
            placed[y + 2] |= (next_piece >> (7 * 1)) & mask;
            placed[y + 3] |= (next_piece >> (7 * 0)) & mask;
            highest = ((y..y + 4).filter(|&i| placed[i] > 0).max().unwrap() + 1).max(highest);
            break;
        }
        let key = (instr_id, piece_id);
        if let Some(prev) = cache.get(&key) {
            let (prev_n, prev_highest) = prev;
            let h_cycle = highest - prev_highest;
            let p_cycle = n - prev_n;
            let num = goal - n - 1;
            if num % p_cycle == 0 {
                return (num / p_cycle) * h_cycle + highest;
            }
        } else {
            cache.insert(key, (n, highest));
        }
        if n + 1 == goal {
            return highest;
        }
    }
    unreachable!();
}

// y2022 d17 part1 full    time:   [81.866 µs 82.396 µs 82.872 µs]
pub fn part1(pd: &ParsedData) -> usize {
    solve(pd, 2022)
}

// y2022 d17 part2 full    time:   [100.04 µs 100.82 µs 101.67 µs]
pub fn part2(pd: &ParsedData) -> usize {
    solve(pd, 1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";
        assert_eq!(part1(&parse_str(str_input)), 3068);
        assert_eq!(part2(&parse_str(str_input)), 1514285714288);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/17.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 3193);
        assert_eq!(part2(&input), 1577650429835);
    }
}
