import sys


def parse(data):
    return [list(map(int, line.split(","))) for line in data.splitlines()]


def part1(tiles):
    best = 0
    for i, (xi, yi) in enumerate(tiles):
        for xj, yj in tiles[i + 1 :]:
            best = max(best, (abs(xi - xj) + 1) * (abs(yi - yj) + 1))
    return best


def part2(tiles):
    sorted_edges = []
    for i in range(L := len(tiles)):
        x1, y1 = tiles[i]
        x2, y2 = tiles[(i + 1) % L]
        sorted_edges.append((min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)))
    sorted_edges.sort(key=lambda x: (x[2] - x[0]) + (x[3] - x[1]), reverse=True)

    def intersects(min_x, min_y, max_x, max_y):
        for e_min_x, e_min_y, e_max_x, e_max_y in sorted_edges:
            if (
                min_x < e_max_x
                and max_x > e_min_x
                and min_y < e_max_y
                and max_y > e_min_y
            ):
                return True
        return False

    best = 0
    for i, (xi, yi) in enumerate(tiles):
        for xj, yj in tiles[i + 1 :]:
            min_x, max_x = min(xi, xj), max(xi, xj)
            min_y, max_y = min(yi, yj), max(yi, yj)

            area = (max_x - min_x + 1) * (max_y - min_y + 1)

            if area > best and not intersects(min_x, min_y, max_x, max_y):
                best = area
    return best


if __name__ == "__main__":
    sdata = open(sys.argv[-1], "r").read()
    data = parse(sdata)
    print(f"{part1(data)}")
    print(f"{part2(data)}")
