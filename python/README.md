### Dev cycle

```
YEAR=2025; DAY=3; S=python/$YEAR/$DAY.py; sh -c "ls $S | entr -r -c time python $S inputs/$YEAR/$(printf '%02d' $DAY).input"
```



### Save hyperfine benchmarks

```
YEAR=2025 PY=$(uv python list | rg -v download | awk '{print $2}' | head -n 1); cmds=("$PY -c 'exit()'"); for i in {1..25}; do cmds+=("$PY $YEAR/$i.py ../inputs/$YEAR/$(printf '%02d' $i).input"); done; hyperfine --warmup 5 --export-markdown hyperfine.$YEAR.md $cmds
```