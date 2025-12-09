import sys


def parse(data):
    return [list(map(int, line.split(","))) for line in data.splitlines()]


def part1(tiles):
    return max(
        (abs(x1 - x2) + 1) * (abs(y1 - y2) + 1)
        for i, (x1, y1) in enumerate(tiles)
        for x2, y2 in tiles[i + 1 :]
    )


def part2(tiles):
    sorted_edges = []
    for i in range(L := len(tiles)):
        x1, y1 = tiles[i]
        x2, y2 = tiles[(i + 1) % L]
        sorted_edges.append((min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)))
    sorted_edges.sort(key=lambda x: (x[2] - x[0]) + (x[3] - x[1]), reverse=True)

    def intersects(bx1, by1, bx2, by2):
        for ex1, ey1, ex2, ey2 in sorted_edges:
            if bx1 < ex2 and bx2 > ex1 and by1 < ey2 and by2 > ey1:
                return True
        return False

    best = 0
    for i, (x1, y1) in enumerate(tiles):
        for x2, y2 in tiles[i + 1 :]:
            bx1, by1, bx2, by2 = min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)
            area = (bx2 - bx1 + 1) * (by2 - by1 + 1)
            if area > best and not intersects(bx1, by1, bx2, by2):
                best = area
    return best


if __name__ == "__main__":
    sdata = open(sys.argv[-1], "r").read()
    data = parse(sdata)
    print(f"{part1(data)}")
    print(f"{part2(data)}")
