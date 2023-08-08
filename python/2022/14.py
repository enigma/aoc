import sys
from collections import defaultdict

EMPTY = False
SAND = True

def parse(text):
    grid = defaultdict(lambda: EMPTY)
    max_y = 0
    for line in text.strip().split('\n'):
        waypoints = [tuple(map(int,xy.split(','))) for xy in line.split(' -> ')]
        for i, (fx, fy) in enumerate(waypoints[:-1]):
            tx, ty = waypoints[i+1]
            dx = (tx > fx) - (tx < fx)
            dy = (ty > fy) - (ty < fy)
            x, y = fx - dx, fy - dy
            while x != tx or y != ty:
                x += dx
                y += dy
                grid[x, y] = SAND
                max_y = max(y, max_y)
    return grid, max_y

from copy import deepcopy
def solve(grid, max_y, hard_bottom):
    grid = deepcopy(grid)
    if hard_bottom:
        max_y += 1
    dropped = 0
    source = (500, 0)
    dirs = ((0, 1), (-1, 1), (1, 1))[::-1]
    stack = [(source, list(dirs))]
    while stack:
        (x, y), attempt_left = stack.pop()
        if y > max_y:
            if hard_bottom:
                continue
            else:
                return dropped
        if grid[x, y]: continue
        if attempt_left:
            (dx, dy) = attempt_left.pop()
            stack.append(((x, y), attempt_left))
            stack.append(((x + dx, y + dy), list(dirs)))
        else:
            grid[x, y] = SAND
            dropped += 1
            if (x, y) == source:
                return dropped

def part1(grid, max_y):
    return solve(grid, max_y, False)

def part2(grid, max_y):
    return solve(grid, max_y, True)

text = open(sys.argv[-1], 'r').read()
parsed = parse(text)
print(part1(*parsed))
print(part2(*parsed))
