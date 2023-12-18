import sys
from itertools import pairwise


DY = [0, 1, 0, -1]
DX = [1, 0, -1, 0]
DD = list(zip(DY, DX))
NN = "RDLU"
DIRS = {k: (dy, dx) for k, dy, dx in zip(NN, DY, DX)}


def parse(data):
    for line in data.splitlines():
        d, s, col = line.split()
        yield DIRS[d], int(s), col


def total_area(instrs):
    y, x = 0, 0
    vertex = [(y, x)]
    for (dy, dx), step, _ in instrs:
        y += dy * step
        x += dx * step
        vertex.append((y, x))
    shoelace = 0
    for (y1, x1), (y2, x2) in pairwise(vertex + vertex[:1]):
        shoelace += x1*y2 - x2*y1 + abs(x1-x2) + abs(y1-y2)
    return shoelace // 2 + 1


def solve(instrs):
    p1 = total_area(instrs)
    p2 = total_area((DD[int(c[-2])], int(c[2:-2], 16), 0)
                    for _, _, c in instrs)
    return p1, p2


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
parsed = list(parsed)
p1, p2 = solve(parsed)
print(p1)
print(p2)

assert p1 == 47527, p1
assert p2 == 52240187443190, p2
