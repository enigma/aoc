import sys
from heapq import heappop, heappush
from functools import cache
from collections import deque
from typing import List
from math import inf
from collections import defaultdict


def parse(text):
    res = {}
    for row in text.strip().split("\n"):
        parts = row.split()
        v, rate, rest = parts[1], int(parts[4][5:-1]), "".join(parts[9:]).split(",")
        res[v] = (rate, rest)
    return res

def solve(parsed):
    encode = lambda x: 1 << (x + 0)
    coding = {}
    all_closed = 0
    for i, (flow, _) in parsed.items():
        id = coding[i] = len(coding)
        if flow:
            all_closed |= encode(id)
    flows = {}
    N = len(parsed)
    dists = [[N**3 for _ in range(N)] for _ in range(N)]
    for src, (flow, dsts) in parsed.items():
        src = coding[src]
        dsts = [coding[dst] for dst in dsts]
        flows[src] = flow
        dists[src][src] = 0
        for other in dsts:
            dists[src][other] = 1
    assert N == len(coding), (N, len(coding))
    cv = list(coding.values())
    for i in cv:
        for j in cv:
            for k in cv:
                dists[j][k] = min(dists[j][k], dists[j][i] + dists[i][k])
    start = coding['AA']
    
    @cache
    def neighbors(valve):
        return [(k, dists[valve][k] + 1, flow) for k, flow in flows.items() if flow]

    @cache
    def max_pressure(pos, min_left, closed,el=False):
        best = 0
        for v, dist, flow in neighbors(pos):
            if dist >= min_left: continue
            venc = encode(v)
            if not (closed & (venc)): continue
            time = min_left - dist
            pressure = time * flow + max_pressure(v, time, closed - (venc))
            if pressure > best:
                best = pressure
        return best

    @cache
    def max_pressure(pos, min_left, closed):
        best = 0
        for v, dist, flow in neighbors(pos):
            if dist > min_left: continue
            venc = encode(v)
            if not (closed & (venc)): continue
            time = min_left - dist
            pressure = time * flow + max_pressure(v, time, closed - (venc))
            if pressure > best:
                best = pressure
        return best

    def subsets(lst, lenght=None):
        if lenght is None:
            lenght = len(lst) // 2
        if len(lst) == lenght:
            yield sum(encode(k) for k in lst)
            return
        if len(lst) < lenght or not lenght: return
        item = encode(lst[0])
        for rest in subsets(lst[1:], lenght-1):
            yield rest | item
        yield from subsets(lst[1:], lenght)

    yield max_pressure(start, 30, all_closed)
    all_subsets = list(subsets(list(set(k for k,v in flows.items() if v))))
    best = 0
    for subset in all_subsets[:len(all_subsets)//2]:
        a = max_pressure(start, 26, all_closed - subset)
        b = max_pressure(start, 26, subset)
        if a + b > best:
            best = a + b
    yield best

itext = open(sys.argv[-1], "r").read()
text = """Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"""
parsed = parse(itext)
for part in solve(parsed):
    print(part)
