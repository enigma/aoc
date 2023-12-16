#![feature(int_roundings)]

//pub mod y2020;
//pub mod y2021;
pub mod y2022;
pub mod y2023;

#[macro_export]
macro_rules! aoc_main {
    ( $year:expr, $($day:expr => $module:ident),* $(,)?) => {

        fn main() {
            let year = $year;
            let args: Vec<_> = std::env::args().collect();

            if args.len() == 1 {
                eprintln!("Running all y{} on default path", year);
                $(
                        let day = $day;
                        let path = format!("../inputs/{}/{:02}.input", year, day);
                        let parsing = $module::parse(&path);
                        let part1 = $module::part1(&parsing);
                        println!("y{}::d{:02} part1: {}", year, day, part1);
                        let part2 = $module::part2(&parsing);
                        println!("y{}::d{:02} part2: {}", year, day, part2);
                        println!();
                )*
                return;
            }

            if args[1].starts_with("-p") {
                use rayon::iter::IntoParallelIterator;
                use rayon::iter::ParallelIterator;
                use rayon::iter::IndexedParallelIterator;
                use std::fmt::Display;
                type Res = String;
                let mut lambdas = vec![];

                eprintln!("Running all y{} on default path, in parallel", year);
                $(
                    lambdas.push(Box::new(|| {
                        let day = $day;
                        let path = format!("../inputs/{}/{:02}.input", year, day);
                        let parsing = $module::parse(&path);
                        let part1 = $module::part1(&parsing);
                        let part2 = $module::part2(&parsing);
                        format!(
                            "y{}::d{:02} part1: {}\ny{}::d{:02} part2: {}\n",
                            year, day, part1, year, day, part2
                        ).to_string()
                    }) as Box<dyn Fn() -> Res + Send + Sync>);
                )*

                let mut v: Vec<(usize, String)> = lambdas.into_par_iter().enumerate()
                    .map(|(i, lambda)| (i, lambda())).collect();
                v.sort();
                v.iter().for_each(|(_, s)| println!("{}", s));
                return;
            }

            let day = args[1]
                .parse::<usize>()
                .expect(&format!("Must pass an integer day, found {:?}", args[1]));

            let path = if args.len() > 2 {
                args[2].clone()
            } else {
                format!("../inputs/{}/{:02}.input", year, day)
            };

            eprintln!("Running y{}::d{:02} on path '{}'", year, day, path);

            match day {
                $(
                    $day => {
                        let parsing = $module::parse(&path);
                        let part1 = $module::part1(&parsing);
                        println!("y{}::d{:02} part1: {}", year, day, part1);
                        let part2 = $module::part2(&parsing);
                        println!("y{}::d{:02} part2: {}", year, day, part2);
                    }
                )*
                _ => panic!("y{}::d{:02} not found", year, day)
            }
        }
    };
}

#[macro_export]
macro_rules! aoc_bench {
    ( $year:expr, $($day:expr => $module:ident),* $(,)?) => {
        use criterion::{criterion_group, criterion_main, Criterion};
        pub fn bench_singles(c: &mut Criterion) {
            $(
                let path = format!("../inputs/{}/{:02}.input", $year, $day);
                let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file.");
                c.bench_function(&format!("y{} d{:02} parsing", $year, $day), |b| b.iter(|| $module::parse_str(&contents)));
                let parsing = $module::parse_str(&contents);
                c.bench_function(&format!("y{} d{:02} part1 noparsing", $year, $day), |b| b.iter(|| $module::part1(&parsing)));
                c.bench_function(&format!("y{} d{:02} part2 noparsing", $year, $day), |b| b.iter(|| $module::part2(&parsing)));
                c.bench_function(&format!("y{} d{:02} part1 full",      $year, $day), |b| b.iter(|| $module::part1(&$module::parse_str(&contents))));
                c.bench_function(&format!("y{} d{:02} part2 full",      $year, $day), |b| b.iter(|| $module::part2(&$module::parse_str(&contents))));
                c.bench_function(&format!("y{} d{:02} complete", $year, $day), |b| {
                    b.iter(|| {
                        let pd = &$module::parse_str(&contents);
                        $module::part1(&pd);
                        $module::part2(&pd);
                    });
                });
            )*
        }

        pub fn bench_total(c: &mut Criterion) {
            c.bench_function(
                &format!("y{} total", $year), |b| b.iter(|| {
                    $(
                        let path = format!("../inputs/{}/{:02}.input", $year, $day);
                        let parsed = $module::parse(&path);
                        $module::part1(&parsed);
                        $module::part2(&parsed);
                    )*
                })
            );
        }

        criterion_group!(benches, bench_singles, bench_total);

        criterion_main!(benches);
    };
}
