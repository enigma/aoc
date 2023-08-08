use sscanf::sscanf;
use std::fs;

enum Op {
    Square,
    Mul(usize),
    Add(usize),
}

pub struct Monkey {
    id: usize,
    items: Vec<usize>,
    op: Op,
    test: usize,
    pos: usize,
    neg: usize,
}
pub type ParsedData = Vec<Monkey>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    contents.trim().split("\n\n").for_each(|sm| {
        let (id, items, operation, test, pos, neg) = sscanf!(
            sm,
            "Monkey {usize}:
  Starting items: {str}
  Operation: new = old {str}
  Test: divisible by {usize}
    If true: throw to monkey {usize}
    If false: throw to monkey {usize}"
        )
        .unwrap();
        let mut pitem = vec![];
        items
            .split(", ")
            .for_each(|it| pitem.push(it.parse::<usize>().unwrap()));
        let op: Op = if operation == "* old" {
            Op::Square
        } else {
            let (c, arg) = sscanf!(operation, "{char} {usize}").unwrap();
            if c == '*' {
                Op::Mul(arg)
            } else {
                Op::Add(arg)
            }
        };
        let monkey = Monkey {
            id: id,
            items: pitem,
            op,
            test: test,
            pos: pos,
            neg: neg,
        };
        res.push(monkey);
    });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn solve(monkeys: &ParsedData, rounds: usize, div: usize, limit: usize) -> usize {
    let mut counter = [0; 10];
    let mut m_items = [[0usize; 50]; 10];
    let mut m_cnt = [0usize; 10];
    for (n, m) in monkeys.iter().enumerate() {
        for &i in &m.items {
            m_items[n][m_cnt[n]] = i;
            m_cnt[n] += 1;
        }
    }
    for _ in 0..rounds {
        for (n, m) in monkeys.iter().enumerate() {
            for i in 0..m_cnt[n] {
                let item = m_items[n][i];
                counter[m.id] += 1;
                let new_item = (match m.op {
                    Op::Square => item * item,
                    Op::Add(i) => item + i,
                    Op::Mul(i) => item * i,
                }) / div
                    % limit;
                let next_monkey = if new_item % m.test == 0 { m.pos } else { m.neg };
                m_items[next_monkey][m_cnt[next_monkey]] = new_item;
                m_cnt[next_monkey] += 1;
            }
            m_cnt[n] = 0;
        }
    }
    counter.sort_by(|a, b| b.cmp(a));
    counter[0] * counter[1]
}

// y2022 d11 part1 full    time:   [32.731 µs 32.947 µs 33.187 µs]
pub fn part1(monkeys: &ParsedData) -> usize {
    let mut cap = 1;
    monkeys.iter().for_each(|m| cap *= m.test);
    solve(monkeys, 20, 3, cap)
}

// y2022 d11 part2 full    time:   [1.2965 ms 1.3014 ms 1.3063 ms]
pub fn part2(monkeys: &ParsedData) -> usize {
    let mut cap = 1;
    monkeys.iter().for_each(|m| cap *= m.test);
    solve(monkeys, 10_000, 1, cap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        assert_eq!(part1(&parse_str(str_input)), 10605);
        assert_eq!(part2(&parse_str(str_input)), 2713310158);
    }

    #[test]
    fn actual() {
        let path = &"inputs/2022/11.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 95472);
        assert_eq!(part2(&input), 17926061332);
    }
}
