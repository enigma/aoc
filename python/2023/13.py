import sys


def parse(data):
    for pz in data.split("\n\n"):
        yield [[i == '#' for i in line] for line in pz.splitlines()]


def score(pz, part2=False):
    H, W = len(pz), len(pz[0])
    threshold = 1 * part2
    for x in range(1, W):
        eq = sum(pz[y][xa] == pz[y][xb]
                 for y in range(H)
                 for (xa, xb) in zip(reversed(range(x)), range(x, W)))
        if abs(eq - (H * min(x, W - x))) == threshold:
            return x
    for y in range(1, H):
        eq = sum(pz[ya][x] == pz[yb][x]
                 for x in range(W)
                 for (ya, yb) in zip(reversed(range(y)), range(y, H)))
        if abs(eq - (W * min(y, H - y))) == threshold:
            return y * 100
    return 0


def solve(records):
    p1 = p2 = 0
    for pz in records:
        p1 += score(pz)
        p2 += score(pz, True)
    return p1, p2


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
parsed = list(parsed)
p1, p2 = solve(parsed)
print(p1)
print(p2)

assert p1 == 33520, p1
assert p2 == 34824, p2
