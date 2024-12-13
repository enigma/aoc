import sys
from functools import cache
from math import log10, floor


@cache
def splitting(datum):
    if datum == 0:
        return (1,)
    digits = floor(log10(datum)) + 1
    if digits % 2 == 0:
        lhs = datum // 10 ** (digits // 2)
        rhs = datum % 10 ** (digits // 2)
        return (lhs, rhs)
    return (datum * 2024,)


@cache
def blink_node(datum, n):
    if n == 0:
        return 1
    return blink(splitting(datum), n - 1)


def blink(data, n):
    return sum(blink_node(datum, n) for datum in data)


if __name__ == "__main__":
    data = tuple(map(int, open(sys.argv[-1], "r").read().strip().split()))
    print(p1 := blink(data, 25))
    print(p2 := blink(data, 75))
