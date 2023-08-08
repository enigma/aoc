import sys
from collections import defaultdict, deque

def parse(text):
    grid = set()
    for y, row in enumerate(text.strip().split('\n')):
        for x, val in enumerate(row):
            if val == '#':
                grid.add((x, y))
    return grid

DX = [-1,  0,  1, -1,  0,  1, -1, -1, -1,  1,  1,  1]
DY = [-1, -1, -1,  1,  1,  1, -1,  0,  1, -1,  0,  1]
DS = list(zip(DX, DY))
DELTAS = set(DS)

DTS = [
    ((0, -1), DS[:3]),
    ((0, 1), DS[3:6]),
    ((-1, 0), DS[6:9]),
    ((1, 0), DS[9:]),
]

def run(data):
    grid = data
    rules = deque(DTS)
    while True:
        attempts = defaultdict(list)
        new_grid = set()
        moved = False
        for x, y in grid:
            if not any((x + dx, y + dy) in grid for dx, dy in DELTAS):
                new_grid.add((x, y))
                continue
            for (dest_dx, dest_dy), delta in rules:
                if not any((x + dx, y + dy) in grid for (dx, dy) in delta):
                    attempts[x + dest_dx, y + dest_dy].append((x, y))
                    break
            else:
                new_grid.add((x, y))
        for dest, sources in attempts.items():
            if len(sources) == 1:
                moved = True
                new_grid.add(dest)
            else:
                new_grid.update(sources)
        assert len(new_grid) == len(data), (len(data), len(new_grid))
        grid = new_grid
        yield grid
        rules.rotate(-1)
        if not moved:
            return

def part1(data):
    for round, grid in enumerate(run(data), 1):
        if round == 10:
            x, y = next(iter(grid))
            minx = maxx = x
            miny = maxy = y
            for x, y in grid:
                minx = min(minx, x)
                maxx = max(maxx, x)
                miny = min(miny, y)
                maxy = max(maxy, y)
            return (maxy - miny + 1) * (maxx - minx + 1) - len(grid)

def part2(data):
    return len(list(run(data)))
            
text = open(sys.argv[-1], 'r').read()
parsed = parse(text)
print(p1:=part1(parsed))
print(p2:=part2(parsed))

assert (p1, p2) == (3800, 916), (p1, p2)
