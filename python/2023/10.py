import sys
from collections import defaultdict, deque


DI = {(0, -1): "-J7",
      (0, 1): "-FL",
      (1, 0): "|F7",
      (-1, 0): "|JL"}

D = defaultdict(set)
for k, v in DI.items():
    for i in v:
        D[i].add(k)


def neigh(m, pos):
    H, W = len(m), len(m[0])
    py, px = pos
    if not (0 <= py < H and 0 <= px < W):
        return
    for (ny, nx) in m[py][px]:
        if 0 <= ny <= H and 0 <= nx < W:
            yield (ny, nx)


def parse(data):
    rows = []
    start = None
    for y, line in enumerate(data.splitlines()):
        res = []
        for x, v in enumerate(line):
            if v == 'S':
                start = (y, x)
                res.append([])
            else:
                res.append([(y+dy, x+dx) for dy, dx in D[v]])
        rows.append(res)
    for n in [(start[0]+dy, start[1]+dx) for dy, dx in DI.keys()]:
        for o in neigh(rows, n):
            if o == start:
                rows[start[0]][start[1]].append(n)

    return list(data.splitlines()), rows, start


def fill(m, loop, f):
    H, W = len(m), len(m[0])
    seen = set()
    fringe = list(f)
    while fringe:
        p = py, px = fringe.pop()
        if p in seen or p in loop:
            continue
        seen.add(p)
        for dy, dx in DI.keys():
            ny, nx = py + dy, px + dx
            if 0 <= ny < H and 0 <= nx < W:
                fringe.append((ny, nx))
    return len(seen)


def track(raw, cur, dy, dx, left, right):
    y, x = cur
    match raw[y][x]:
        case '|':
            left.add((y, x + dy))
            right.add((y, x - dy))
        case '-':
            left.add((y - dx, x))
            right.add((y + dx, x))
        case 'J':
            side = [right, left][dy == 0]
            side.add((y, x + 1))
            side.add((y+1, x))
        case 'L':
            side = [right, left][dy == -1]
            side.add((y, x - 1))
            side.add((y+1, x))
        case '7':
            side = [right, left][dy == 1]
            side.add((y - 1, x))
            side.add((y, x + 1))
        case 'F':
            side = [right, left][dy == 0]
            side.add((y, x + 1))
            side.add((y-1, x))
        case 'S':
            pass
        case _:
            print("WTF", raw[y][x], dy, dx)


def solve(raw, m, start):
    loop = set()
    cur = start
    dy, dx = [b-a for a, b in zip(cur, next(neigh(m, cur)))]
    left, right = set(), set()
    while cur not in loop:
        loop.add(cur)
        track(raw, cur, dy, dx, left, right)
        prev = cur
        cur = cur[0] + dy, cur[1] + dx
        nxt = next(i for i in neigh(m, cur) if i != prev)
        dy, dx = [b-a for a, b in zip(cur, nxt)]

    return len(loop) // 2, min(fill(m, loop, i) for i in [left, right])


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
p1, p2 = solve(*parsed)
print(p1)
print(p2)

assert p1 == 7086, p1
assert p2 == 317, p2
