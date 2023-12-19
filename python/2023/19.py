import sys
from math import prod
from typing import NamedTuple


class Part(NamedTuple):
    x: int
    m: int
    a: int
    s: int


def parse(data):
    parts = data.split("\n\n")
    rules = []
    for line in parts[0].splitlines():
        name, rs = line[:-1].split('{')
        rules.append((name, [(i.split(':') if ':' in i else ('True', i))
                             for i in rs.split(',')]))
    d = []
    for line in parts[1].splitlines():
        d.append(eval('Part(%s)' % line[1:-1]))
    return rules, d


def solve(rules, parts):
    rn = {n: r for n, r in rules}
    p1 = 0
    for part in parts:
        x, m, a, s = part
        rule = 'in'
        while rule != 'A' and rule != 'R':
            for cond, dst in rn[rule]:
                if eval(cond):
                    rule = dst
                    break
        if rule == 'A':
            p1 += sum(part)

    p2 = 0
    fringe = [('in', ((1, 4001),) * 4)]
    while fringe:
        cur, ranges = fringe.pop()
        if cur == 'A':
            p2 += prod(b-a for a, b in ranges)
            continue
        if cur == 'R':
            continue
        for cond, dst in rn[cur]:
            if cond == 'True':
                fringe.append((dst, ranges))
                continue
            pi = "xmas".find(cond[0])
            lo, hi = ranges[pi]
            v = int(cond[2:])
            ll = list(ranges)
            if cond[1] == '>':
                if v + 1 < hi:
                    middle = max(v+1, lo)
                    ll[pi] = (middle, hi)
                    fringe.append((dst, tuple(ll)))
                    ll[pi] = (lo, middle)
            elif cond[1] == '<':
                if lo < v:
                    middle = min(v, hi)
                    ll[pi] = (lo, middle)
                    fringe.append((dst, tuple(ll)))
                    ll[pi] = (middle, hi)
            ranges = tuple(ll)

    return p1, p2


data = open(sys.argv[-1]).read()
parsed = parse(data.strip())
p1, p2 = solve(*parsed)
print(p1)
print(p2)

assert p1 == 395382, p1
assert p2 == 103557657654583, p2
