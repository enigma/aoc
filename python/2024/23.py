from dataclasses import dataclass
from itertools import combinations
import sys


@dataclass
class Node:
    name: str
    neigh: set["Node"]

    def __hash__(self):
        return hash(self.name)


def parse(s):
    nodes = {}
    for line in s.strip().splitlines():
        lhs, rhs = [nodes.setdefault(n, Node(n, set())) for n in line.split("-")]
        lhs.neigh.add(rhs)
        rhs.neigh.add(lhs)
    return set(nodes.values())


def bron_kerbosch(R, P, X):
    if not P and not X:
        yield R
        return
    for v in P - next(iter(P | X)).neigh:
        yield from bron_kerbosch(R | {v}, P & v.neigh, X & v.neigh)
        P = P - {v}
        X = X | {v}


def part1(nodes):
    return len(
        {
            frozenset(triplet)
            for clique in bron_kerbosch(set(), set(nodes), set())
            for triplet in combinations(clique, 3)
            if any(t.name[0] == "t" for t in triplet)
        }
    )


def part2(nodes):
    res = max(iter(bron_kerbosch(set(), set(nodes), set())), key=len)
    return ",".join(sorted(n.name for n in res))


if __name__ == "__main__":
    full_input = open(sys.argv[-1], "r").read()
    data = parse(full_input)
    print(p1 := part1(data))
    print(p2 := part2(data))
