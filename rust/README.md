# AoC in Rust

## Perfomance

- [y2023 results](benches/y2023.md)  
- [y2022 results](benches/y2022.md)

## Objectives

1. Correcteness 
If it doesn't for [your input](#run-on-your-input) it's a bug, please let me know.
1. Run each day in under 1ms on my machine  
Arbitrary stop conditions, many solutions are far from there.
1. Easy to share  
To make code easier to share with others (and godbolt) each day should be copy-pastable modulo the import of some popular crates.

## Commands

### Run on your input

```sh
YEAR=2023; DAY=14
cargo build --release
target/release/y$YEAR $DAY path/to/your/input/file
```

### Run on `inputs/` for all days
```sh
YEAR=2023
cargo build --release
target/release/y$YEAR
```

### Test

```sh
YEAR=2023; DAY=d01; cargo test y$YEAR::$DAY 
```

### Benchmark:

```sh
YEAR=2023; DAY=d01; cargo bench --bench y$YEAR -- $DAY
```

### Dev cycle:

```sh
YEAR=2023; DAY=d01; cargo watch -x "test y$YEAR::$DAY && cargo bench --bench y$YEAR -- $DAY"
```

### Save criterion benchmark baseline:

```sh
YEAR=2023; cargo bench --bench y$YEAR -- --save-baseline base
python benches/bench_report.py target $YEAR > benches/y$YEAR.md
```

### Save hyperfine benchmarks:

```sh
YEAR=2023
hyperfine --warmup 3 --export-markdown benches/hyperfine.$YEAR.md -P day 1 25  "target/release/y$YEAR {day} inputs/$YEAR/\$(printf '%02d' {day}).input"
```

```sh
YEAR=2023; DAY=25; cmds=(); for i in {1..$DAY}; do cmds+=("./target/release/y$YEAR $i"); done;cmds+=("./target/release/y$YEAR"); cmds+=("./target/release/y$YEAR -parallel"); hyperfine --warmup 10 --export-markdown benches/hyperfine.$YEAR.md -N $cmds
```