### Dev cycle

```
YEAR=2023; DAY=3; S=python/$YEAR/$DAY.py; sh -c "ls $S | entr -r -c time python $S inputs/$YEAR/$(printf '%02d' $DAY).input"
```



### Save hyperfine benchmarks

```
YEAR=2023; cmds=("python -c 'exit()'"); for i in {1..25}; do cmds+=("python $YEAR/$i.py ../inputs/$YEAR/$(printf '%02d' $i).input"); done; hyperfine --warmup 5 --export-markdown hyperfine.$YEAR.md $cmds
```