import sys
from collections import Counter


def parse(full_input):
    res = []
    for line in full_input.strip().splitlines():
        p, v = [tuple(map(int, s[2:].split(","))) for s in line.split()]
        res.append((p, v))
    return res


def predict(ps, vs, times=100, H=103, W=101):
    x, y = ps
    vx, vy = vs
    x, y = x + vx * times, y + vy * times
    return x % W, y % H


def solve(data, times=100, H=103, W=101, threshold=10):
    q = Counter()
    landed = set()
    for p, v in data:
        x, y = predict(p, v, times, H, W)
        landed.add((x, y))
        if x > threshold and all((xi, y) in landed for xi in range(x - threshold, x)):
            return times, True
        if x == W // 2 or y == H // 2:
            continue
        q[x < W // 2, y < H // 2] += 1
    res = 1
    for v in q.values():
        res *= v
    return res, False


def part1(data):
    return solve(data, 100)[0]


def part2(data):
    for i in range(100, 10000):
        res, found = solve(data, i)
        if found:
            return res


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(p1 := part1(data))
    print(p2 := part2(data))
