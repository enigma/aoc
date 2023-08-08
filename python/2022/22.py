import sys
from collections import deque
from operator import add, sub, mul, floordiv
from functools import cache

def parse(text):
    sgrid, letter = text.split('\n\n')
    grid = {}
    y_ranges = {}
    x_ranges = {}
    for y, row in enumerate(sgrid.split('\n')):
        x_min = x_max = None
        for x, val in enumerate(row):
            if val == ' ': continue
            x_min = x if x_min is None else min(x_min, x)
            x_max = x if x_max is None else max(x_max, x)
            y_min, y_max = y_ranges.setdefault(x, (y, y+1))
            y_ranges[x] = (min(y_min, y), max(y_max, y+1))
            grid[y, x] = val
        if x_min != None:
            x_ranges[y] = (x_min, x_max + 1)
    path = []
    i=0
    while i < len(letter.strip()):
        j = i
        while j < len(letter) and letter[j].isdigit():
            j += 1
        if j-i:
            path.append(int(letter[i:j]))
            i = j
        else:
            path.append(letter[i])
            i += 1
    return grid, path, x_ranges, y_ranges

from copy import deepcopy
def inrange(rng, i):
    a, b = rng
    if i < a: return b - 1
    if i >= b: return a
    return i

def part1(data):
    grid, letter, x_ranges, y_ranges = data
    y, x = min(grid.keys())
    dy, dx = 0, 1
    for do in letter:
        match do:
            case int():
                for _ in range(do):
                    ny, nx = y + dy, x + dx
                    if (ny, nx) not in grid:
                        if dy != 0 and nx in y_ranges:
                            ny = inrange(y_ranges[nx], ny)
                        if dx != 0 and ny in x_ranges:
                            nx = inrange(x_ranges[ny], nx)
                    if grid[ny, nx] == '.':
                        y, x = ny, nx
            case 'R':
                dy, dx = dx, -dy
            case 'L':
                dy, dx = -dx, dy
            case _: assert False, do
    password = [(0, 1), (1, 0), (0, -1), (-1, 0)].index((dy, dx))
    password += (x+1) * 4 + (y + 1) * 1000
    return password

def part2(data, side=50):
    grid, letter, x_ranges, y_ranges = data
    y, x = min(grid.keys())
    dy, dx = 0, 1
    for do in letter:
        match do:
            case int():
                for _ in range(do):
                    ny, nx = y + dy, x + dx
                    original_delta = (dy, dx)
                    if (ny, nx) not in grid:
                        fy = y // side
                        fx = x // side
                        assert y >= 0, (ny, y,x, fx)
                        assert x >= 0, (nx, y,x, fx)

                        match ((fy, fx), (dy, dx)):
                            case ((0, 1), (0, -1)):
                                ny, nx = 3 * side - y - 1, 0
                                assert (ny // side, nx // side) == (2, 0), (ny, nx)
                                dy, dx = 0, 1
                            case ((0, 1), (-1, 0)):
                                ny, nx = 3 * side + x % side, 0
                                assert (ny // side, nx // side) == (3, 0), (ny, nx)
                                dy, dx = 0, 1
                            # 0, 2
                            case ((0, 2), (-1, 0)):
                                ny, nx = 4 * side - 1, x % side
                                assert (ny // side, nx // side) == (3, 0), (ny, nx)
                                dy, dx = -1, 0 
                            case ((0, 2), (0, 1)):
                                ny, nx = 3 * side - y - 1, 2 * side - 1
                                assert (ny // side, nx // side) == (2, 1), (ny, nx)
                                dy, dx = 0, -1
                            case ((0, 2), (1, 0)):
                                ny, nx = 1 * side + x % side, 2 * side - 1
                                assert (ny // side, nx // side) == (1, 1), (ny, nx)
                                dy, dx = 0, -1

                            case ((1, 1), (0, -1)):
                                ny, nx = 2 * side, y % side
                                assert (ny // side, nx // side) == (2, 0), (ny, nx)
                                dy, dx = 1, 0
                            case ((1, 1), (0, 1)):
                                ny, nx = side - 1, 2 * side + y % side
                                assert (ny // side, nx // side) == (0, 2), (ny, nx)
                                dy, dx = -1, 0
                            # 2, 0
                            case ((2, 0), (-1, 0)):
                                ny, nx = side + x % side, side
                                assert (ny // side, nx // side) == (1, 1), (ny, nx)
                                dy, dx = (0, 1)
                            case ((2, 0), (0, -1)):
                                ny, nx = side - (y % side) - 1, side
                                assert (ny // side, nx // side) == (0, 1), (ny, nx)
                                dy, dx = 0, 1
                            # 2, 1
                            case ((2, 1), (0, 1)):
                                ny, nx = side - (y % side) - 1, 3 * side - 1
                                assert (ny // side, nx // side) == (0, 2), (ny, nx)
                                dy, dx = 0, -1
                            case ((2, 1), (1, 0)):
                                ny, nx = 3*side + (x % side), side - 1
                                assert (ny // side, nx // side) == (3, 0), (ny, nx)
                                dy, dx = 0, -1
                            # 3, 0
                            case ((3, 0), (0, -1)):
                                ny, nx = 0, side + y % side
                                assert (ny // side, nx // side) == (0, 1), (ny, nx)
                                dy, dx = 1, 0
                            case ((3, 0), (0, 1)):
                                ny, nx = 3 * side - 1, side + y % side
                                assert (ny // side, nx // side) == (2, 1), (ny, nx)
                                dy, dx = -1, 0
                            case ((3, 0), (1, 0)):
                                ny, nx = 0, 2 * side + x % side
                                assert (ny // side, nx // side) == (0, 2), (ny, nx)
                                dy, dx = 1, 0
                            case v:
                                assert False, ('wtf', v, (y, x))
                        assert (ny + dy, nx + dx) in grid, ((y // side, x // side), (dy, dx), (y, x), (ny, nx), (ny // side, nx // side))
                        assert (ny + dy * (side - 1), nx + dx * (side - 1)) in grid, ((y // side, x // side), (dy, dx), (y, x), (ny, nx), (ny // side, nx // side))
                    assert (ny, nx) in grid, ((y // side, x // side), (dy, dx), (y, x), (ny, nx), (ny // side, nx // side))

                    if grid[ny, nx]  == '.':
                        y, x = ny, nx
                    else:
                        dy, dx = original_delta
                        break
            case 'R':
                dy, dx = dx, -dy
            case 'L':
                dy, dx = -dx, dy
            case _: assert False, do
    password = [(0, 1), (1, 0), (0, -1), (-1, 0)].index((dy, dx))
    password += (x+1) * 4 + (y + 1) * 1000
    return password


# 174082 too high
text = open(sys.argv[-1], 'r').read()
parsed = parse(text)
p1 = 65368
print(p1:=part1(parsed))
print(p2:=part2(parsed))

assert (p1, p2) == (65368, 156166), (p1, p2)
