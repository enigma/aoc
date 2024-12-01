import sys
from collections import Counter


def part1(data):
    idx = lambda i: sorted(d[i] for d in data)
    return sum(abs(rhs - lhs) for lhs, rhs in zip(idx(0), idx(1)))


def part2(data):
    c = Counter(rhs for _, rhs in data)
    return sum(lhs * c[lhs] for lhs, _ in data)


if __name__ == "__main__":
    data = [
        tuple(int(i) for i in line.split())
        for line in open(sys.argv[-1], "r").read().splitlines()
    ]
    print(f"{part1(data)=} {part2(data)=}")
