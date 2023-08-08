use std::fs;

type D = i64;
pub type ParsedData = D;

const DIGITS: [u8; 5] = [b'=', b'-', b'0', b'1', b'2'];

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut total = 0;
    contents.trim().lines().for_each(|line| {
        let mut res = 0;
        for c in line.as_bytes() {
            res = 5 * res + DIGITS.iter().position(|d| d == c).unwrap() - 2;
        }
        total += res;
    });
    total as D
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

pub fn part1(dec: &ParsedData) -> String {
    let mut n = *dec;
    let mut result = Vec::with_capacity(30);
    while n > 0 {
        let cur = (n + 2).rem_euclid(5);
        result.push(DIGITS[cur as usize]);
        n = (n - (cur - 2)) / 5;
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

pub fn part2(_x: &ParsedData) -> String {
    "25!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";
        assert_eq!(part1(&parse_str(str_input)), "2=-1=0");
        assert_eq!(part2(&parse_str(str_input)), "25!");
    }

    #[test]
    fn actual() {
        let path = &"inputs/2022/25.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), "2-1=10=1=1==2-1=-221");
        assert_eq!(part2(&input), "25!");
    }
}
