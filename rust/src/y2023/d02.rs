use std::fs;

type T = usize;
type Game = (T, Vec<(T, T, T)>);
pub type ParsedData = Vec<Game>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    contents.trim().lines().for_each(|l| {
        let mut game: Game = (0, vec![]);
        let mut gamep = l.split(": ");
        game.0 = gamep.next().unwrap()[5..].parse().unwrap();
        let mut subsets = gamep.next().unwrap().split("; ");
        while let Some(subset) = subsets.next() {
            let mut parts = subset.split(", ");
            let mut conf = (0, 0, 0);
            while let Some(part) = parts.next() {
                let mut qty = part.split(" ");
                let number = qty.next().unwrap().parse().unwrap();
                match qty.next().unwrap().chars().nth(0).unwrap() {
                    'r' => conf.0 = number,
                    'g' => conf.1 = number,
                    'b' => conf.2 = number,
                    _ => panic!("What happened? {:?}", part),
                }
            }
            game.1.push(conf);
        }
        res.push(game);
    });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect(&format!(
        "Something went wrong reading the file {:?}.",
        path
    ));
    parse_str(&contents)
}

pub fn part1(games: &ParsedData) -> usize {
    games
        .iter()
        .filter(|&g| {
            for s in g.1.iter() {
                if s.0 > 12 || s.1 > 13 || s.2 > 14 {
                    return false;
                }
            }
            true
        })
        .map(|g| g.0)
        .sum()
}

pub fn part2(games: &ParsedData) -> usize {
    games
        .iter()
        .map(|g| {
            let mut r = (0, 0, 0);
            for s in g.1.iter() {
                r.0 = r.0.max(s.0);
                r.1 = r.1.max(s.1);
                r.2 = r.2.max(s.2);
            }
            r.0 * r.1 * r.2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "";
        // assert_eq!(part1(&parse_str(str_input)), 2);
        // assert_eq!(part2(&parse_str(str_input)), 2);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2023/02.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 2162);
        assert_eq!(part2(&input), 72513);
    }
}
