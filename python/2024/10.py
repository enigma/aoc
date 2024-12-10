from functools import cache
import sys


def parse(full_input):
    return tuple(tuple(int(i) for i in line) for line in full_input.splitlines())


DX = (0, 1, 0, -1)
DY = (1, 0, -1, 0)


@cache
def reachable(data, y, x):
    if data[y][x] == 9:
        return set([(y, x)])
    res = set()
    for dy, dx in zip(DY, DX):
        if 0 <= (ny := y + dy) < len(data) and 0 <= (nx := x + dx) < len(data[0]):
            if data[ny][nx] == data[y][x] + 1:
                res |= reachable(data, ny, nx)
    return res


def part1(data):
    res = 0
    for y, row in enumerate(data):
        for x, val in enumerate(row):
            if val == 0:
                res += len(reachable(data, y, x))
    return res


@cache
def paths(data, y, x):
    if data[y][x] == 9:
        return 1
    res = 0
    for dy, dx in zip(DY, DX):
        if 0 <= (ny := y + dy) < len(data) and 0 <= (nx := x + dx) < len(data[0]):
            if data[ny][nx] == data[y][x] + 1:
                res += paths(data, ny, nx)
    return res


def part2(data):
    res = 0
    for y, row in enumerate(data):
        for x, val in enumerate(row):
            if val == 0:
                res += paths(data, y, x)
    return res


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(part1(data))
    print(part2(data))
