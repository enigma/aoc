use std::fs;

pub type ParsedData = Vec<Option<isize>>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = vec![];
    contents.trim().lines().for_each(|line| {
        res.push(match line {
            "noop" => None,
            _ => Some(line[5..].parse::<isize>().unwrap()),
        });
    });
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn runner(instructions: &ParsedData) -> impl Iterator<Item = (usize, isize)> + '_ {
    let mut value = 1;
    let mut add = false;
    let mut instrs = instructions.iter();
    let mut last_add = 0;
    (1..241).map(move |cycle| {
        if add {
            add = false;
            value += last_add;
            (cycle, (value - last_add))
        } else if let Some(arg) = instrs.next().unwrap() {
            last_add = *arg;
            add = true;
            (cycle, value)
        } else {
            (cycle, value)
        }
    })
}

// y2022 d10 part1 full    time:   [17.533 µs 17.701 µs 17.877 µs]
pub fn part1(instr: &ParsedData) -> usize {
    let mut strength = 0;
    for (cycle, value) in &mut runner(instr) {
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                strength += cycle * (value as usize);
            }
            _ => {}
        }
    }
    strength
}

// y2022 d10 part2 full    time:   [18.106 µs 18.390 µs 18.706 µs]
pub fn part2(instr: &ParsedData) -> String {
    let mut crt = [['.'; 40]; 6];
    for (cycle, value) in &mut runner(instr) {
        let pixel = (cycle - 1) % 40;
        crt[((cycle - 1) % 240) / 40][pixel] =
            if (value - 1 <= pixel as isize) && (pixel as isize <= value + 1) {
                '#'
            } else {
                '.'
            };
    }
    let mut res = String::new();
    crt.iter().for_each(|row| {
        res.push_str(&String::from_iter(row.iter()));
        res.push('\n');
    });
    res.pop();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(part1(&parse_str(str_input)), 13140);
        assert_eq!(
            part2(&parse_str(str_input)),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/10.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 16060);
        assert_eq!(
            part2(&input),
            "###...##...##..####.#..#.#....#..#.####.
#..#.#..#.#..#.#....#.#..#....#..#.#....
###..#..#.#....###..##...#....####.###..
#..#.####.#....#....#.#..#....#..#.#....
#..#.#..#.#..#.#....#.#..#....#..#.#....
###..#..#..##..####.#..#.####.#..#.#...."
        );
    }
}
