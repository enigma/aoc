use std::fs;

use hashbrown::HashMap;

pub type ParsedData = HashMap<String, usize>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut fs = HashMap::new();
    let mut pwd: Vec<String> = vec![];
    contents.trim().lines().for_each(|line| {
        if line.chars().next().unwrap() == '$' {
            let mut cmd_line = line.split(" ");
            cmd_line.next();
            let cmd = cmd_line.next().unwrap();
            if cmd == "cd" {
                match cmd_line.next().unwrap() {
                    "/" => {
                        pwd.clear();
                    }
                    ".." => {
                        pwd.pop();
                    }
                    path => {
                        let mut new_path = pwd.last().unwrap_or(&"".to_string()).to_owned();
                        new_path.push_str("/");
                        new_path.push_str(path);
                        pwd.push(new_path.to_string());
                    }
                }
            }
        } else {
            let mut res_line = line.split(" ");
            let size = res_line.next().unwrap();
            let fsize = if size == "dir" {
                0
            } else {
                size.to_string().parse::<usize>().unwrap()
            };
            let df = "".to_string();
            let mut fname = String::new();
            fname.push_str(pwd.last().unwrap_or(&df));
            fname.push_str("/");
            fname.push_str(res_line.next().unwrap());
            fs.insert(fname.clone(), fsize);
        }
    });
    fs
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

fn sizes(fs: &ParsedData) -> HashMap<String, usize> {
    let mut paths: Vec<String> = vec![];
    fs.keys().for_each(|path| paths.push(path.clone()));
    paths.sort_by_key(|k| usize::MAX - k.chars().filter(|c| *c == '/').count());

    let mut total_sizes = HashMap::new();
    for path in paths.iter() {
        let ts = fs[&*path];
        let mut parent = String::new();
        let mut spath = path.split("/").peekable();
        while let Some(part) = spath.next() {
            if spath.peek().is_some() {
                parent.push_str(part);
                parent.push_str("/");
                let ancestor = total_sizes.entry(parent.clone()).or_insert(0);
                *ancestor += ts;
            }
        }
    }
    total_sizes
}

// y2022 d07 part1 full    time:   [525.48 µs 526.74 µs 528.12 µs]
pub fn part1(fs: &ParsedData) -> usize {
    sizes(fs).values().filter(|&&v| v <= 100_000).sum()
}

// y2022 d07 part2 full    time:   [527.61 µs 530.29 µs 534.28 µs]
pub fn part2(fs: &ParsedData) -> usize {
    let binding = sizes(fs);
    let total_sizes: Vec<&usize> = binding.values().collect();
    let need_gone = *total_sizes.iter().max().unwrap() + 30_000_000 - 70_000_000;
    **total_sizes
        .iter()
        .filter(|i| **i >= &need_gone)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(part1(&parse_str(str_input)), 95437);
        assert_eq!(part2(&parse_str(str_input)), 24933642);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/07.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 919137);
        assert_eq!(part2(&input), 2877389);
    }
}
