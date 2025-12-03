import sys

from functools import cache


def parse(data):
    return [tuple(int(x) for x in row) for row in data.splitlines()]


def max_jolt(bank, size):
    @cache
    def _max_jolt(start, size):
        result = cur = None
        for i in range(start, len(bank) - size + 1):
            if cur is None or bank[i] > cur:
                if size == 1:
                    result = cur = bank[i]
                elif (candidate := _max_jolt(i + 1, size - 1)) is not None:
                    cur = bank[i]
                    result = cur * (10 ** (size - 1)) + candidate
        return result

    return _max_jolt(0, size)


def solve(banks, size):
    return sum(max_jolt(i, size) for i in banks)


def part1(data):
    return solve(data, 2)


def part2(data):
    return solve(data, 12)


if __name__ == "__main__":
    sdata = open(sys.argv[-1], "r").read()
    data = parse(sdata)
    p1 = part1(data)
    print(f"{p1=}")

    p2 = part2(data)
    print(f"{p2=}")
