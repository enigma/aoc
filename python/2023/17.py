import sys
from heapq import heappop, heappush


def parse(data):
    return [[int(i) for i in line] for line in data.splitlines()]


DY = [0, 1, 0, -1]
DX = [1, 0, -1, 0]
DIRS = tuple(zip(DY, DX))


def min_loss(grid, mmin, mmax):
    h, w = len(grid), len(grid[0])
    fringe = [(0, 0, (0, 0), DIRS)]
    seen = set()
    while fringe:
        _heu, lost, (y, x), viable_dirs = heappop(fringe)
        if (y, x) == (h-1, w-1):
            return lost
        k = ((y, x), viable_dirs)
        if k in seen:
            continue
        seen.add(k)
        for dy, dx in viable_dirs:
            nxt_viable = tuple(i for i in DIRS if i not in ((-dy, -dx), (dy, dx)))
            nxt_lost = lost
            for i in range(1, mmax+1):
                ny = y + i * dy
                nx = x + i * dx
                if 0 <= ny < h and 0 <= nx < w:
                    nxt_lost += grid[ny][nx]
                    if i < mmin:
                        continue
                    nxt_heu = nxt_lost + abs(h-ny) + abs(w-nx)
                    heappush(fringe, (nxt_heu, nxt_lost, (ny, nx), nxt_viable))


def solve(grid):
    p1 = min_loss(grid, 1, 3)
    p2 = min_loss(grid, 4, 10)
    return p1, p2


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
p1, p2 = solve(parsed)
print(p1)
print(p2)

assert p1 == 785, p1
assert p2 == 922, p2
