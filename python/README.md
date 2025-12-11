### Dev cycle

```
YEAR=2025; DAY=3; S=python/$YEAR/$DAY.py; sh -c "ls $S | entr -r -c time python $S inputs/$YEAR/$(printf '%02d' $DAY).input"
```



### Save hyperfine benchmarks

```
bb hyperfine
```
