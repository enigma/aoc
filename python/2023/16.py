import sys


def parse(data):
    return [line for line in data.splitlines()]


DY = [0, 1, 0, -1]
DX = [1, 0, -1, 0]
DIRS = EAST, SOUTH, WEST, NORTH = list(zip(DY, DX))


def energy(grid, start, dd):
    h, w = len(grid), len(grid[0])
    seen = set()
    fringe = [(start, dd)]
    while fringe:
        e = (y, x), dir = fringe.pop()
        if e in seen:
            continue
        seen.add(e)
        ndirs = []
        match (grid[y][x], dir):
            case '.', _:
                ndirs.append(dir)
            case '|', (_, 0):
                ndirs.append(dir)
            case '|', (0, _):
                ndirs.extend([NORTH, SOUTH])
            case '-', (0, _):
                ndirs.append(dir)
            case '-', (_, 0):
                ndirs.extend([WEST, EAST])
            case '\\', (dy, dx):
                ndirs.append((dx, dy))
            case '/', (dy, dx):
                ndirs.append((-dx, -dy))
        for dy, dx in ndirs:
            ny = y + dy
            nx = x + dx
            if 0 <= ny < h and 0 <= nx < w:
                fringe.append(((ny, nx), (dy, dx)))
    return len(set(p for (p, _) in seen))


def solve(grid):
    p1 = energy(grid, (0, 0), EAST)

    p2 = 0
    h, w = len(grid), len(grid[0])
    for y in range(h):
        p2 = max(p2, energy(grid, (y, 0), EAST))
        p2 = max(p2, energy(grid, (y, w-1), WEST))
    for x in range(w):
        p2 = max(p2, energy(grid, (0, x), SOUTH))
        p2 = max(p2, energy(grid, (h-1, x), NORTH))
    return p1, p2


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
p1, p2 = solve(parsed)
print(p1)
print(p2)

assert p1 == 7434, p1
assert p2 == 8183, p2
