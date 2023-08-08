import os
import sys
import json

from collections import defaultdict

BENCH_BASELINE = 'base'
DEN, UNIT = 1_000, 'µs'
DEN, UNIT = 1_000_000, 'ms'

yearly = defaultdict(list)
target = sys.argv[-2]
year = sys.argv[-1]
path = os.path.join(target, 'criterion')
for i in os.scandir(path):
    if not i.is_dir(): continue
    name = tuple(i.name.split())
    if name[0].startswith('y20') and year in name[0]:
        if any(i in name for i in 'parsing noparsing complete total'.split()):
            yearly[name[0]].append(i)

def read_performance(path):
    if not os.path.isfile(path):
        return None
    with open(path) as f:
        content = json.loads(f.read())
        return content

def format_timing(timing):
    if not timing:
        return '  '
    
    den = DEN
    median = timing['median']
    pe = median['point_estimate'] / den
    low = median['confidence_interval']['lower_bound'] / den
    upper = median['confidence_interval']['upper_bound'] / den
    if pe <= 1_000_000/den:
        return f'`{pe:.4f}`'
    return f'**__{pe:.2f}__**'
    if pe <= 1_000_000/den:
        return f'`{pe:.3f}`<br>  `[{low:.2f}-{upper:.2f}]`'
    return f'**__{pe:.3f}__**<br>`[{low:.2f}-{upper:.2f}]`'

for year, results in sorted(yearly.items()):
    COLUMNS = ['parsing', 'part1', 'part2', 'complete']
    by_day = defaultdict(dict)
    total = None
    for result in results:
        if 'total' in result.name:
            total = result
            continue
        name = result.name.split()
        day = int(name[1].replace('d', ''))
        if name[2] in COLUMNS:
            by_day[day][name[2]] = result
        else:
            print(name[2], name)
    results = []
    for day, entries in sorted(by_day.items()):
        timings = []
        for column in COLUMNS:
            if not (to_scan := entries.get(column)):
                timings.append(None)
                continue
            path = os.path.join(to_scan, BENCH_BASELINE, 'estimates.json')
            if content := read_performance(path):
                timings.append(content)
            else:
                timings.append(None)
        results.append((day, timings))

    if total:
        path = os.path.join(total, BENCH_BASELINE, 'estimates.json')
        if (content := read_performance(path)):
            results.append(('total', [None, None, None, content]))
    unit = f'({UNIT})'
    print(f'|{year}<br>Day|Parsing {unit}|Part 1 {unit}<br>(no parsing)|Part 2 {unit}<br>(no parsing)| Complete {unit}|')
    print(f'|-:|-:|-:|-:|-:|')
    for element, timings in results:
        row = '|'.join([str(element)] + list(map(format_timing, timings)))
        print(f'|{row}|')
