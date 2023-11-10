use std::fs;

pub type ParsedData = Vec<String>;

#[inline]
pub fn parse_str(_contents: &str) -> ParsedData {
    unimplemented!();
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

pub fn part1(_x: &ParsedData) -> usize {
    0
}

pub fn part2(_x: &ParsedData) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "";
        assert_eq!(part1(&parse_str(str_input)), 2);
        assert_eq!(part2(&parse_str(str_input)), 2);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 245);
        assert_eq!(part2(&input), 133);
    }
}
