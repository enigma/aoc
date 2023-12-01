### Save hyperfine benchmarks

YEAR=2023; cmds=(); for i in {1..25}; do cmds+=("python $YEAR/$i.py ../inputs/$YEAR/$(printf '%02d' $i).input"); done; hyperfine --warmup 5 --export-markdown hyperfine.$YEAR.md $cmds
