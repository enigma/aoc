use core::panic;
use std::fs;

use hashbrown::HashMap;

type Q = u64;

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum Op {
    Add,
    Mul,
    Sub,
    Div,
}

#[derive(Debug)]
pub enum Monkey {
    Value(Q),
    Lazy(Op, String, String),
}

pub type ParsedData = HashMap<String, Monkey>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = HashMap::with_capacity(3000);
    contents.trim().lines().for_each(|line| {
        let mut parts = line.split(": ");
        let alias = parts.next().unwrap().to_string();
        let mut parts = parts.next().unwrap().split(" ");
        let lhs = parts.next().unwrap();
        if let Some(op_str) = parts.next() {
            let rhs = parts.next().unwrap().to_string();
            let op = match op_str {
                "+" => Op::Add,
                "*" => Op::Mul,
                "-" => Op::Sub,
                "/" => Op::Div,
                _ => panic!("What's this? {}", op_str),
            };
            res.insert(alias, Monkey::Lazy(op, lhs.to_string(), rhs));
        } else {
            res.insert(alias, Monkey::Value(lhs.parse::<Q>().unwrap()));
        }
    });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn eval(monkey: &str, data: &ParsedData) -> Q {
    let val = data.get(monkey).unwrap();
    match val {
        Monkey::Value(v) => *v,
        Monkey::Lazy(Op::Add, lhs, rhs) => eval(lhs, data) + eval(rhs, data),
        Monkey::Lazy(Op::Sub, lhs, rhs) => eval(lhs, data) - eval(rhs, data),
        Monkey::Lazy(Op::Mul, lhs, rhs) => eval(lhs, data) * eval(rhs, data),
        Monkey::Lazy(Op::Div, lhs, rhs) => eval(lhs, data).div_floor(eval(rhs, data)),
    }
}

pub fn part1(data: &ParsedData) -> usize {
    eval(&"root".to_string(), data) as usize
}

#[derive(PartialEq, Clone)]
enum InvMonkey {
    Human,
    Value(Q),
    Lazy(Op, Box<InvMonkey>, Box<InvMonkey>),
}

fn eval_inv(key: String, data: &ParsedData) -> (bool, InvMonkey) {
    if key == "humn" {
        (true, InvMonkey::Human)
    } else {
        match data.get(&key).unwrap() {
            Monkey::Value(v) => (false, InvMonkey::Value(*v)),
            Monkey::Lazy(op, lhs, rhs) => {
                let left = eval_inv(lhs.clone(), data);
                let right = eval_inv(rhs.clone(), data);
                match (left.0 || right.0, left.1, right.1) {
                    (true, l, r) => (true, InvMonkey::Lazy(*op, Box::new(l), Box::new(r))),
                    (_, InvMonkey::Value(lv), InvMonkey::Value(rv)) => (
                        false,
                        InvMonkey::Value(match op {
                            Op::Add => lv + rv,
                            Op::Mul => lv * rv,
                            Op::Sub => lv - rv,
                            Op::Div => lv.div_floor(rv),
                        }),
                    ),
                    _ => unreachable!(),
                }
            }
        }
    }
}

pub fn part2(data: &ParsedData) -> usize {
    let root = data.get("root").unwrap();
    let (lhs, rhs) = match root {
        Monkey::Lazy(_, lhs, rhs) => (lhs, rhs),
        _ => unreachable!(),
    };
    let (lhs, rhs) = (eval_inv(lhs.clone(), data), eval_inv(rhs.clone(), data));
    let (mut mine, other) = if lhs.0 {
        (lhs.1, rhs.1)
    } else {
        (rhs.1, lhs.1)
    };
    let mut other = match other {
        InvMonkey::Value(v) => v,
        _ => unreachable!(),
    };
    loop {
        match mine {
            InvMonkey::Human => return other as usize,
            InvMonkey::Lazy(op, lhs, rhs) => {
                (other, mine) = match (*lhs, *rhs) {
                    (InvMonkey::Value(n), rhs) => match op {
                        Op::Add => (other - n, rhs),
                        Op::Mul => (other / n, rhs),
                        Op::Sub => (n - other, rhs),
                        Op::Div => (n / other, rhs),
                    },
                    (lhs, InvMonkey::Value(n)) => match op {
                        Op::Add => (other - n, lhs),
                        Op::Mul => (other / n, lhs),
                        Op::Sub => (other + n, lhs),
                        Op::Div => (other * n, lhs),
                    },
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
        assert_eq!(part1(&parse_str(str_input)), 152);
        assert_eq!(part2(&parse_str(str_input)), 301);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/21.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 31017034894002);
        assert_eq!(part2(&input), 3555057453229);
    }
}
