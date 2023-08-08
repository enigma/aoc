import sys
from collections import deque

def parse(text):
    start, end = None, None
    result = []
    for y, row in enumerate(text.strip().split()):
        this = []
        for x, h in enumerate(row):
            match h:
                case 'S':
                    this.append(ord('a'))
                    start = (y, x)
                case 'E':
                    this.append(ord('z'))
                    end = (y, x)
                case _:
                    this.append(ord(h))
        result.append(this)
    return result, start, end

def neighbors(grid, pos):
    H, W = len(grid), len(grid[0])
    for dir in [(1, 0), (-1, 0), (0, -1), (0, 1)]:
        y, x = [i+j for (i, j) in zip(pos, dir)]
        if 0 <= y < H and 0 <= x < W:
            yield (y, x)

def distances(grid, end):
    distances = {end: 0}
    fringe = deque([end])
    seen = set()
    while fringe:
        (y, x) = cur = fringe.popleft()
        if cur in seen: continue
        seen.add(cur)
        for (ny, nx) in neighbors(grid, cur):
            if (ny, nx) in seen or grid[ny][nx] + 1 < grid[y][x]:
                continue
            distances[ny, nx] = distances[cur] + 1
            fringe.append((ny, nx))
    return distances    

text = open(sys.argv[-1], 'r').read().strip()
grid, start, end = parse(text)
dists = distances(grid, end)

part1 = dists[start]
part2 = min(d for (y, x), d in dists.items() if grid[y][x] == ord('a'))

print(part1)
print(part2)
