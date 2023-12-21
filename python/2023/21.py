import sys
from collections import deque
from functools import cache


def parse(data):
    grid = []
    start = None
    for y, line in enumerate(data.splitlines()):
        if (x := line.find('S')) > -1:
            start = (y, x)
        grid.append(line.replace('S', '.'))
    return grid, start


DY = [0, 1, 0, -1]
DX = [1, 0, -1, 0]
DIRS = EAST, SOUTH, WEST, NORTH = list(zip(DY, DX))


def solve(grid, start):
    h, w = len(grid), len(grid[0])
    assert start[0] == start[1], start
    assert h == w, (h, w)
    side = h

    @cache
    def count_from(start, goal, d=0):
        if d > 0:
            return count_from(start, goal-d)
        if goal < 0:
            return 0
        if goal > 3*h*w:
            return count_from(start, 2*h*w + goal % 2)
        seen = set()
        y, x = start
        y %= h
        x %= w
        fringe = deque([(d, (y, x))])
        steps = [0, 0]
        while fringe:
            d, cur = fringe.popleft()
            if cur in seen:
                continue
            seen.add(cur)
            if d > goal:
                continue
            steps[d % 2] += 1
            y, x = cur
            for dy, dx in DIRS:
                ny = y + dy
                nx = x + dx
                if 0 <= ny < h and 0 <= nx < w and grid[ny][nx] != '#':
                    fringe.append((d + 1, (ny, nx)))
        return steps[goal % 2]
    p1 = count_from(start, 64)

    p2steps = 26501365
    gw = p2steps // h - 1

    full_odd = count_from(start, 2 * side + 1)
    full_even = count_from(start, 2 * side)

    y, x = start
    even_grids = ((gw + 1) // 2 * 2) ** 2
    odd_grids = (gw // 2 * 2 + 1) ** 2

    p2 = 0
    p2 += even_grids * full_even
    p2 += odd_grids * full_odd

    p2 += count_from((y, 0), h - 1)
    p2 += count_from((y, side - 1), h - 1)
    p2 += count_from((0, x), h - 1)
    p2 += count_from((side - 1, x), h - 1)

    p2 += (gw + 1) * count_from((side - 1, 0), side // 2 - 1)
    p2 += (gw + 1) * count_from((side - 1, side - 1), side // 2 - 1)
    p2 += (gw + 1) * count_from((0, 0), side // 2 - 1)
    p2 += (gw + 1) * count_from((0, side - 1), side // 2 - 1)

    p2 += gw * count_from((side - 1, 0), side * 3 // 2 - 1)
    p2 += gw * count_from((side - 1, side - 1), side * 3 // 2 - 1)
    p2 += gw * count_from((0, 0), side * 3 // 2 - 1)
    p2 += gw * count_from((0, side - 1), side * 3 // 2 - 1)

    return p1, p2


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
p1, p2 = solve(*parsed)
print(p1)
print(p2)

assert p1 == 3651, p1
assert p2 == 607334325965751, p2
