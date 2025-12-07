import sys
from operator import mul, add
from functools import reduce

OPS = {"+": (add, 0), "*": (mul, 1)}


def parse(data):
    rows = data.splitlines()
    ops = [OPS[i.strip()] for i in rows.pop().strip().split()]
    cols = [[] for _ in ops]
    for row in rows:
        for col, el in zip(cols, row.split()):
            col.append(int(el))
    return ops, cols


def part1(data):
    return sum(reduce(op, col, zero) for (op, zero), col in zip(*parse(data)))


def part2(data):
    res = 0
    rows = data.splitlines()
    ops = {i: OPS[sop] for i, sop in enumerate(rows[-1]) if sop != " "}
    for oi, (op, zero) in ops.items():
        val = zero
        for ic in range(oi, len(rows[0])):
            if ic + 1 in ops:
                break
            if col := [int(row[ic]) for row in rows[:-1] if row[ic] != " "]:
                val = op(val, reduce(lambda acc, i: acc * 10 + i, col, 0))
        res += val
    return res


if __name__ == "__main__":
    data = open(sys.argv[-1], "r").read()
    print(f"{part1(data)}")
    print(f"{part2(data)}")
    p2 = part2(data)
    assert p2 == 8674740488592, p2
