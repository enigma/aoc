import sys


def parse(data):
    return [list(line) for line in data.splitlines()]


def roll(grid, start, end, step):
    y, x = fy, fx = start
    dy, dx = step
    while (y, x) != end:
        if grid[y][x] == 'O':
            grid[y][x] = '.'
            grid[fy][fx] = 'O'
            fy += dy
            fx += dx
        elif grid[y][x] == '#':
            fy = y + dy
            fx = x + dx
        y += dy
        x += dx


def tilt(grid, d):
    H, W = len(grid), len(grid[0])
    total_o = sum(i == 'O' for line in grid for i in line)
    match d:
        case 'E':
            for y in range(H):
                roll(grid, (y, W-1), (y, -1), (0, -1))
        case 'W':
            for y in range(H):
                roll(grid, (y, 0), (y, W), (0, 1))
        case 'N':
            for x in range(W):
                roll(grid, (0, x), (H, x), (1, 0))
        case 'S':
            for x in range(W):
                roll(grid, (H-1, x), (-1, x), (-1, 0))
    atotal_o = sum(i == 'O' for line in grid for i in line)
    assert total_o == atotal_o, (total_o, atotal_o)
    return grid


def score(grid):
    s = 0
    for i, row in enumerate(grid[::-1], 1):
        s += sum(i for c in row if c == 'O')
    return s


def cycle(grid):
    for d in 'NWSE':
        tilt(grid, d)
    return grid


def part2(grid):
    seen = {}
    g = 1_000_000_000
    for i in range(g):
        k = tuple(i for row in grid for i in row)
        if found := seen.get(k, None):
            loop = found - i
            times = (g - i) % loop + 3
            for _ in range(times % 4):
                cycle(grid)
            break
        seen[k] = i
        cycle(grid)
    return grid


def solve(grid):
    cp = [i[:] for i in grid]
    p1 = score(tilt(cp, 'N'))
    p2 = score(part2(grid))
    return p1, p2


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
p1, p2 = solve(parsed)
print(p1)
print(p2)

assert p1 == 109654, p1
assert p2 == 94828, p2
