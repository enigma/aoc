import sys

from dataclasses import dataclass
from heapq import heappop, heapify

@dataclass
class Node:
    parent: tuple[int, int, int]
    size: int = 1

def parse(data):
    return [tuple(int(i) for i in line.split(",")) for line in data.splitlines()]


def dist(p1, p2):
    return sum((a - b) ** 2 for a, b in zip(p1, p2))


def find_set(x, forest):
    if forest[x].parent != x:
        res = forest[x].parent = find_set(forest[x].parent, forest)
        return res
    return x


def union_sets(x, y, forest):
    if (x := find_set(x, forest)) == (y := find_set(y, forest)):
        return
    if forest[x].size < forest[y].size:
        x, y = y, x
    forest[y].parent = x
    forest[x].size += forest[y].size


def solve(pos):
    dists, forest = [], {}
    for i, p1 in enumerate(pos):
        forest[p1] = Node(p1)  # make_set https://en.wikipedia.org/wiki/Disjoint-set_data_structure
        for p2 in pos[i + 1:]:
            dists.append((dist(p1, p2), p1, p2))
    heapify(dists)

    part1 = part2 = None
    n = 0
    while dists and (dd := heappop(dists)):
        _, p1, p2 = dd
        if n == 1000:
            roots = set(find_set(x, forest) for x in forest.keys())
            sizes = list(sorted(forest[r].size for r in roots))
            a, b, c = sizes[-3:]
            part1 = a * b * c
        if (u := find_set(p1, forest)) != (v := find_set(p2, forest)):
            union_sets(u, v, forest)
            part2 = p1[0] * p2[0]
            if forest[find_set(u, forest)].size == len(forest):
                break
        n += 1
    return part1, part2


if __name__ == "__main__":
    sdata = open(sys.argv[-1], "r").read()
    data = parse(sdata)
    p1, p2 = solve(data)
    print(f"{p1}")
    print(f"{p2}")
