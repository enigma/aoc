import sys
from itertools import combinations
from collections import defaultdict


def parse(full_input):
    res = defaultdict(set)
    width = 0
    height = 0
    for y, row in enumerate(full_input.splitlines()):
        height += 1
        for x, c in enumerate(row):
            if c != ".":
                res[c].add((y, x))
            width = max(width, x + 1)
    return res, height, width


def part1(data):
    antennas, H, W = data
    anti = set()
    for _symbol, positions in antennas.items():
        for (ay, ax), (by, bx) in combinations(positions, 2):
            dx = ax - bx
            dy = ay - by
            for f in [1, -2]:
                if 0 <= (ny := ay + f * dy) < H and 0 <= (nx := ax + f * dx) < W:
                    anti.add((nx, ny))
    return len(anti)


def part2(data):
    antennas, H, W = data
    anti = set()
    for _symbol, positions in antennas.items():
        for (ay, ax), (by, bx) in combinations(positions, 2):
            dx = ax - bx
            dy = ay - by
            if dy < 0:
                dy, dx = -dy, -dx
            sy, sx = ay, ax
            while sy - dy >= 0:
                sy -= dy
                sx -= dx
            for f in range(0, max(H, W)):
                if 0 <= (ny := sy + f * dy) < H and 0 <= (nx := sx + f * dx) < W:
                    anti.add((nx, ny))
    return len(anti)


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    p1 = part1(data)
    p2 = part2(data)
    print(f"part1 = {p1}\npart2 = {p2}")
