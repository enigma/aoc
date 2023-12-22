import sys
from collections import defaultdict
from collections import deque
from functools import cache


def parse(data):
    blocks = []
    for line in data.splitlines():
        start, end = line.split("~")
        start = [int(i) for i in start.split(',')]
        end = [int(i) for i in end.split(',')]
        blocks.append(list(zip(start, [i+1 for i in end])))
    blocks.sort(key=lambda a: a[-1][-1])
    return blocks


def solve(blocks):
    maxz_at = defaultdict(lambda: (0, 0))
    above = defaultdict(set)
    below = defaultdict(set)
    for i, (xr, yr, zr) in enumerate(blocks, 1):
        zmax = 0
        support = set()
        for x in range(*xr):
            for y in range(*yr):
                z, piece = maxz_at[x, y]
                if z == zmax:
                    support.add(piece)
                if z > zmax:
                    zmax = z
                    support = set([piece])
        above[i]  # sneaky set default!
        below[i] = support
        for j in support:
            above[j].add(i)
        
        for x in range(*xr):
            for y in range(*yr):
                assert zr[0] >= zmax, (zmax, zr)
                maxz_at[x, y] = zmax + zr[1]-zr[0],  i
        
    p1 = sum(all(len(below[t]) > 1 for t in top)
              for b, top in above.items() if b)
    
    def fall_from(start):
        fell = set([start])
        fringe = deque(above[start])
        while fringe:
            cur = fringe.popleft()
            if below[cur] <= fell:
                fell.add(cur)
                fringe.extend(above[cur])
        return len(fell) - 1
    # p2 = sum(map(fall_from, range(1, len(blocks) + 1)))

    @cache
    def sops(node):
        bn = set(below[node])
        assert 0 not in bn, bn
        if not bn:
            return bn
        if len(bn) == 1:
            return bn | sops(next(iter(bn)))
        base = set(sops(bn.pop()))
        while base and bn:
            base = base & sops(bn.pop())
        return base

    for i in above[0]:
        below[i].remove(0)
    p2 = sum(len(sops(i)) for i in range(1, len(blocks) + 1))
    return p1, p2


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
p1, p2 = solve(parsed)
print(p1)
print(p2)

assert p1 == 454, p1
assert p2 == 74287, p2
