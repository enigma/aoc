import sys


def parse(data):
    for y, line in enumerate(data.splitlines()):
        for x, v in enumerate(line):
            if v == '#':
                yield (y, x)


def solve(grid):
    rows = set(i for i, _ in grid)
    cols = set(i for _, i in grid)
    missing_rows = [i for i in range(max(rows)+1) if i not in rows]
    missing_cols = [i for i in range(max(cols)+1) if i not in cols]
    result = []
    for factor in [2, 1_000_000]:
        egrid = [(y + sum(factor-1 for i in missing_rows if i < y),
                  x + sum(factor-1 for i in missing_cols if i < x))
                 for (y, x) in grid]

        result.append(sum(abs(y1-y2) + abs(x1-x2)
                          for i, (y1, x1) in enumerate(egrid)
                          for (y2, x2) in egrid[i+1:]))
    return result


data = open(sys.argv[-1]).read()
parsed = list(parse(data.strip()))
p1, p2 = solve(parsed)
print(p1)
print(p2)

assert p1 == 10494813, p1
assert p2 == 840988812853, p2
