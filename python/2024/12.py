import sys
from collections import Counter


def parse(full_input):
    return [line for line in full_input.strip().splitlines()]


def part1(data):
    H = len(data)
    W = len(data[0])
    seen = [[None] * W for _ in range(H)]
    region = 1
    for sy, row in enumerate(data):
        for sx, cell in enumerate(row):
            if seen[sy][sx]:
                continue
            fringe = [(sy, sx)]
            while fringe:
                y, x = fringe.pop()
                if seen[y][x]:
                    continue
                seen[y][x] = region
                for dy, dx in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                    if (
                        0 <= (ny := y + dy) < H
                        and 0 <= (nx := x + dx) < W
                        and data[ny][nx] == cell
                    ):
                        fringe.append((ny, nx))
            region += 1
    areas = Counter()
    perimeters = Counter()
    for y, row in enumerate(seen):
        for x, cell in enumerate(row):
            areas[cell] += 1
            p = 4
            if y and seen[y - 1][x] == cell:
                p -= 1
            if x and seen[y][x - 1] == cell:
                p -= 1
            if y < H - 1 and seen[y + 1][x] == cell:
                p -= 1
            if x < W - 1 and seen[y][x + 1] == cell:
                p -= 1
            perimeters[cell] += p
    return sum(area * perimeters[lot] for lot, area in areas.items())


DD = ((1, 0), (-1, 0), (0, 1), (0, -1))


def part2(data):
    H = len(data)
    W = len(data[0])
    regions = [[None] * W for _ in range(H)]
    region = 1
    areas = Counter()
    borders = set()
    for sy, row in enumerate(data):
        for sx, cell in enumerate(row):
            if regions[sy][sx]:
                continue
            fringe = [(sy, sx)]
            while fringe:
                y, x = fringe.pop()
                if regions[y][x]:
                    continue
                areas[region] += 1
                regions[y][x] = region
                for dy, dx in DD:
                    hit_border = False
                    if 0 <= (ny := y + dy) < H and 0 <= (nx := x + dx) < W:
                        if data[ny][nx] == cell:
                            fringe.append((ny, nx))
                        else:
                            hit_border = True
                    else:
                        hit_border = True
                    if hit_border:
                        borders.add(((y, x), (dy, dx), region))
            region += 1

    region2borders = Counter()
    for (y, x), (dy, dx), region in borders:
        match (dy, dx):
            case (0, 1) | (0, -1):
                if ((y - 1, x), (dy, dx), region) not in borders:
                    region2borders[region] += 1
            case (1, 0) | (-1, 0):
                if ((y, x - 1), (dy, dx), region) not in borders:
                    region2borders[region] += 1

    return sum(areas[k] * region2borders[k] for k in region2borders)


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(p1 := part1(data))
    print(p2 := part2(data))
