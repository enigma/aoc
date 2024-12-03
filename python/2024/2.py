import sys
from collections import Counter


def _one_way_safe(d, s=0):
    for i in range(len(d) - 1):
        if not 1 <= d[i] - d[i + 1] <= 3:
            return s and any(
                _one_way_safe(d[:j] + d[j + 1 :]) for j in (i - 1, i, i + 1)
            )
    return True


def _is_safe(row, part2=False):
    return _one_way_safe(row, part2) or _one_way_safe(row[::-1], part2)


def part1(data):
    return sum(_is_safe(row) for row in data)


def part2(data):
    return sum(_is_safe(row, True) for row in data)


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = [tuple(int(i) for i in line.split()) for line in full_input.splitlines()]
    print(f"{part1(data)=} {part2(data)=}")
