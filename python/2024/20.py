import sys
from scipy.spatial import KDTree


def parse(data):
    res = []
    start = end = None
    for y, line in enumerate(data.strip().splitlines()):
        row = []
        for x, c in enumerate(line):
            match c:
                case "#" | ".":
                    row.append(c)
                case "S":
                    row.append(".")
                    start = (y, x)
                case "E":
                    row.append(".")
                    end = (y, x)
                case _:
                    raise ValueError(f"Invalid character: {c} at {x=}, {y=}")
        res.append(row)
    return res, start, end


def honest_path(grid, start, end):
    res = [start]
    prev, cur = None, start
    while cur != end:
        for dy, dx in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            if (
                0 <= (ny := cur[0] + dy) < len(grid)
                and 0 <= (nx := cur[1] + dx) < len(grid[0])
                and (ny, nx) != prev
            ):
                if grid[ny][nx] == ".":
                    prev, cur = cur, (ny, nx)
                    res.append(cur)
                    break
    assert res[0] == start, (res[0], start)
    assert res[-1] == end, (res[-1], end)
    return res


def solve(data):
    grid, start, end = data
    path = honest_path(grid, start, end)
    tree = KDTree(path)
    parts = [0, 0]
    for part, threshold in enumerate([2, 20]):
        for i, (y1, x1) in enumerate(path):
            for j in tree.query_ball_point((y1, x1), threshold, p=1):
                if j < i:
                    continue
                y2, x2 = path[j]
                dist = abs(y1 - y2) + abs(x1 - x2)
                if j - i - dist >= 100:
                    parts[part] += 1
    return parts


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    p1, p2 = solve(data)
    print(p1)
    print(p2)
