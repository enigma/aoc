import sys

from collections import defaultdict
from heapq import heappop, heappush

DJ = [1, -1, 1j, -1j]
S = '><v^'
DELTA = {s: dj for s, dj in zip(S, DJ)}
REV_D = {v: k for k, v in DELTA.items()}

def wrap(s: complex, o: complex, low=1):
    if low < s.real <= o.real and low < s.imag < o.imag: return s
    x = low if s.real > o.real else (o.real if s.real < low else s.real)
    y = low if s.imag >= o.imag else (o.imag-1 if s.imag < low else s.imag)
    return x + y * 1j

def parse(text):
    grid = defaultdict(list)
    max_x = max_y = 0
    for y, row in enumerate(text.strip().split('\n')):
        for x, val in enumerate(row):
            if val == '#': continue
            max_x = max(max_x, x)
            max_y = max(max_y, y)
            if val == '.': continue
            grid[x + y*1j].append(DELTA[val])
    return grid, max_x + max_y * 1j

def advance(grid, mx):
    res = defaultdict(list)
    for pos, dirs in grid.items():
        for dir in dirs:
            res[wrap(pos + dir, mx)].append(dir)
    return res

def unroll(frames, mx):
    while True:
        frames.append(advance(frames[-1], mx))
        yield frames[-1]
        
def shortest_path(grid, mx, start, goal):
    frames = [grid]
    unroller = unroll(frames, mx)
    moves = [0] + list(DELTA.values())
    fringe = [(0, 0, start)]
    seen = set()
    while fringe:
        _, elapsed, pos = heappop(fringe)
        if pos == goal:
            return elapsed, frames[elapsed]
        k = elapsed, pos
        if k in seen:
            continue
        seen.add(k)
        while len(frames) < elapsed + 2: next(unroller)
        for move in moves:
            n: complex = pos + move
            if n.real == 0 or n.real == mx.real + 1: continue
            if n.imag > mx.imag or n.real > mx.real: continue
            if (n.imag == 0 and n.real != 1) or (n.imag == mx.imag and n.real != mx.real): continue
            if (elapsed + 1, n) in seen: continue
            if n in frames[elapsed+1] and frames[elapsed+1][n]: continue
            diff = goal - n
            heu = elapsed + abs(diff.real) + abs(diff.imag)
            heu = (heu, elapsed, n.real, n.imag)
            heappush(fringe, (heu, elapsed + 1, n))

def solve(data):
    grid, mx = data
    start = 1
    goal = mx
    pass1, grid1 = shortest_path(grid, mx, start, goal)
    yield pass1
    pass2, grid2 = shortest_path(grid1, mx, goal, start)
    pass3, _ = shortest_path(grid2, mx, start, goal)
    yield (pass1 + pass2 + pass3)

lines = open(sys.argv[-1]).read().strip()
#lines = '''#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#'''
parsed = list(parse(lines))
p1, p2 = list(solve(parsed))

print(p1)
print(p2)

assert (p1, p2) == (301, 859), (p1, p2)
