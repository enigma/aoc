use std::fs;

pub type ParsedData = Vec<(char, char)>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        let opponent = line.chars().nth(0).unwrap();
        let strategy = line.chars().nth(2).unwrap();
        res.push((opponent, strategy));
    }
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn score(them: usize, me: usize) -> usize {
    let mut points = me + 1;
    if them == me {
        points += 3;
    }
    if me == (them + 1) % 3 {
        points += 6;
    }
    points
}

// y2022 d02 part1         time:   [81.435 µs 81.815 µs 82.200 µs]
pub fn part1(numbers: &ParsedData) -> usize {
    let mut total_score = 0;
    for (opponent, strategy) in numbers.iter() {
        let them = "ABC".find(*opponent).unwrap();
        let me = "XYZ".find(*strategy).unwrap();
        total_score += score(them, me);
    }
    total_score
}

// y2022 d02 part2         time:   [81.758 µs 82.027 µs 82.302 µs]
pub fn part2(numbers: &ParsedData) -> usize {
    let mut total_score = 0;
    for (opponent, strategy) in numbers.iter() {
        let them = "ABC".find(*opponent).unwrap();
        let me = (3 + them + "XYZ".find(*strategy).unwrap() - 1) % 3;
        total_score += score(them, me);
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "A Y
B X
C Z";
        assert_eq!(part1(&parse_str(str_input)), 15);
        assert_eq!(part2(&parse_str(str_input)), 12);
    }

    #[test]
    fn actual() {
        let numbers = parse(&"../inputs/2022/02.input".to_string());
        assert_eq!(part1(&numbers), 13675);
        assert_eq!(part2(&numbers), 14184);
    }
}
