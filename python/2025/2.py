import sys
from math import log10

from bisect import bisect_left

SIZES = (2, 3, 4, 5, 6, 7)


def parse(data):
    return [[int(x) for x in row.split("-")] for row in data.split(",")]


def gen_invalid(cap=10**10, sizes=SIZES):
    res, seen = [], set()
    for size in sizes:
        for i in range(1, int(10 ** (log10(cap) / size))):
            if (candidate := int(str(i) * size)) not in seen:
                seen.add(candidate)
                res.append((candidate, size))
    return list(sorted(res))


all_invalid = gen_invalid()


def solve(data, sizes):
    total = 0
    for a, b in data:
        ap = bisect_left(all_invalid, (a, sizes[0]))
        bp = bisect_left(all_invalid, (b, sizes[-1]))
        for invalid, size in all_invalid[ap:bp]:
            if invalid >= a and size in sizes:
                total += invalid
    return total


def part1(data):
    return solve(data, (2,))


def part2(data):
    return solve(data, SIZES)


if __name__ == "__main__":
    sdata = open(sys.argv[-1], "r").read()
    data = parse(sdata)
    p1 = part1(data)
    print(f"{p1=}")

    p2 = part2(data)
    print(f"{p2=}")

    # assert p1 == 23039913998, p1
    # assert p2 == 35950619148, p2
